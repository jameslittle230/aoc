use crate::{AOCOutput, Variant};
use std::error::Error;

struct Slope {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
fn replace_nth_letter_in_string(original: &str, index: usize, replacement: char) -> String {
    original
        .chars()
        .enumerate()
        .map(|(map_index, char)| {
            if map_index == index {
                replacement
            } else {
                char
            }
        })
        .collect::<String>()
}

pub fn main(buffer: &String, variant: Variant) -> Result<AOCOutput, Box<dyn Error>> {
    let lines: Vec<&str> = buffer.split('\n').collect();
    let lines_count = &lines.clone().len();

    fn get_tree_count_with_slope(lines: &Vec<&str>, slope: &Slope) -> usize {
        let line_length = lines.first().unwrap().len();
        lines
            .into_iter()
            .enumerate()
            .step_by(slope.y)
            .map(|(y_position, line)| {
                let x_index =
                    (y_position as f64 * (slope.x as f64 / slope.y as f64)) as usize % line_length;
                match line.chars().collect::<Vec<char>>().get(x_index) {
                    Some('#') => 1,
                    _ => 0,
                }
            })
            .sum()
    }

    match variant {
        Variant::One => {
            let tree_count = get_tree_count_with_slope(&lines, &Slope { x: 3, y: 1 });
            let stderr = format!("{} trees in {} lines", tree_count, lines_count);
            let stdout = format!("{}", tree_count);
            Ok(AOCOutput { stderr, stdout })
        }

        Variant::Two => {
            let slopes = vec![
                Slope { x: 1, y: 1 },
                Slope { x: 3, y: 1 },
                Slope { x: 5, y: 1 },
                Slope { x: 7, y: 1 },
                Slope { x: 1, y: 2 },
            ];

            let mut stderr_components = vec![];

            let product: usize = slopes
                .into_iter()
                .map(|slope| {
                    let tree_count = get_tree_count_with_slope(&lines, &slope);
                    stderr_components.push(format!(
                        "Right {}, Down {}, Trees {}",
                        &slope.x, &slope.y, tree_count
                    ));
                    tree_count
                })
                .product();

            let stderr = format!("{}\n{}", stderr_components.join("\n"), product);
            let stdout = format!("{}", product);

            Ok(AOCOutput { stderr, stdout })
        }
    }
}
