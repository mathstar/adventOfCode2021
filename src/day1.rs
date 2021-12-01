use crate::day::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, input: &str) -> String {
        let mut increases = 0;
        let mut prev : Option<i32> = None;
        for n in input.lines() {
            let n = n.parse().unwrap();
            match prev {
                None => prev = Some(n),
                Some(p) => {
                    if n > p {
                        increases += 1;
                    }
                    prev = Some(n)
                }
            }
        }
        increases.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let nums : Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut a = nums[0] + nums[1] + nums[2];
        let mut b = nums[1] + nums[2] + nums[3];

        let mut increases = 0;
        if b > a {
            increases += 1;
        }

        for i in 4..nums.len() {
            a -= nums[i - 4];
            a += nums[i - 1];
            b -= nums[i - 3];
            b += nums[i];

            if b > a {
                increases += 1;
            }
        }

        increases.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day1{}.part1("199
200
208
210
200
207
240
269
260
263"), "7");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day1{}.part2("199
200
208
210
200
207
240
269
260
263"), "5");
    }
}
