use std::fs;

pub(crate) fn exec() -> u32 {
    let file_path = "./inputs/1.txt";
    let contents = fs::read_to_string(file_path).expect("couldn't read");

    contents
        .split("\n\n")
        .into_iter()
        .map(|elf_inventory| -> u32 {
            elf_inventory
                .split('\n')
                .into_iter()
                .map(|food_cal_value| food_cal_value.parse::<u32>().expect(""))
                .sum()
        })
        .max()
        .expect("")
}
