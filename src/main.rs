use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    process::exit,
};

pub mod day1;
pub mod day2;
pub mod day3;

pub struct AOCOutput {
    stderr: String,
    stdout: String,
}

pub enum Variant {
    One,
    Two,
}

pub type OperationOutput = Result<AOCOutput, Box<dyn Error>>;
pub type Operation = fn(&String, Variant) -> OperationOutput;

fn main() {
    let matches = App::new("AOC 2020")
        .arg(
            Arg::with_name("day")
                .short("d")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("variant").short("v").takes_value(true))
        .get_matches();

    let variant = matches.value_of("variant");
    let operation_result: OperationOutput = match matches.value_of("day").unwrap() {
        "1" => execute_puzzle("./inputs/1.txt", day1::main, variant),
        "2" => execute_puzzle("./inputs/2.txt", day2::main, variant),
        "3" => execute_puzzle("./inputs/3.txt", day3::main, variant),
        _ => {
            eprintln!("Day not recognized");
            exit(1);
        }
    };

    match operation_result {
        Ok(output) => {
            eprintln!("{}", output.stderr);
            println!("{}", output.stdout);
            exit(0)
        }

        Err(error) => {
            eprintln!("{}", error);
            exit(1)
        }
    }
}

fn execute_puzzle(path: &str, function: Operation, variant: Option<&str>) -> OperationOutput {
    let mut buffer = String::new();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut buffer)?;

    let variant = match variant {
        None => Variant::Two,
        Some(value) => match value {
            "1" => Variant::One,
            "2" => Variant::Two,
            _ => return Err(From::from("Variant not recognized")),
        },
    };

    function(&buffer, variant)
}
