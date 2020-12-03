use super::AOCOutput;
use std::error::Error;

#[allow(dead_code)]
fn find_pair(numbers: &Vec<u32>, target: u32) -> Result<(u32, u32), Box<dyn Error>> {
    for number in numbers.into_iter() {
        if numbers.contains(&(target - number)) {
            return Ok((number.to_owned(), target - number));
        }
    }

    Err(From::from("No pair"))
}

#[allow(dead_code)]
fn find_triplet(numbers: &Vec<u32>, target: u32) -> Result<(u32, u32, u32), Box<dyn Error>> {
    for number in numbers.into_iter() {
        if let Ok(pair) = find_pair(numbers, target - number) {
            return Ok((number.to_owned(), pair.0, pair.1));
        }
    }

    Err(From::from("No triplet"))
}

fn find_n_entries_summing_to(
    numbers: &Vec<u32>,
    n: u8,
    target: u32,
) -> Result<Vec<u32>, Box<dyn Error>> {
    match n {
        1 => {
            if numbers.contains(&target) {
                return Ok(vec![target]);
            } else {
                return Err(From::from("No"));
            }
        }

        _ => {
            for (index, number) in numbers.into_iter().enumerate() {
                let mut vec_without_this_number = numbers.clone();
                vec_without_this_number.remove(index);

                let new_n_value = n
                    .checked_sub(1)
                    .ok_or::<Box<dyn Error>>(From::from("n too small"))?;

                let new_target_value = target
                    .checked_sub(number.to_owned())
                    .ok_or::<Box<dyn Error>>(From::from("target too small"))?;

                if let Ok(recursive_result) = find_n_entries_summing_to(
                    &vec_without_this_number,
                    new_n_value,
                    new_target_value,
                ) {
                    let mut result_so_far = recursive_result.clone();
                    result_so_far.push(number.clone());
                    return Ok(result_so_far);
                }
            }

            return Err(From::from("Result not found"));
        }
    }
}

pub fn main(buffer: &String) -> Result<AOCOutput, Box<dyn Error>> {
    let mut numbers: Vec<u32> = buffer
        .split('\n')
        .into_iter()
        .map(|str| str.trim().parse().unwrap())
        .collect();

    &numbers.sort();

    let entries = find_n_entries_summing_to(&numbers, 3, 2020).unwrap();
    let mut product = 1;
    for entry in &entries {
        product *= entry;
    }

    let stderr = format!(
        "{} = {}",
        &entries
            .into_iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(" × "),
        product
    );

    let stdout = format!("{}", product);

    Ok(AOCOutput { stderr, stdout })
}
