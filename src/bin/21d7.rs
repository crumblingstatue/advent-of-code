#![feature(int_abs_diff)]

fn crab_pos(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.trim().split(',').map(|tok| tok.parse().unwrap())
}

fn part1(input: &str) -> u32 {
    let crabs = crab_pos(input).collect::<Vec<_>>();
    least_costly_point(&crabs)
}

fn fuel_cost(crabs: impl Iterator<Item = u32>, converge_point: u32) -> u32 {
    crabs.map(|c| c.abs_diff(converge_point)).sum()
}

fn least_costly_point(crabs: &[u32]) -> u32 {
    let mut min = crabs[0];
    let mut max = crabs[0];
    for crab in crabs {
        min = *crab.min(&min);
        max = *crab.max(&max);
    }
    (min..max)
        .map(|point| fuel_cost(crabs.iter().cloned(), point))
        .min()
        .unwrap()
}

#[cfg(test)]
const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

aoc::tests! {
    fn part1:
    TEST_INPUT => 37;
    => 352707;
}

aoc::main!(part1);
