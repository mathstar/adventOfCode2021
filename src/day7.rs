use std::cmp;
use std::collections::HashMap;
use crate::day::Day;

pub struct Day7 {}

trait Cost {
    fn cost(&mut self, a: i32, b: i32) -> i32;
}

struct BasicCost {}

impl Cost for BasicCost {
    fn cost(&mut self, a: i32, b: i32) -> i32 {
        (a - b).abs()
    }
}

struct IncreasingCostCache {
    cache: Vec<i32>
}

impl IncreasingCostCache {
    fn new() -> IncreasingCostCache {
        IncreasingCostCache {cache: vec![0]}
    }
}

impl Cost for IncreasingCostCache {
    fn cost(&mut self, a: i32, b: i32) -> i32 {
        let dist = (a-b).abs() as usize;
        while self.cache.len() < dist + 1 {
            self.cache.push(self.cache[self.cache.len() - 1] + self.cache.len() as i32);
        }
        self.cache[dist]
    }
}

fn compute_min_cost(input: &str, cost_method: &mut dyn Cost) -> String {
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
            cost += count * cost_method.cost(i, *pos);
        }
        best = cmp::min(best, cost);
    }
    best.to_string()
}

impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        compute_min_cost(input, &mut BasicCost{})
    }

    fn part2(&self, input: &str) -> String {
        compute_min_cost(input, &mut IncreasingCostCache::new())
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
