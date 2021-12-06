use crate::day::Day;

pub struct Day4 {}

struct BingoBoard {
    values: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>
}

impl BingoBoard {
    fn new(values: Vec<Vec<i32>>) -> BingoBoard {
        let mut marked = Vec::new();
        for a in &values {
            let mut row = Vec::new();
            for _ in a {
                row.push(false);
            }
            marked.push(row);
        }

        BingoBoard {
            values,
            marked
        }
    }

    fn mark(&mut self, n: i32) {
        for (i, v) in self.values.iter().enumerate() {
            for (j, a) in v.iter().enumerate() {
                if *a == n {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for v in &self.marked {
            if v.iter().all(|b| *b) {
                return true;
            }
        }
        for i in 0..self.marked[0].len() {
            let mut all_marked = true;
            for v in &self.marked {
                if !v[i] {
                    all_marked = false;
                    break;
                }
            }
            if all_marked {
                return true;
            }
        }

        false
    }

    fn score(&self) -> i32 {
        self.marked.iter()
            .enumerate()
            .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, val)| (i, j, val)))
            .filter(|(_,_,val)| !**val)
            .map(|(i,j,_)| self.values[i][j])
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let calls : Vec<i32> = lines.next().unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    lines.next();

    let mut boards = Vec::new();
    let mut board = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            boards.push(BingoBoard::new(board));
            board = Vec::new();
        } else {
            board.push(line.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect());
        }
    }
    if !board.is_empty() {
        boards.push(BingoBoard::new(board));
    }

    (calls, boards)
}

impl Day for Day4 {
    fn part1(&self, input: &str) -> String {
        let (calls, mut boards) = parse_input(input);

        for call in calls {
            for board in &mut boards {
                board.mark(call);
            }

            for board in &boards {
                if board.has_won() {
                    return (board.score() * call).to_string();
                }
            }
        }

        String::new()
    }

    fn part2(&self, input: &str) -> String {
        let (calls, mut boards) = parse_input(input);

        let mut boards_won = Vec::new();
        for _ in &boards {
            boards_won.push(false);
        }
        let mut won_count = 0;

        for call in calls {
            for board in &mut boards {
                board.mark(call);
            }

            for (i, board) in boards.iter().enumerate() {
                if !boards_won[i] && board.has_won() {
                    won_count += 1;
                    boards_won[i] = true;
                    if won_count == boards.len() {
                        return (board.score() * call).to_string();
                    }
                }
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day4{}.part1("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"), "4512");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day4{}.part2("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"), "1924");
    }
}
