use crate::day::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, _input: &str) -> String {
        String::new()
    }

    fn part2(&self, _input: &str) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day1{}.part1(""), "");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day1{}.part2(""), "");
    }
}
