fn part1(input: &str) -> i32 {
    input.lines().filter(|line| nice(line)).count() as i32
}

fn nice(string: &str) -> bool {
    let mut wovel_count = 0;
    let mut prev = '\0';
    let mut twice_row = false;
    let mut contains_naughty = false;
    for ch in string.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => wovel_count += 1,
            _ => {}
        }
        if ch == prev {
            twice_row = true;
        }
        if (prev == 'a' && ch == 'b')
            || (prev == 'c' && ch == 'd')
            || (prev == 'p' && ch == 'q')
            || (prev == 'x' && ch == 'y')
        {
            contains_naughty = true;
        }
        // Finally
        prev = ch;
    }
    wovel_count >= 3 && twice_row && !contains_naughty
}

fn nice2(string: &str) -> bool {
    use std::collections::HashMap;

    type Pair = (char, char);
    let mut last_pair: Option<Pair> = None;
    let mut pair_counts = HashMap::<Pair, i32>::new();
    let mut prev_ch = None;
    let mut contains_repeating_letter_one_between = false;

    for ch in string.chars() {
        if let Some(prev) = prev_ch {
            let pair = (prev, ch);
            let consider_pair;
            match last_pair {
                Some(last_pair) => {
                    // Overlapping, don't consider this a pair?
                    if last_pair == pair {
                        consider_pair = false;
                    } else {
                        consider_pair = true;
                    }
                    // Check if there is repeating
                    if last_pair.0 == ch {
                        contains_repeating_letter_one_between = true;
                    }
                }
                // Can't be overlapping, no previous pair
                None => consider_pair = true,
            }
            if consider_pair {
                *pair_counts.entry(pair).or_insert(0) += 1;
                last_pair = Some(pair);
            } else {
                last_pair = None;
            }
        }
        prev_ch = Some(ch);
    }
    let contains_nonoverlapping_pair = pair_counts.iter().any(|(_k, &v)| v > 1);
    contains_nonoverlapping_pair && contains_repeating_letter_one_between
}

fn part2(input: &str) -> i32 {
    input.lines().filter(|line| nice2(line)).count() as i32
}

aoc::tests! {
    fn nice:
    "ugknbfddgicrmopn" => true;
    "aaa" => true;
    "jchzalrnumimnmhp" => false;
    "haegwjzuvuyypxyu" => false;
    "dvszwmarrgswjxmb" => false;
    fn part1:
    in => 236;
    fn nice2:
    "qjhvhtzxzqqjkmpb" => true;
    "xxyxx" => true;
    "uurcxstgmygtbstg" => false;
    "ieodomkazucvgmuy" => false;
    "xxx" => false; // Teddy :3
    "yyyy" => true; // Teddy 2 :3
    fn part2:
    in => 51;
}

aoc::main!(part1, part2);
