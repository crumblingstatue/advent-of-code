#![feature(iter_array_chunks)]

#[cfg(test)]
const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn numpairs(input: &str) -> impl Iterator<Item = [u32; 2]> {
    input.lines().map(|l| {
        l.split_whitespace()
            .array_chunks()
            .next()
            .unwrap()
            .map(|tok| tok.parse().unwrap())
    })
}

fn part1(input: &str) -> u32 {
    let [mut fst, mut snd] = [Vec::new(), Vec::new()];
    for [n1, n2] in numpairs(input) {
        fst.push(n1);
        snd.push(n2);
    }
    fst.sort();
    snd.sort();
    fst.into_iter().zip(snd).map(|(a, b)| a.abs_diff(b)).sum()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 11;
    in => 2164381;
}

aoc::main!(part1);
