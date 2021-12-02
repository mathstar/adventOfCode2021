use crate::day::Day;

pub struct Day2 {}

impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let mut x = 0;
        let mut y = 0;

        for line in input.lines() {
            let mut split = line.split(" ");
            match split.next() {
                Some("forward") => x += split.next().unwrap().parse::<i32>().unwrap(),
                Some("down") => y += split.next().unwrap().parse::<i32>().unwrap(),
                Some("up") => y -= split.next().unwrap().parse::<i32>().unwrap(),
                _ => panic!("invalid input")
            }
        }

        (x * y).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for line in input.lines() {
            let mut split = line.split(" ");
            match split.next() {
                Some("forward") => {
                    let change = split.next().unwrap().parse::<i32>().unwrap();
                    x += change;
                    y += aim * change;
                },
                Some("down") => aim += split.next().unwrap().parse::<i32>().unwrap(),
                Some("up") => aim -= split.next().unwrap().parse::<i32>().unwrap(),
                _ => panic!("invalid input")
            }
        }

        (x * y).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day2{}.part1("forward 5
down 5
forward 8
up 3
down 8
forward 2"), "150");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day2{}.part2("forward 5
down 5
forward 8
up 3
down 8
forward 2"), "900");
    }
}
