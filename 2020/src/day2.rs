use crate::{AOCOutput, Variant};
use std::{convert::TryFrom, convert::TryInto, error::Error};

/**
# Part One

Your flight departs in a few days from the coastal airport; the easiest way
down to the coast from here is via toboggan.
The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
"Something's wrong with our computers; we can't log in!" You ask if you can
take a look.
 
Their password database seems to be a little corrupted: some of the
passwords wouldn't have been allowed by the Official Toboggan Corporate
Policy that was in effect when they were chosen.
 
To try to debug the problem, they have created a list (your puzzle input) of
passwords (according to the corrupted database) and the corporate policy
when that password was set.
 
For example, suppose you have the following list:
 
- 1-3 a: abcde 
- 1-3 b: cdefg 
- 2-9 c: ccccccccc 

Each line gives the password
policy and then the password. The password policy indicates the lowest and
highest number of times a given letter must appear for the password to be
valid. For example, 1-3 a means that the password must contain a at least 1
time and at most 3 times.
 
In the above example, 2 passwords are valid. The middle password, cdefg, is
not; it contains no instances of b, but needs at least 1. The first and
third passwords are valid: they contain one a or nine c, both within the
limits of their respective policies.
 
How many passwords are valid according to their policies?
 
# Part Two

While it appears you validated the passwords correctly, they don't seem to
be what the Official Toboggan Corporate Authentication System is expecting.
 
The shopkeeper suddenly realizes that he just accidentally explained the
password policy rules from his old job at the sled rental place down the
street! The Official Toboggan Corporate Policy actually works a little
differently.
 
Each policy actually describes two positions in the password, where 1 means
the first character, 2 means the second character, and so on. (Be careful;
Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of
these positions must contain the given letter. Other occurrences of the
letter are irrelevant for the purposes of policy enforcement.
 
Given the same example list from above:
 
- 1-3 a: abcde is valid: position 1 contains a and position 3 does not. 
- 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b. 
- 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c. 

How many passwords are valid according to the new interpretation of the 
policies?
*/
pub fn main(buffer: &String, variant: Variant) -> Result<AOCOutput, Box<dyn Error>> {
    let count: u32 = buffer
        .split('\n')
        .into_iter()
        
        // map to password
        .map(|str| str.trim().to_string().try_into().unwrap())
       
        // validate password
        .map(|pw: Password| match variant {
            Variant::One => pw.is_valid_for_sled_shop(),
            Variant::Two => pw.is_valid_for_toboggan(),
        })
      
        // map to 1 or 0 to allow for sum
        .map(|bool| u32::from(bool))
        .sum::<u32>();

    let stderr = format!(
        "{} out of {}",
        count,
        buffer.split("\n").collect::<Vec<&str>>().len()
    );

    let stdout = format!("{}", count);

    Ok(AOCOutput { stderr, stdout })
}

struct PasswordPolicy {
    letter: char,
    min: u8,
    max: u8,
}

impl TryFrom<String> for PasswordPolicy {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value
            .split(|c| c == '-' || c == ' ')
            .map(|s| s.trim())
            .collect();

        if split.len() != 3 {
            return Err(From::from("Password policy formatted incorrectly"));
        }

        let min: u8 = split.get(0).unwrap().parse()?;
        let max: u8 = split.get(1).unwrap().parse()?;
        let letter: char = split.get(2).unwrap().parse()?;

        Ok(Self { min, max, letter })
    }
}

struct Password {
    policy: PasswordPolicy,
    password: String,
}

impl TryFrom<String> for Password {
    type Error = Box<dyn Error>;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        let split: Vec<&str> = string.split(":").map(|s| s.trim()).collect();

        if split.len() != 2 {
            return Err(From::from("Password entry formatted incorrectly"));
        }

        let policy = PasswordPolicy::try_from(split.get(0).unwrap().to_string())?;
        Ok(Self {
            policy,
            password: split.get(1).unwrap().to_string(),
        })
    }
}

impl Password {
    fn is_valid_for_sled_shop(&self) -> bool {
        let policy_letter_count = self
            .password
            .chars()
            .into_iter()
            .map(|char| char == self.policy.letter)
            .map(|bool| u8::from(bool))
            .sum::<u8>();

        (self.policy.min..=self.policy.max).contains(&policy_letter_count)
    }

    fn is_valid_for_toboggan(&self) -> bool {
        let first_position_letter = self.password.chars().nth((self.policy.min - 1) as usize);
        let second_position_letter = self.password.chars().nth((self.policy.max - 1) as usize);

        match (first_position_letter, second_position_letter) {
            (Some(first), Some(second)) => {
                let letter_one_should_match = self.policy.letter;
                (first == letter_one_should_match) ^ (second == letter_one_should_match)
            }
            _ => false,
        }
    }
}