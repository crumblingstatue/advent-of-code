use std::collections::HashSet;

fn sequence_has_4_unique(seq: &[u8]) -> bool {
    let mut encountered = HashSet::new();
    for &ch in seq.iter().take(4.min(seq.len())) {
        if encountered.contains(&ch) {
            return false;
        }
        encountered.insert(ch);
    }
    true
}

fn find_marker(input: &str) -> usize {
    let input = input.as_bytes();

    for i in 0..input.len() {
        if sequence_has_4_unique(&input[i..]) {
            return i + 4;
        }
    }
    panic!("Couldn't find marker");
}

fn part1(input: &str) -> usize {
    find_marker(input)
}

aoc::tests! {
fn sequence_has_4_unique:
    b"mjqj" => false;
    b"jpqm" => true;

fn find_marker:
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7;
    "bvwbjplbgvbhsrlpgdmjqwftvncz" => 5;
    "nppdvjthqldpwncqszvftbrmjlhg" => 6;
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10;
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11;
fn part1:
    in => 1766;
}

aoc::main!(part1);
