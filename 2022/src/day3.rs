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

pub(crate) fn exec(_part: &Part) -> u32 {
    let contents = include_str!("../inputs/3.txt");

    contents
        .split('\n')
        .into_iter()
        .map(|rucksack_contents_str| -> u32 {
            let rucksack_contents = rucksack_contents_str.chars().collect_vec();

            let (a, b) = rucksack_contents.split_at(rucksack_contents.len() / 2);
            let mut compt_a: HashSet<&char> = HashSet::new();
            let mut compt_b: HashSet<&char> = HashSet::new();

            for item in a {
                compt_a.insert(item);
            }
            for item in b {
                compt_b.insert(item);
            }

            let intersection = compt_a.intersection(&compt_b).collect_vec();
            intersection.iter().map(|item| get_item_score(item)).sum()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::get_item_score;

    #[test]
    fn it_works() {
        assert_eq!(get_item_score(&'a'), 1);
        assert_eq!(get_item_score(&'z'), 26);
        assert_eq!(get_item_score(&'A'), 27);
        assert_eq!(get_item_score(&'Z'), 52);
    }
}
