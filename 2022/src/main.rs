use std::{fmt::Display, time::Instant};

mod day1;
mod day2;
mod day3;

enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Part::One => "1",
            Part::Two => "2",
        })
    }
}

fn main() {
    run_fn("1", &day1::exec);
    run_fn("2", &day2::exec);
    run_fn("3", &day3::exec);
}

fn run_fn<T>(day: &str, f: &dyn Fn(&Part) -> T)
where
    T: ToString,
{
    for part in &vec![Part::One, Part::Two] {
        let start = Instant::now();
        let retval = f(part);
        println!(
            "Day {day} part {part}: {} ({} us)",
            retval.to_string(),
            start.elapsed().as_micros()
        )
    }
}
