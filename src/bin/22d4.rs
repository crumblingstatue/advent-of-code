use std::ops::RangeInclusive;

type SectionId = u8;

type IdRange = RangeInclusive<SectionId>;

fn parse_range(input: &str) -> IdRange {
    let (lo, hi) = input.split_once('-').unwrap();
    lo.parse().unwrap()..=hi.parse().unwrap()
}

type AssignmentPair = [IdRange; 2];

fn parse_assignment_pair(input: &str) -> AssignmentPair {
    let (fst, snd) = input.split_once(',').unwrap();
    [parse_range(fst), parse_range(snd)]
}

trait RangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeExt for IdRange {
    fn contains_range(&self, other: &Self) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.end() >= other.start() && self.start() <= other.end()
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_assignment_pair)
        .filter(|[fst, snd]| fst.contains_range(snd) || snd.contains_range(fst))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_assignment_pair)
        .filter(|[fst, snd]| fst.overlaps(snd))
        .count()
}

#[cfg(test)]
const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

aoc::tests! {
fn parse_range:
    "2-4" => 2..=4;
fn parse_assignment_pair:
    "5-7,7-9" => [5..=7, 7..=9];
fn part1:
    TEST_INPUT => 2;
    in => 538;
fn part2:
    TEST_INPUT => 4;
    "9-10,4-8" => 0; // First end higher than second, no overlap
    in => 792;
}

aoc::main!(part1, part2);
