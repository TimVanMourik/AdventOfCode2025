use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};

use crate::Machine;

pub struct MachineBinary {
    pub(crate) lights: usize,
    pub(crate) buttons: Vec<usize>,
}

fn convert_machine(machine: &Machine) -> MachineBinary {
    // consider lights as a binary number, where each light is a bit
    let lights = machine
        .lights
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, light)| match light {
            crate::types::Light::On => acc | (1 << i),
            crate::types::Light::Off => acc,
        });

    let width = machine.lights.len();
    let buttons = machine
        .buttons
        .iter()
        .map(|wiring| {
            wiring.iter().fold(0, |acc, &button| {
                acc + (1 << width.saturating_sub(button + 1))
            })
        })
        .collect();

    MachineBinary { lights, buttons }
}

fn compute_lights_state(buttons: &[usize]) -> usize {
    buttons.iter().fold(0, |acc, &button| acc ^ button)
}

fn compute_minimal_presses(machine: MachineBinary) -> Result<i64> {
    let target = machine.lights;
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();

    dist.insert(0usize, 0_i64);
    queue.push_back(0usize);

    while let Some(state) = queue.pop_front() {
        let steps = *dist.get(&state).unwrap();
        if state == target {
            return Ok(steps);
        }

        for &button in &machine.buttons {
            let next = compute_lights_state(&[state, button]);
            if !dist.contains_key(&next) {
                dist.insert(next, steps + 1);
                queue.push_back(next);
            }
        }
    }

    Err(anyhow!(
        "No sequence of button presses reaches the target lights state {target:b}"
    ))
}

#[cfg(test)]
mod tests {
    use super::{compute_lights_state, compute_minimal_presses, MachineBinary};

    #[test]
    fn computes_lights_state_from_button_permutation() {
        let buttons = vec![1, 2, 3, 4];
        assert_eq!(compute_lights_state(&buttons), 4);
    }

    #[test]
    fn computes_minimal_button_presses_for_target_state() {
        let machine = MachineBinary {
            lights: 5,
            buttons: vec![1, 2, 3, 4],
        };

        assert_eq!(compute_minimal_presses(machine).unwrap(), 2);
    }

    #[test]
    fn rejects_unreachable_target_state() {
        let machine = MachineBinary {
            lights: 8,
            buttons: vec![1, 2, 4],
        };

        assert!(compute_minimal_presses(machine).is_err());
    }
}

pub fn part1(items: &[Machine]) -> i64 {
    items
        .iter()
        .map(convert_machine)
        .map(|machine| compute_minimal_presses(machine).unwrap())
        .sum()
}
