#![feature(array_windows)]

use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

#[cfg(test)]
const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

struct Report(Vec<u32>);

#[derive(PartialEq)]
enum Order {
    Inc,
    Dec,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut established_order = None;
        for &[a, b] in self.0.array_windows() {
            if !(1..=3).contains(&a.abs_diff(b)) {
                return false;
            }
            match a.cmp(&b) {
                Ordering::Less => {
                    if *established_order.get_or_insert(Order::Inc) != Order::Inc {
                        return false;
                    }
                }
                Ordering::Equal => return false,
                Ordering::Greater => {
                    if *established_order.get_or_insert(Order::Dec) != Order::Dec {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test_report_is_safe() {
    assert!(Report(vec![7, 6, 4, 2, 1]).is_safe());
    assert!(!Report(vec![1, 2, 7, 8, 9]).is_safe());
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Result<Vec<u32>, _> = s.split_whitespace().map(|tok| tok.parse()).collect();
        Ok(Report(nums?))
    }
}

fn reports(input: &str) -> impl Iterator<Item = Report> {
    input.lines().map(|line| Report::from_str(line).unwrap())
}

fn part1(input: &str) -> usize {
    reports(input).filter(Report::is_safe).count()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 2;
    in => 502;
}

aoc::main!(part1);
