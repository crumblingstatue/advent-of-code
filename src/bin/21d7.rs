#![feature(int_abs_diff)]

fn crab_pos(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.trim().split(',').map(|tok| tok.parse().unwrap())
}

fn part1(input: &str) -> u32 {
    let crabs = crab_pos(input).collect::<Vec<_>>();
    least_costly_point(&crabs, fuel_cost)
}

fn part2(input: &str) -> u32 {
    let crabs = crab_pos(input).collect::<Vec<_>>();
    least_costly_point(&crabs, fuel_cost_2)
}

fn fuel_cost(crabs: &[u32], converge_point: u32) -> u32 {
    crabs.iter().map(|&c| c.abs_diff(converge_point)).sum()
}

fn fuel_cost_2(crabs: &[u32], converge_point: u32) -> u32 {
    crabs
        .iter()
        .map(|&c| part2_movecost(c, converge_point))
        .sum()
}

fn part2_movecost(from: u32, to: u32) -> u32 {
    (1..=from.abs_diff(to)).sum()
}

#[test]
fn test_part2_movecost() {
    assert_eq!(part2_movecost(16, 5), 66);
    assert_eq!(part2_movecost(1, 5), 10);
    assert_eq!(part2_movecost(2, 5), 6);
    assert_eq!(part2_movecost(0, 5), 15);
    assert_eq!(part2_movecost(4, 5), 1);
    assert_eq!(part2_movecost(2, 5), 6);
    assert_eq!(part2_movecost(7, 5), 3);
    assert_eq!(part2_movecost(1, 5), 10);
    assert_eq!(part2_movecost(2, 5), 6);
    assert_eq!(part2_movecost(14, 5), 45);
}

fn least_costly_point<F>(crabs: &[u32], mut cost_fn: F) -> u32
where
    F: FnMut(&[u32], u32) -> u32,
{
    let mut min = crabs[0];
    let mut max = crabs[0];
    for crab in crabs {
        min = *crab.min(&min);
        max = *crab.max(&max);
    }
    (min..max).map(|point| cost_fn(crabs, point)).min().unwrap()
}

#[cfg(test)]
const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

aoc::tests! {
    fn part1:
    TEST_INPUT => 37;
    in => 352707;
    fn part2:
    TEST_INPUT => 168;
    in => 95519693;
}

aoc::main!(part1, part2);
