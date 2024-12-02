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

#[derive(Clone)]
struct Report(Vec<u32>);

#[derive(PartialEq)]
enum Order {
    Inc,
    Dec,
}

impl Report {
    fn first_violation(&self) -> Option<usize> {
        let mut established_order = None;
        for (idx, &[a, b]) in self.0.array_windows().enumerate() {
            if !(1..=3).contains(&a.abs_diff(b)) {
                return Some(idx);
            }
            match a.cmp(&b) {
                Ordering::Less => {
                    if *established_order.get_or_insert(Order::Inc) != Order::Inc {
                        return Some(idx);
                    }
                }
                Ordering::Equal => return Some(idx),
                Ordering::Greater => {
                    if *established_order.get_or_insert(Order::Dec) != Order::Dec {
                        return Some(idx);
                    }
                }
            }
        }
        None
    }
    fn is_safe(&self) -> bool {
        self.first_violation().is_none()
    }
    fn is_safe_dampened(&self) -> bool {
        match self.first_violation() {
            Some(idx) => {
                let mut clone = self.clone();
                clone.0.remove(idx);
                clone.is_safe()
            }
            None => true,
        }
    }
}

#[test]
fn test_report_is_safe() {
    assert!(Report(vec![7, 6, 4, 2, 1]).is_safe());
    assert!(!Report(vec![1, 2, 7, 8, 9]).is_safe());
    assert_eq!(Report(vec![1, 3, 2, 4, 5]).first_violation(), Some(1));
    assert_eq!(Report(vec![8, 6, 4, 4, 1]).first_violation(), Some(2));
    assert!(Report(vec![1, 3, 2, 4, 5]).is_safe_dampened());
    assert!(Report(vec![8, 6, 4, 4, 1]).is_safe_dampened());
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

fn part2(input: &str) -> usize {
    reports(input).filter(Report::is_safe_dampened).count()
}

#[test]
fn part2_too_low() {
    // The answer '527' is too low for part 2
    assert!(part2(include_str!("24d2.txt")) > 527)
}

aoc::tests! {
fn part1:
    TEST_INPUT => 2;
    in => 502;
fn part2:
    TEST_INPUT => 4;
}

aoc::main!(part1, part2);
