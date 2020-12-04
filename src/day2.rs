use crate::{AOCOutput, Variant};
use std::{convert::TryFrom, convert::TryInto, error::Error};

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
