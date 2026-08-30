use std::ops::Index;

use anyhow::{anyhow, Result};

use crate::Machine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ButtonIndex(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CounterIndex(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PressCount(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Joltage(i64);

/// `contributions[counter][button]` is `true` when pressing `button`
/// increments joltage counter `counter` by one.
#[derive(Debug)]
struct WiringMatrix(Vec<Vec<bool>>);

impl WiringMatrix {
    fn num_buttons(&self) -> usize {
        self.0.first().map_or(0, Vec::len)
    }

    fn num_counters(&self) -> usize {
        self.0.len()
    }

    fn counters(&self) -> impl Iterator<Item = CounterIndex> {
        (0..self.num_counters()).map(CounterIndex)
    }

    fn buttons(&self) -> impl Iterator<Item = ButtonIndex> {
        (0..self.num_buttons()).map(ButtonIndex)
    }

    fn contributes(&self, counter: CounterIndex, button: ButtonIndex) -> bool {
        self.0[counter.0][button.0]
    }
}

#[derive(Debug)]
struct JoltageTargets(Vec<Joltage>);

impl Index<CounterIndex> for JoltageTargets {
    type Output = Joltage;

    fn index(&self, counter: CounterIndex) -> &Joltage {
        &self.0[counter.0]
    }
}

#[derive(Debug)]
struct ButtonPressSystem {
    contributions: WiringMatrix,
    target_joltage: JoltageTargets,
}

fn convert_machine(machine: &Machine) -> ButtonPressSystem {
    let num_counters = machine.joltage.len();
    let num_buttons = machine.buttons.len();
    let mut contributions = vec![vec![false; num_buttons]; num_counters];

    for (button_index, wiring) in machine.buttons.iter().enumerate() {
        for &counter_index in wiring {
            contributions[counter_index][button_index] = true;
        }
    }

    let target_joltage = machine
        .joltage
        .iter()
        .map(|&joltage| Joltage(joltage as i64))
        .collect();

    ButtonPressSystem {
        contributions: WiringMatrix(contributions),
        target_joltage: JoltageTargets(target_joltage),
    }
}

/// One row of the linear system: `coefficients[button] * presses(button)`,
/// summed over every button, must equal `target`.
#[derive(Debug, Clone)]
struct Equation {
    coefficients: Vec<f64>,
    target: f64,
}

impl Equation {
    fn coefficient(&self, button: ButtonIndex) -> f64 {
        self.coefficients[button.0]
    }

    fn is_trivially_zero(&self) -> bool {
        self.coefficients.iter().all(|&coefficient| coefficient.abs() < 1e-6)
    }

    fn is_unsatisfiable(&self) -> bool {
        self.is_trivially_zero() && self.target.abs() > 1e-6
    }
}

#[derive(Debug)]
struct EquationSystem(Vec<Equation>);

impl EquationSystem {
    fn from_system(system: &ButtonPressSystem) -> Self {
        let equations = system
            .contributions
            .counters()
            .map(|counter| Equation {
                coefficients: system
                    .contributions
                    .buttons()
                    .map(|button| if system.contributions.contributes(counter, button) {
                        1.0
                    } else {
                        0.0
                    })
                    .collect(),
                target: system.target_joltage[counter].0 as f64,
            })
            .collect();

        Self(equations)
    }

    fn has_no_solution(&self) -> bool {
        self.0.iter().any(Equation::is_unsatisfiable)
    }
}

/// Reduces the system to reduced row echelon form in place, returning the
/// button each successive row was pivoted on.
fn reduce_to_echelon_form(equations: &mut EquationSystem, num_buttons: usize) -> Vec<ButtonIndex> {
    let num_equations = equations.0.len();
    let mut pivot_buttons = Vec::new();
    let mut pivot_row = 0;

    for button in (0..num_buttons).map(ButtonIndex) {
        if pivot_row >= num_equations {
            break;
        }

        let best_row = (pivot_row..num_equations)
            .filter(|&row| equations.0[row].coefficient(button).abs() > 1e-9)
            .max_by(|&a, &b| {
                equations.0[a]
                    .coefficient(button)
                    .abs()
                    .total_cmp(&equations.0[b].coefficient(button).abs())
            });

        let Some(best_row) = best_row else { continue };
        equations.0.swap(pivot_row, best_row);

        let pivot_value = equations.0[pivot_row].coefficient(button);
        for coefficient in &mut equations.0[pivot_row].coefficients {
            *coefficient /= pivot_value;
        }
        equations.0[pivot_row].target /= pivot_value;

        for row in 0..num_equations {
            if row == pivot_row {
                continue;
            }
            let factor = equations.0[row].coefficient(button);
            if factor.abs() > 1e-12 {
                let pivot_equation = equations.0[pivot_row].clone();
                let equation = &mut equations.0[row];
                for (coefficient, pivot_coefficient) in
                    equation.coefficients.iter_mut().zip(&pivot_equation.coefficients)
                {
                    *coefficient -= factor * pivot_coefficient;
                }
                equation.target -= factor * pivot_equation.target;
            }
        }

        pivot_buttons.push(button);
        pivot_row += 1;
    }

    pivot_buttons
}

/// A button can never be pressed more times than the smallest joltage target
/// among the counters it feeds, since every other button's contribution to
/// that counter is non-negative.
fn upper_bound_on_presses(system: &ButtonPressSystem, button: ButtonIndex) -> PressCount {
    let bound = system
        .contributions
        .counters()
        .filter(|&counter| system.contributions.contributes(counter, button))
        .map(|counter| system.target_joltage[counter].0)
        .min()
        .unwrap_or(0);

    PressCount(bound)
}

#[derive(Debug)]
struct PressCounts(Vec<PressCount>);

impl PressCounts {
    fn total(&self) -> i64 {
        self.0.iter().map(|count| count.0).sum()
    }
}

/// Given chosen press counts for the free buttons, solves for the remaining
/// (pivoted) buttons via the reduced system and checks that the full result
/// is an exact, non-negative integer solution of the original equations.
fn press_counts_for(
    system: &ButtonPressSystem,
    reduced_equations: &EquationSystem,
    pivot_buttons: &[ButtonIndex],
    free_buttons: &[ButtonIndex],
    free_press_counts: &[PressCount],
) -> Option<PressCounts> {
    let num_buttons = system.contributions.num_buttons();
    let mut presses = vec![0f64; num_buttons];
    for (&button, &count) in free_buttons.iter().zip(free_press_counts) {
        presses[button.0] = count.0 as f64;
    }

    for (row, &button) in pivot_buttons.iter().enumerate() {
        let equation = &reduced_equations.0[row];
        let mut value = equation.target;
        for (&free_button, &free_count) in free_buttons.iter().zip(free_press_counts) {
            value -= equation.coefficient(free_button) * free_count.0 as f64;
        }
        presses[button.0] = value;
    }

    let rounded: Vec<i64> = presses.iter().map(|value| value.round() as i64).collect();
    let is_integral = presses
        .iter()
        .zip(&rounded)
        .all(|(exact, whole)| (exact - *whole as f64).abs() < 1e-6);
    let is_non_negative = rounded.iter().all(|&count| count >= 0);

    let reaches_target = system.contributions.counters().all(|counter| {
        let joltage: i64 = system
            .contributions
            .buttons()
            .filter(|&button| system.contributions.contributes(counter, button))
            .map(|button| rounded[button.0])
            .sum();
        joltage == system.target_joltage[counter].0
    });

    (is_integral && is_non_negative && reaches_target)
        .then(|| PressCounts(rounded.into_iter().map(PressCount).collect()))
}

/// Tries every combination of press counts for the free buttons (each
/// bounded by `upper_bound_on_presses`) and returns the smallest total
/// across every valid, complete solution.
fn find_minimal_total_presses(
    system: &ButtonPressSystem,
    reduced_equations: &EquationSystem,
    pivot_buttons: &[ButtonIndex],
    free_buttons: &[ButtonIndex],
    free_button_bounds: &[PressCount],
) -> Option<i64> {
    fn search_free_button_assignments(
        depth: usize,
        free_press_counts: &mut Vec<PressCount>,
        system: &ButtonPressSystem,
        reduced_equations: &EquationSystem,
        pivot_buttons: &[ButtonIndex],
        free_buttons: &[ButtonIndex],
        free_button_bounds: &[PressCount],
        best_total: &mut Option<i64>,
    ) {
        if depth == free_button_bounds.len() {
            if let Some(press_counts) =
                press_counts_for(system, reduced_equations, pivot_buttons, free_buttons, free_press_counts)
            {
                let total = press_counts.total();
                if best_total.is_none_or(|best| total < best) {
                    *best_total = Some(total);
                }
            }
            return;
        }

        for presses in 0..=free_button_bounds[depth].0 {
            free_press_counts[depth] = PressCount(presses);
            search_free_button_assignments(
                depth + 1,
                free_press_counts,
                system,
                reduced_equations,
                pivot_buttons,
                free_buttons,
                free_button_bounds,
                best_total,
            );
        }
    }

    let mut best_total = None;
    let mut free_press_counts = vec![PressCount(0); free_button_bounds.len()];
    search_free_button_assignments(
        0,
        &mut free_press_counts,
        system,
        reduced_equations,
        pivot_buttons,
        free_buttons,
        free_button_bounds,
        &mut best_total,
    );
    best_total
}

fn compute_minimal_presses(system: &ButtonPressSystem) -> Result<i64> {
    let num_buttons = system.contributions.num_buttons();

    let mut equations = EquationSystem::from_system(system);
    let pivot_buttons = reduce_to_echelon_form(&mut equations, num_buttons);

    if equations.has_no_solution() {
        return Err(anyhow!("machine has no way to reach the target joltage"));
    }

    let free_buttons: Vec<ButtonIndex> = system
        .contributions
        .buttons()
        .filter(|button| !pivot_buttons.contains(button))
        .collect();
    let free_button_bounds: Vec<PressCount> = free_buttons
        .iter()
        .map(|&button| upper_bound_on_presses(system, button))
        .collect();

    find_minimal_total_presses(system, &equations, &pivot_buttons, &free_buttons, &free_button_bounds)
        .ok_or_else(|| anyhow!("no combination of button presses reaches the target joltage"))
}

pub fn part2(items: &[Machine]) -> i64 {
    items
        .iter()
        .map(|machine| {
            let system = convert_machine(machine);
            compute_minimal_presses(&system).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{compute_minimal_presses, ButtonPressSystem, Joltage, JoltageTargets, WiringMatrix};

    #[test]
    fn computes_minimal_presses_for_underdetermined_system() {
        // buttons: (3) (1,3) (2) (2,3) (0,2) (0,1), target {3,5,4,7}
        let contributions = vec![
            vec![false, false, false, false, true, true],
            vec![false, true, false, false, false, true],
            vec![false, false, true, true, true, false],
            vec![true, true, false, true, false, false],
        ];
        let target_joltage = vec![3, 5, 4, 7].into_iter().map(Joltage).collect();
        let system = ButtonPressSystem {
            contributions: WiringMatrix(contributions),
            target_joltage: JoltageTargets(target_joltage),
        };

        assert_eq!(compute_minimal_presses(&system).unwrap(), 10);
    }
}
