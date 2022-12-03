#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

fn priority(item: u8) -> u8 {
    let base = if item.is_ascii_lowercase() {
        b'a'
    } else {
        b'A' - 26
    };
    (item - base) + 1
}

type Rucksack<'a> = (&'a [u8], &'a [u8]);

fn rucksack(input: &[u8]) -> Rucksack {
    let middle = input.len() / 2;
    input.split_at(middle)
}

fn rucksacks(input: &str) -> impl Iterator<Item = Rucksack> {
    input.lines().map(|l| rucksack(l.as_bytes()))
}

fn part1(input: &str) -> u32 {
    rucksacks(input)
        .map(|(fst, sec)| {
            let fst: HashSet<u8> = fst.iter().cloned().collect();
            let sec: HashSet<u8> = sec.iter().cloned().collect();
            fst.intersection(&sec)
                .map(|item| priority(*item) as u32)
                .sum::<u32>()
        })
        .sum()
}

type Group<'a> = [&'a [u8]; 3];

fn groups(input: &str, mut f: impl FnMut(Group)) {
    for chunk in input.lines().map(|l| l.as_bytes()).array_chunks() {
        f(chunk)
    }
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    groups(input, |group| {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        for sack in group {
            let set: HashSet<u8> = sack.iter().cloned().collect();
            for item in set {
                *counts.entry(item).or_insert(0) += 1;
            }
        }
        sum += counts
            .into_iter()
            .filter_map(|(k, v)| (v == 3).then(|| priority(k) as u32))
            .sum::<u32>()
    });
    sum
}

#[cfg(test)]
const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

aoc::tests! {
fn priority:
    b'a' => 1;
    b'z' => 26;
    b'A' => 27;
    b'Z' => 52;
fn rucksack:
    b"vJrwpWtwJgWrhcsFMMfFFhFp" => (b"vJrwpWtwJgWr".as_slice(), b"hcsFMMfFFhFp".as_slice());
fn part1:
    TEST_INPUT => 157;
    in => 8233;
fn part2:
    TEST_INPUT => 70;
    in => 2821;
}

aoc::main!(part1, part2);
