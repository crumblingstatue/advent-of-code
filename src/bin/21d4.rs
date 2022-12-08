use std::{fmt::Debug, num::ParseIntError, str::FromStr};

use aoc::array_2d::Array2D;

#[derive(PartialEq)]
struct Markable {
    num: u32,
    mark: bool,
}

impl Debug for Markable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)?;
        if self.mark {
            write!(f, "x")?;
        }
        Ok(())
    }
}

impl FromStr for Markable {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            num: s.parse()?,
            mark: false,
        })
    }
}

#[derive(Debug)]
struct Board {
    array: Array2D<Markable>,
}

impl Board {
    fn from_str(input: &str) -> Self {
        Self {
            array: Array2D::from_flat(5, input.split_whitespace().map(|tok| tok.parse().unwrap())),
        }
    }
    fn mark_num(&mut self, num: u32) {
        for field in self.array.flat_mut() {
            if field.num == num {
                field.mark = true;
            }
        }
    }
    fn winning(&self) -> bool {
        self.array
            .rows()
            .any(|row| row.iter().all(|field| field.mark))
            || self
                .array
                .cols()
                .any(|col| col.iter().all(|field| field.mark))
    }
    fn unmarked(&self) -> impl Iterator<Item = u32> + '_ {
        self.array
            .flat_iter()
            .filter_map(|m| if !m.mark { Some(m.num) } else { None })
    }
}

#[test]
fn test_board_from_str() {
    let nums = "22 13 17 11  0
     8   2 23  4 24
    21   9 14 16  7
     6  10  3 18  5
     1  12 20 15 19";
    let board = Board::from_str(nums);
    assert_eq!(
        board.array.get(0, 0),
        &Markable {
            num: 22,
            mark: false
        }
    );
    assert_eq!(
        board.array.get(1, 1),
        &Markable {
            num: 2,
            mark: false
        }
    );
    assert_eq!(
        board.array.get(4, 4),
        &Markable {
            num: 19,
            mark: false
        }
    );
}

fn draw_and_boards(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut segments = input.split("\n\n");
    let draw: Vec<u32> = segments
        .next()
        .unwrap()
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .collect();
    let boards: Vec<Board> = segments.map(Board::from_str).collect();
    (draw, boards)
}

fn part1(input: &str) -> u32 {
    let (draw, mut boards) = draw_and_boards(input);
    for draw_num in draw {
        for board in &mut boards {
            board.mark_num(draw_num);
            if board.winning() {
                return board.unmarked().sum::<u32>() * draw_num;
            }
        }
    }
    panic!("No winning board")
}

fn part2(input: &str) -> u32 {
    let (draw, mut boards) = draw_and_boards(input);
    let mut winning_sum = 0;
    for draw_num in draw {
        boards.retain_mut(|board| {
            board.mark_num(draw_num);
            if board.winning() {
                winning_sum = board.unmarked().sum::<u32>() * draw_num;
                false
            } else {
                true
            }
        });
    }
    winning_sum
}

#[cfg(test)]
const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7";

aoc::tests!(
    fn part1:
    TEST_INPUT => 4512;
    in => 45031;
    in !=> 11067;
    fn part2:
    TEST_INPUT => 1924;
    in => 2568;
);

aoc::main!(part1, part2);
