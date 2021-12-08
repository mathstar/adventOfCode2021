use std::cmp::{max, min};
use std::collections::HashMap;
use crate::day::Day;

pub struct Day5 {}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32)
}

enum AxialClassification {
    X, Y, NonAxial
}

impl Line {
    fn axial_classification(&self) -> AxialClassification {
        if self.start.0 == self.end.0 {
            AxialClassification::X
        } else if self.start.1 == self.end.1 {
            AxialClassification::Y
        } else {
            AxialClassification::NonAxial
        }
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let mut split = line.split(" -> ");
        let mut p_split = split.next().unwrap().split(",");
        let start = (p_split.next().unwrap().parse().unwrap(), p_split.next().unwrap().parse().unwrap());
        p_split = split.next().unwrap().split(",");
        let end = (p_split.next().unwrap().parse().unwrap(), p_split.next().unwrap().parse().unwrap());

        lines.push(Line {start, end});
    }

    lines
}

fn increment_grid_position(grid: &mut HashMap<i32, HashMap<i32, i32>>, overlap: &mut i32, x: i32, y: i32) {
    match grid.get_mut(&x) {
        Some(r) => {
            match r.get_mut(&y) {
                Some(existing) if *existing == 1 => {
                    *overlap += 1;
                    *existing += 1;
                }
                Some(existing) => {
                    *existing += 1;
                }
                None => {
                    r.insert(y, 1);
                }
            }
        },
        None => {
            let mut m = HashMap::new();
            m.insert(y, 1);
            grid.insert(x, m);

        }
    }
}

fn compute_overlap(lines: &Vec<Line>, consider_diagonal: bool) -> i32 {
    let mut grid : HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    let mut overlap = 0;
    for line in lines {
        match line.axial_classification() {
            AxialClassification::X => {
                let y_start = min(line.start.1, line.end.1);
                let y_end = max(line.start.1, line.end.1) + 1;
                for y in y_start .. y_end {
                    increment_grid_position(&mut grid, &mut overlap, line.start.0, y);
                }
            },
            AxialClassification::Y => {
                let x_start = min(line.start.0, line.end.0);
                let x_end = max(line.start.0, line.end.0) + 1;
                for x in x_start..x_end {
                    increment_grid_position(&mut grid, &mut overlap, x, line.start.1);
                }
            },
            AxialClassification::NonAxial => {
                if consider_diagonal {
                    let x_start = line.start.0;
                    let y_start = line.start.1;
                    let x_sign = if line.end.0 - line.start.0 > 0 {1} else {-1};
                    let y_sign = if line.end.1 - line.start.1 > 0 {1} else {-1};
                    let length = (line.start.0 - line.end.0).abs() + 1;
                    for inc in 0..length {
                        let x = x_start + inc * x_sign;
                        let y = y_start + inc * y_sign;
                        increment_grid_position(&mut grid, &mut overlap, x, y);
                    }
                }
            }
        }
    }

    overlap
}

impl Day for Day5 {
    fn part1(&self, input: &str) -> String {
        let lines = parse_input(input);
        compute_overlap(&lines, false).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let lines = parse_input(input);
        compute_overlap(&lines, true).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day5{}.part1("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"), "5");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day5{}.part2("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"), "12");
    }
}
