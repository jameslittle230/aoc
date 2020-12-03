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

    fn do_the_thing(
        path: &str,
        function: fn(&String) -> Result<AOCOutput, Box<dyn Error>>,
    ) -> Result<AOCOutput, Box<dyn Error>> {
        let mut buffer = String::new();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut buffer)?;
        function(&buffer)
    }

    let operation_result: Result<AOCOutput, Box<dyn Error>> = match matches.value_of("day").unwrap()
    {
        "1" => do_the_thing("./inputs/1.txt", day1::main),
        "2" => do_the_thing("./inputs/2.txt", day2::main),
        "3" => do_the_thing("./inputs/3.txt", day3::main),
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
