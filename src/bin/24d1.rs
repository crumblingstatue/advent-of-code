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

fn get_lists(input: &str) -> [Vec<u32>; 2] {
    let [mut l_list, mut r_list] = [Vec::new(), Vec::new()];
    for [l, r] in numpairs(input) {
        l_list.push(l);
        r_list.push(r);
    }
    [l_list, r_list]
}

fn part1(input: &str) -> u32 {
    let [mut l_list, mut r_list] = get_lists(input);
    l_list.sort();
    r_list.sort();
    l_list
        .into_iter()
        .zip(r_list)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2(input: &str) -> u32 {
    let [l_list, r_list] = get_lists(input);
    l_list
        .into_iter()
        .map(|l| r_list.iter().filter(|&&r| l == r).count() as u32 * l)
        .sum()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 11;
    in => 2164381;
fn part2:
    TEST_INPUT => 31;
    in => 20719933;
}

aoc::main!(part1, part2);
