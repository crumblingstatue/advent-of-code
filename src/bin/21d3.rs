use std::cmp::Ordering;

#[derive(Default, Clone, Copy, Debug)]
struct BitCount {
    zero: u32,
    one: u32,
}

fn bit_counts(input: &str) -> Vec<BitCount> {
    let len = input.lines().next().unwrap().len();
    let mut counts = vec![BitCount::default(); len];
    for line in input.lines() {
        for (i, count) in counts.iter_mut().enumerate() {
            match line.as_bytes()[i] {
                b'0' => count.zero += 1,
                b'1' => count.one += 1,
                c => panic!("What? {} byte encountered ({})", c, c as char),
            }
        }
    }
    counts
}

fn bit_counts_2(bin_strings: &[Vec<u8>]) -> Vec<BitCount> {
    let mut counts = vec![BitCount::default(); bin_strings[0].len()];
    for bin_string in bin_strings {
        for (i, count) in counts.iter_mut().enumerate() {
            match bin_string[i] {
                0 => count.zero += 1,
                1 => count.one += 1,
                c => panic!("What? {} byte encountered ({})", c, c as char),
            }
        }
    }
    counts
}

fn most_common_bits(bc: &[BitCount]) -> Vec<u8> {
    bc.iter()
        .map(|bc| if bc.zero > bc.one { 0 } else { 1 })
        .collect()
}

fn least_common_bits(bc: &[BitCount]) -> Vec<u8> {
    bc.iter()
        .map(|bc| if bc.zero < bc.one { 0 } else { 1 })
        .collect()
}

fn bin_to_u32(digits: &[u8]) -> u32 {
    let mut accum = 0;
    for (i, &digit) in digits.iter().rev().enumerate() {
        accum += 2u32.pow(i as u32) * digit as u32;
    }
    accum
}

fn part1(input: &str) -> u32 {
    let counts = bit_counts(input);
    let gamma = bin_to_u32(&most_common_bits(&counts));
    let epsilon = bin_to_u32(&least_common_bits(&counts));
    gamma * epsilon
}

fn part2(input: &str) -> u32 {
    let mut binary_numbers: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        binary_numbers.push(
            line.bytes()
                .map(|b| if b == b'0' { 0 } else { 1 })
                .collect(),
        );
    }
    let oxygen_generator_rating = find_matching(binary_numbers.clone(), |count, bin_digit| {
        match count.zero.cmp(&count.one) {
            Ordering::Less | Ordering::Equal => bin_digit == 1,
            Ordering::Greater => bin_digit == 0,
        }
    });
    let co2_scrubber_rating = find_matching(binary_numbers.clone(), |count, bin_digit| match count
        .zero
        .cmp(&count.one)
    {
        Ordering::Less | Ordering::Equal => bin_digit == 0,
        Ordering::Greater => bin_digit == 1,
    });
    oxygen_generator_rating * co2_scrubber_rating
}

fn find_matching(
    mut binary_numbers: Vec<Vec<u8>>,
    mut retain_cond: impl FnMut(&BitCount, u8) -> bool,
) -> u32 {
    let mut i = 0;
    while binary_numbers.len() > 1 {
        let counts = bit_counts_2(&binary_numbers);
        binary_numbers.retain(|bin_string| {
            let count = &counts[i];
            retain_cond(count, bin_string[i])
        });
        i += 1;
    }
    bin_to_u32(&binary_numbers[0])
}

#[cfg(test)]
const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

aoc::tests! {
    fn bin_to_u32:
    &[1,0,1,1,0] => 22;
    &[0,1,0,0,1] => 9;
    fn part1:
    TEST_INPUT => 198;
    in => 2743844;
    fn part2:
    TEST_INPUT => 230;
    in => 6677951;
}

aoc::main!(part1, part2);
