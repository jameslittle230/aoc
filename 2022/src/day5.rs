use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use anyhow::anyhow;

use crate::Part;

#[derive(Debug)]
struct Instruction {
    crate_quantity: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }

        let captures = RE.captures(s).ok_or_else(|| anyhow!("asdf"))?;

        Ok(Self {
            crate_quantity: captures[1].parse()?,
            from_stack: captures[2].parse()?,
            to_stack: captures[3].parse()?,
        })
    }
}

pub(crate) fn exec(part: &Part) -> String {
    let contents = include_str!("../inputs/5.txt");

    let (structure, instructions_s) = contents
        .split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    structure
        .lines()
        .map(|row| row.chars().skip(1).step_by(4).collect_vec())
        .rev()
        .skip(1)
        .for_each(|row| {
            row.iter().enumerate().for_each(|(a, b)| {
                stacks
                    .entry(a + 1)
                    .and_modify(|stack| match b {
                        ' ' => {}
                        _ => stack.push(b.to_owned()),
                    })
                    .or_insert_with(|| vec![b.to_owned()]);
            })
        });

    let instructions = instructions_s
        .lines()
        .filter_map(|line| Instruction::from_str(line).ok())
        .collect_vec();

    assert!(instructions.len() == instructions_s.lines().collect_vec().len());

    match part {
        Part::One => {
            for instruction in instructions {
                for _ in 0..instruction.crate_quantity {
                    let from = stacks.get_mut(&instruction.from_stack).unwrap();
                    let krate = from.pop().unwrap();
                    let to = stacks.get_mut(&instruction.to_stack).unwrap();
                    to.push(krate);
                }
            }
        }
        Part::Two => {
            for instruction in instructions {
                let from = stacks.get_mut(&instruction.from_stack).unwrap();
                let split_point = from.len() - instruction.crate_quantity;
                let mut picked_up = from.split_off(split_point);
                let to = stacks.get_mut(&instruction.to_stack).unwrap();
                to.append(&mut picked_up);
            }
        }
    }

    (1..=9)
        .map(|stack_idx| stacks.get(&stack_idx).unwrap().last().unwrap())
        .join("")
}

#[cfg(test)]
mod tests {
    use super::exec;
    use crate::Part;

    #[test]
    fn it_works() {
        assert_eq!(exec(&Part::One), "LJSVLTWQM");
        assert_eq!(exec(&Part::Two), "BRQWDBBJM");
    }
}
