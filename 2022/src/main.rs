use std::time::Instant;

mod day1;
mod day2;

fn main() {
    run_fn("1", &day1::exec);
    run_fn("2", &day2::exec);
}

fn run_fn<T>(day: &str, f: &dyn Fn() -> T)
where
    T: ToString,
{
    let start = Instant::now();
    let retval = f();
    println!(
        "Day {day}: {} ({} us)",
        retval.to_string(),
        start.elapsed().as_micros()
    )
}
