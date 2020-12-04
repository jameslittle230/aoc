use crate::{AOCOutput, Variant};
use std::{collections::HashMap, convert::TryFrom, error::Error};

/**

# Part One

You arrive at the airport only to realize that you grabbed your North Pole
Credentials instead of your passport. While these documents are extremely
similar, North Pole Credentials aren't issued by a country and therefore
aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long
line has formed for the automatic passport scanners, and the delay could
upset your travel itinerary.

Due to some questionable network security, you realize you might be able to
solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble
detecting which passports have all required fields. The expected fields are
as follows:

- byr (Birth Year)
- iyr (Issue Year)
- eyr (Expiration Year)
- hgt (Height)
- hcl (Hair Color)
- ecl (Eye Color)
- pid (Passport ID)
- cid (Country ID)

Passport data is validated in batch files (your puzzle input). Each passport
is represented as a sequence of key:value pairs separated by spaces or
newlines. Passports are separated by blank lines.

Here is an example batch file containing four passports:

```
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
```

The first passport is valid - all eight fields are present. The second
passport is invalid - it is missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it looks
like data from North Pole Credentials, not a passport at all! Surely, nobody
would mind if you made the system temporarily ignore missing cid fields.
Treat this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is fine,
but missing any other field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid
passports.

Count the number of valid passports - those that have all required fields.
Treat cid as optional. In your batch file, how many passports are valid?

# Part Two

The line is moving more quickly now, but you overhear airport security
talking about how passports with invalid data are getting through. Better add
some data validation, quick!

You can continue to ignore the cid field, but each other field has strict
rules about what values are valid for automatic validation:

- byr (Birth Year) - four digits; at least 1920 and at most 2002.
- iyr (Issue Year) - four digits; at least 2010 and at most 2020.
- eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
- hgt (Height) - a number followed by either cm or in:
    - If cm, the number must be at least 150 and at most 193.
    - If in, the number must be at least 59 and at most 76.
- hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
- ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
- pid (Passport ID) - a nine-digit number, including leading zeroes.
- cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present
and valid according to the above rules. Here are some example values:

*/
pub fn main(buffer: &String, variant: Variant) -> Result<AOCOutput, Box<dyn Error>> {
    let passport_count_with_fields_present: usize = buffer
        .split("\n\n")
        .map(Passport::try_from)
        .map(|result: Result<Passport, Box<dyn Error>>| result.is_ok())
        .fold(0, |acc, x| acc + (if x { 1 } else { 0 }));

    let passport_count_with_valid_values: usize = buffer
        .split("\n\n")
        .map(Passport::try_from)
        .map(|result| match result {
            Err(_e) => 0,
            Ok(passport) => {
                if passport.is_valid() {
                    1
                } else {
                    0
                }
            }
        })
        .sum();

    let valid_passport_count = match variant {
        Variant::One => passport_count_with_fields_present,
        Variant::Two => passport_count_with_valid_values,
    };

    Ok(AOCOutput {
        stderr: format!(
            "{} out of {} passports",
            valid_passport_count,
            buffer.split("\n\n").collect::<Vec<&str>>().len()
        ),

        stdout: format!("{}", valid_passport_count),
    })
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn is_valid(&self) -> bool {
        fn length_and_within_range(string: &String, len: usize, min: u32, max: u32) -> bool {
            let r = string.parse::<u32>();
            let string_len = string.chars().collect::<Vec<char>>().len();
            match r {
                Err(_e) => false,
                Ok(num) => (min..=max).contains(&num) && len == string_len,
            }
        }
        fn byr_valid(byr: &String) -> bool {
            length_and_within_range(byr, 4, 1920, 2002)
        }

        fn iyr_valid(iyr: &String) -> bool {
            length_and_within_range(iyr, 4, 2010, 2020)
        }

        fn eyr_valid(eyr: &String) -> bool {
            length_and_within_range(eyr, 4, 2020, 2030)
        }

        fn hgt_valid(hgt: &String) -> bool {
            let unit = hgt.chars().rev().take(2).collect::<String>();
            let mut number = hgt.clone();
            number.pop();
            number.pop();

            match unit.as_str() {
                "mc" => length_and_within_range(&number, 3, 150, 193),
                "ni" => length_and_within_range(&number, 2, 59, 76),
                _ => false,
            }
        }

        fn hcl_valid(hcl: &String) -> bool {
            let vec: Vec<char> = hcl.chars().collect();
            let (octothorpe, hex_string) = vec.split_at(1);
            let valid_hex_string = hex_string
                .into_iter()
                .fold(true, |acc, char| acc && char.is_ascii_hexdigit());
            octothorpe[0] == '#' && valid_hex_string
        }

        fn ecl_valid(ecl: &String) -> bool {
            let valids = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            valids.contains(&ecl.as_str())
        }

        fn pid_valid(pid: &String) -> bool {
            let string_length = pid.chars().collect::<Vec<char>>().len();
            let all_numeric = pid.chars().fold(true, |acc, char| acc & char.is_numeric());
            string_length == 9 && all_numeric
        }

        let validations: Vec<(&str, &String, bool)> = vec![
            ( "byr", &self.byr, byr_valid(&self.byr)),
            ("iyr", &self.iyr, iyr_valid(&self.iyr)),
            ("eyr", &self.eyr, eyr_valid(&self.eyr)),
            ("hgt", &self.hgt, hgt_valid(&self.hgt)),
            ("hcl", &self.hcl, hcl_valid(&self.hcl)),
            ("ecl", &self.ecl, ecl_valid(&self.ecl)),
            ("pid", &self.pid, pid_valid(&self.pid)),
        ];

        validations.into_iter().fold(true, |acc, v| acc && v.2)
    }
}

impl TryFrom<&str> for Passport {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut passport_entries = HashMap::new();
        value
            .split(|char| char == ' ' || char == '\n')
            .map(|entry| {
                let vec: Vec<&str> = entry.split(":").collect();
                (vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone())
            })
            .for_each(|(k, v)| {
                passport_entries.insert(k, v);
            });

        let byr = passport_entries.get("byr").ok_or("No byr")?.to_string();
        let iyr = passport_entries.get("iyr").ok_or("No iyr")?.to_string();
        let eyr = passport_entries.get("eyr").ok_or("No eyr")?.to_string();
        let hgt = passport_entries.get("hgt").ok_or("No hgt")?.to_string();
        let hcl = passport_entries.get("hcl").ok_or("No hcl")?.to_string();
        let ecl = passport_entries.get("ecl").ok_or("No ecl")?.to_string();
        let pid = passport_entries.get("pid").ok_or("No pid")?.to_string();

        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        })
    }
}
