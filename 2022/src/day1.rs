use itertools::Itertools;

use crate::Part;

pub(crate) fn exec(part: &Part) -> u32 {
    let contents = include_str!("../inputs/1.txt");

    contents
        .split("\n\n")
        .into_iter()
        .map(|elf_inventory| -> u32 {
            elf_inventory
                .split('\n')
                .into_iter()
                .map(|food_cal_value| food_cal_value.parse::<u32>().expect(""))
                .sum()
        })
        .sorted()
        .rev()
        .take(match part {
            Part::One => 1,
            Part::Two => 3,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::exec;
    use crate::Part;

    #[test]
    fn it_works() {
        assert_eq!(exec(&Part::One), 66186);
        assert_eq!(exec(&Part::Two), 196804);
    }
}
