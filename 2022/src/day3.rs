use std::collections::HashSet;

use itertools::Itertools;

use crate::Part;

fn get_item_score(item: &char) -> u32 {
    if item.is_ascii_lowercase() {
        return item.to_owned() as u8 as u32 - 96;
    } else if item.is_ascii_uppercase() {
        return item.to_owned() as u8 as u32 - 38;
    }
    panic!()
}

pub(crate) fn exec(part: &Part) -> u32 {
    let contents = include_str!("../inputs/3.txt");
    let lines = contents.split('\n');

    match part {
        Part::One => lines
            .map(|rucksack_contents_str| -> u32 {
                let rucksack_contents = rucksack_contents_str.chars().collect_vec();

                let (a, b) = rucksack_contents.split_at(rucksack_contents.len() / 2);
                let compt_a: HashSet<char> = HashSet::from_iter(a.iter().cloned());
                let compt_b: HashSet<char> = HashSet::from_iter(b.iter().cloned());

                let intersection = compt_a.intersection(&compt_b).collect_vec();
                intersection.iter().map(|item| get_item_score(item)).sum()
            })
            .sum(),

        Part::Two => lines
            .collect_vec()
            .chunks(3)
            .map(|elf_group| -> u32 {
                let a: HashSet<char> = HashSet::from_iter(elf_group[0].chars());
                let b: HashSet<char> = HashSet::from_iter(elf_group[1].chars());
                let c: HashSet<char> = HashSet::from_iter(elf_group[2].chars());

                HashSet::from_iter(a.intersection(&b).cloned())
                    .intersection(&c)
                    .map(get_item_score)
                    .sum()
            })
            .sum(),
    }
}

#[cfg(test)]
mod tests {

    use super::{exec, get_item_score};
    use crate::Part;

    #[test]
    fn get_item_score_test() {
        assert_eq!(get_item_score(&'a'), 1);
        assert_eq!(get_item_score(&'z'), 26);
        assert_eq!(get_item_score(&'A'), 27);
        assert_eq!(get_item_score(&'Z'), 52);
    }

    #[test]
    fn it_works() {
        assert_eq!(exec(&Part::One), 7746);
        assert_eq!(exec(&Part::Two), 2604);
    }
}
