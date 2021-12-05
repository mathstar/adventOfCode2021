use crate::day::Day;

pub struct Day3 {}

impl Day for Day3 {
    fn part1(&self, input: &str) -> String {
        let mut ones : Vec<u32> = Vec::new();
        let mut line_count = 0;

        for line in input.lines() {
            line_count += 1;
            for (i, c) in line.char_indices() {
                if c == '1' {
                    while ones.len() < i + 1 {
                        ones.push(0);
                    }
                    ones[i] += 1;
                }
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        for n in ones {
            gamma <<= 1;
            epsilon <<= 1;

            if n > line_count / 2 {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        }

        (gamma * epsilon).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut oxygen_values : Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut evaluation_index = 0;
        while oxygen_values.len() > 1 {
            let mut one_count = 0;
            let mut zero_count = 0;
            for v in &oxygen_values {
                if v[evaluation_index] == '1' {
                    one_count += 1;
                } else {
                    zero_count += 1;
                }
            }
            let desired_digit = if one_count >= zero_count {'1'} else {'0'};
            oxygen_values = oxygen_values.into_iter()
                .filter(|c| c[evaluation_index] == desired_digit)
                .collect();
            evaluation_index += 1;
        }

        let mut co2_values : Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut evaluation_index = 0;
        while co2_values.len() > 1 {
            let mut one_count = 0;
            let mut zero_count = 0;
            for v in &co2_values {
                if v[evaluation_index] == '1' {
                    one_count += 1;
                } else {
                    zero_count += 1;
                }
            }
            let desired_digit = if zero_count <= one_count {'0'} else {'1'};
            co2_values = co2_values.into_iter()
                .filter(|c| c[evaluation_index] == desired_digit)
                .collect();
            evaluation_index += 1;
        }

        let mut oxygen_value = 0;
        for c in &oxygen_values[0] {
            oxygen_value <<= 1;
            if *c == '1' {
                oxygen_value += 1;
            }
        }

        let mut co2_value = 0;
        for c in &co2_values[0] {
            co2_value <<= 1;
            if *c == '1' {
                co2_value += 1;
            }
        }

        (oxygen_value * co2_value).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day3{}.part1("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"), "198");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day3{}.part2("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"), "230");
    }
}
