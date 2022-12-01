use crate::{AOCOutput, Variant};
use std::error::Error;

/**

# Part One

After saving Christmas five years in a row, you've decided to take a
vacation at a nice resort on a tropical island. Surely, Christmas will go on
without you.

The tropical island has its own currency and is entirely cash-only. The gold
coins used there have a little picture of a starfish; the locals just call
them stars. None of the currency exchanges seem to have heard of them, but
somehow, you'll need to find fifty of these coins by the time you arrive so
you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each
day in the Advent calendar; the second puzzle is unlocked when you complete
the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense
report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and
then multiply those two numbers together.

For example, suppose your expense report contained the following:

- 1721
- 979
- 366
- 299
- 675
- 1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying
them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum
to 2020; what do you get if you multiply them together?

# Part Two

The Elves in accounting are thankful for your help; one of them even offers
you a starfish coin they had left over from a past vacation. They offer you a
second one if you can find three numbers in your expense report that meet the
same criteria.

Using the above example again, the three entries that sum to 2020 are 979,
366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to
2020?

*/
pub fn main(buffer: &String, variant: Variant) -> Result<AOCOutput, Box<dyn Error>> {
    let mut numbers: Vec<u32> = buffer
        .split('\n')
        .into_iter()
        .map(|str| str.trim().parse().unwrap())
        .collect();

    &numbers.sort();

    let n = match variant {
        Variant::One => 2,
        Variant::Two => 3,
    };

    let entries = find_n_entries_summing_to(&numbers, n, 2020).unwrap();
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
