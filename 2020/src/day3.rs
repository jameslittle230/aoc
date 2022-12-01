use crate::{AOCOutput, Variant};
use std::error::Error;


/**
# Part One

With the toboggan login problems resolved, you set off toward the airport.
While travel by toboggan might be easy, it's certainly not safe: there's
very minimal steering and the area is covered in trees. You'll need to see
which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer
coordinates in a grid. You make a map (your puzzle input) of the open
squares (.) and trees (#) you can see. For example:

```
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
```

These aren't the only trees, though; due to something you read about once
involving arboreal genetics and biome stability, the same pattern repeats to
the right many times:

```
..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
```

You start on the open square (.) in the top-left corner and need to reach the
bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper
model that prefers rational numbers); start by counting all the trees you
would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right
3 and down 1. Then, check the position that is right 3 and down 1 from there,
and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where
there was an open square and X where there was a tree:

```
..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
```

In this example, traversing the map using this slope would cause you to
encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 3
and down 1, how many trees would you encounter?

# Part Two

Time to check the rest of the slopes - you need to minimize the probability
of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the
following slopes, you start at the top-left corner and traverse the map all
the way to the bottom:

- Right 1, down 1.
- Right 3, down 1. (This is the slope you already checked.)
- Right 5, down 1.
- Right 7, down 1.
- Right 1, down 2.

In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s)
respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on
each of the listed slopes?
*/
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