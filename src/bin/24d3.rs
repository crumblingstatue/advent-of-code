#![feature(iter_array_chunks, array_try_map, let_chains)]

#[derive(Debug)]
struct Mul {
    lhs: u32,
    rhs: u32,
}

fn muls<const DO_DONT: bool>(mut input: &str) -> impl Iterator<Item = Mul> {
    std::iter::from_fn(move || {
        let mut do_ = true;
        loop {
            let pos = input.find('(')?;
            let head = &input[..pos];
            input = &input[pos + 1..];
            if head.ends_with("mul") {
                let Some(end) = input.find(')') else { continue };
                let Some(args) = &input[..end].split(',').array_chunks().next() else {
                    continue;
                };
                if do_ && let Ok([lhs, rhs]) = args.try_map(|arg| arg.parse()) {
                    return Some(Mul { lhs, rhs });
                }
            } else if DO_DONT && head.ends_with("do") {
                do_ = true;
            } else if DO_DONT && head.ends_with("don't") {
                do_ = false;
            }
        }
    })
}

fn part1(input: &str) -> u32 {
    muls::<false>(input).map(|mul| mul.lhs * mul.rhs).sum()
}

fn part2(input: &str) -> u32 {
    muls::<true>(input).map(|mul| mul.lhs * mul.rhs).sum()
}

#[cfg(test)]
const TEST_INPUT_PART_1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
// Sneaky bastard advent of code author, subtly changing test input between parts
#[cfg(test)]
const TEST_INPUT_PART_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

aoc::tests! {
fn part1:
    TEST_INPUT_PART_1 => 161;
    in => 167650499;
fn part2:
    TEST_INPUT_PART_2 => 48;
    in => 95846796;
}

#[test]
fn part1_ans_too_low() {
    assert!(part1(include_str!("24d3.txt")) > 25878120);
}

aoc::main!(part1, part2);
