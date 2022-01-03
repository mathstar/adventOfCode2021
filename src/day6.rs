use crate::day::Day;

pub struct Day6 {}

fn cycle_counts(mut counts: Vec<u64>) -> Vec<u64> {
    let zeros = counts[0];
    for i in 1..counts.len() {
        counts[i-1] = counts[i];
    }
    counts[6] += zeros;
    counts[8] = zeros;

    counts
}

impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        let mut counts = vec!(0u64,0u64,0u64,0u64,0u64,0u64,0u64,0u64,0u64);

        for i in input.lines().next().unwrap().split(",") {
            counts[i.parse::<usize>().unwrap()] += 1;
        }

        for _ in 0..80 {
            counts = cycle_counts(counts);
        }

        counts.iter().sum::<u64>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut counts = vec!(0u64,0u64,0u64,0u64,0u64,0u64,0u64,0u64,0u64);

        for i in input.lines().next().unwrap().split(",") {
            counts[i.parse::<usize>().unwrap()] += 1;
        }

        for _ in 0..256 {
            counts = cycle_counts(counts);
        }

        counts.iter().sum::<u64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day6{}.part1("3,4,3,1,2"), "5934");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day6{}.part2("3,4,3,1,2"), "26984457539");
    }
}
