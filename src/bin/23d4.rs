#[cfg(test)]
const TEST_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

aoc::main!(part1);

#[derive(Debug)]
struct Card {
    winning: Vec<u8>,
    actual: Vec<u8>,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let (_, nums) = s.split_once(": ").unwrap();
        let (winning, actual) = nums.split_once(" | ").unwrap();
        Self {
            winning: winning
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            actual: actual
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
    fn number_of_matches(&self) -> u32 {
        self.actual
            .iter()
            .filter(|num| self.winning.contains(num))
            .count() as u32
    }
    fn point_value(&self) -> u32 {
        let n = self.number_of_matches();
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            2_u32.pow(n - 1)
        }
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::from_str(line).point_value())
        .sum()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 13;
    in => 25174;
}
