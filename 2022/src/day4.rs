use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::Part;

#[derive(PartialEq, Eq)]
enum OverlapStatus {
    FullyContains,
    PartiallyContains,
    NoOverlap,
}

fn determine_overlaps<T>(r1: RangeInclusive<T>, r2: RangeInclusive<T>) -> OverlapStatus
where
    T: Ord,
{
    if r1.contains(r2.start()) && r1.contains(r2.end()) {
        return OverlapStatus::FullyContains;
    }
    if r2.contains(r1.start()) && r2.contains(r1.end()) {
        return OverlapStatus::FullyContains;
    }
    if r1.contains(r2.start()) || r1.contains(r2.end()) {
        return OverlapStatus::PartiallyContains;
    }
    if r2.contains(r1.start()) || r2.contains(r1.end()) {
        return OverlapStatus::PartiallyContains;
    }
    OverlapStatus::NoOverlap
}

pub(crate) fn exec(part: &Part) -> u32 {
    let contents = include_str!("../inputs/4.txt");

    let ranges = contents
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .unwrap()
                })
                .collect_tuple::<((u32, u32), (u32, u32))>()
                .unwrap()
        })
        .map(|((a, b), (c, d))| (RangeInclusive::new(a, b), RangeInclusive::new(c, d)))
        .map(|(r1, r2)| determine_overlaps(r1, r2));

    let count = match part {
        Part::One => ranges
            .filter(|overlap_status| overlap_status == &OverlapStatus::FullyContains)
            .count(),
        Part::Two => ranges
            .filter(|overlap_status| overlap_status != &OverlapStatus::NoOverlap)
            .count(),
    };

    count as u32
}

#[cfg(test)]
mod tests {
    use crate::{day4::exec, Part};

    #[test]
    fn it_works() {
        assert_eq!(exec(&Part::One), 424);
        assert_eq!(exec(&Part::Two), 804);
    }
}
