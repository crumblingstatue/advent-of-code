#[derive(Default, Clone, Copy)]
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
    => 2743844;
}

aoc::main!(part1);
