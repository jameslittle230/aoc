use itertools::Itertools;

use crate::Part;

pub(crate) fn exec(part: &Part) -> u32 {
    let contents = include_str!("../inputs/1.txt");

    let mut elf_inventories: Vec<u32> = contents
        .split("\n\n")
        .into_iter()
        .map(|elf_inventory| -> u32 {
            elf_inventory
                .split('\n')
                .into_iter()
                .map(|food_cal_value| food_cal_value.parse::<u32>().expect(""))
                .sum()
        })
        .collect_vec();

    match part {
        Part::One => elf_inventories.into_iter().max().expect(""),
        Part::Two => {
            elf_inventories.sort();
            elf_inventories.reverse();
            elf_inventories.into_iter().take(3).sum()
        }
    }
}
