use std::collections::HashSet;

fn sequence_has_n_unique<const N: usize>(seq: &[u8]) -> bool {
    let mut encountered = HashSet::new();
    for &ch in seq.iter().take(N.min(seq.len())) {
        if encountered.contains(&ch) {
            return false;
        }
        encountered.insert(ch);
    }
    true
}

fn find_marker<const N: usize>(input: &str) -> usize {
    let input = input.as_bytes();

    for i in 0..input.len() {
        if sequence_has_n_unique::<N>(&input[i..]) {
            return i + N;
        }
    }
    panic!("Couldn't find marker");
}

fn part1(input: &str) -> usize {
    find_marker::<4>(input)
}

fn part2(input: &str) -> usize {
    find_marker::<14>(input)
}

aoc::tests! {
fn part1:
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7;
    "bvwbjplbgvbhsrlpgdmjqwftvncz" => 5;
    "nppdvjthqldpwncqszvftbrmjlhg" => 6;
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10;
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11;
    in => 1766;
fn part2:
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 19;
    "bvwbjplbgvbhsrlpgdmjqwftvncz" => 23;
    "nppdvjthqldpwncqszvftbrmjlhg" => 23;
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 29;
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 26;
    in => 2383;
}

aoc::main!(part1, part2);
