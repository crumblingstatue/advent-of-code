#![feature(iter_array_chunks, array_try_map, try_blocks)]

#[derive(Debug)]
struct Mul {
    lhs: u32,
    rhs: u32,
}

fn muls(mut input: &str) -> impl Iterator<Item = Mul> {
    std::iter::from_fn(move || {
        loop {
            let pos = input.find("mul(")?;
            input = &input[pos + 4..];
            let Some(end) = input.find(')') else { continue };
            let Some(args) = &input[..end].split(',').array_chunks().next() else {
                continue;
            };
            if let Ok([lhs, rhs]) = args.try_map(|arg| arg.parse()) {
                return Some(Mul { lhs, rhs });
            }
        }
    })
}

fn part1(input: &str) -> u32 {
    muls(input).map(|mul| mul.lhs * mul.rhs).sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

aoc::tests! {
fn part1:
    TEST_INPUT => 161;
    in => 167650499;
}

#[test]
fn part1_ans_too_low() {
    assert!(part1(include_str!("24d3.txt")) > 25878120);
}

aoc::main!(part1);
