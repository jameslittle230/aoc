use crate::Part;

pub(crate) fn exec(part: &Part) -> u32 {
    let contents = include_str!("../inputs/n.txt");

    match part {
        Part::One => todo!(),
        Part::Two => todo!(),
    }

    0
}

#[cfg(test)]
mod tests {
    use super::exec;
    use crate::Part;

    #[test]
    fn it_works() {
        assert_eq!(exec(&Part::One), 0);
        assert_eq!(exec(&Part::Two), 0);
    }
}
