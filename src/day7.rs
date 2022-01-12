use std::cmp;
use std::collections::HashMap;
use crate::day::Day;

pub struct Day7 {}

fn increasing_cost(a: i32, b: i32) -> i32 {
    let mut cost = 0;
    for i in 1..(a-b).abs()+1 {
        cost += i;
    }
    cost
}

impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut counts = HashMap::new();

        for pos in input.lines().next().unwrap().split(",").map(|n| n.parse().unwrap()) {
            min = cmp::min(min, pos);
            max = cmp::max(max, pos);
            match counts.get_mut(&pos) {
                None => {counts.insert(pos, 1);},
                Some(c) => *c += 1
            }
        }

        let mut best = i32::MAX;
        for i in min..max {
            let mut cost = 0;
            for (pos, count) in counts.iter() {
                cost += count * (i - pos).abs();
            }
            best = cmp::min(best, cost);
        }
        best.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut counts = HashMap::new();

        for pos in input.lines().next().unwrap().split(",").map(|n| n.parse().unwrap()) {
            min = cmp::min(min, pos);
            max = cmp::max(max, pos);
            match counts.get_mut(&pos) {
                None => {counts.insert(pos, 1);},
                Some(c) => *c += 1
            }
        }

        let mut best = i32::MAX;
        for i in min..max {
            let mut cost = 0;
            for (pos, count) in counts.iter() {
                cost += count * increasing_cost(i, *pos);
            }
            best = cmp::min(best, cost);
        }
        best.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day7{}.part1("16,1,2,0,4,2,7,1,2,14"), "37");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day7{}.part2("16,1,2,0,4,2,7,1,2,14"), "168");
    }
}
