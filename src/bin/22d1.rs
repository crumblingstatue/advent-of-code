// Ran out of time for a smart solution, here is the simple one

fn sorted_calories(input: &str) -> impl Iterator<Item = u32> {
    let mut cals = Vec::new();
    let mut cal = 0;
    for line in input.lines() {
        if line.is_empty() {
            cals.push(cal);
            cal = 0;
        } else {
            cal += line.parse::<u32>().unwrap();
        }
    }
    cals.push(cal);
    cals.sort_by(|a, b| a.cmp(b).reverse());
    cals.into_iter()
}

fn part1(input: &str) -> u32 {
    sorted_calories(input).next().unwrap()
}

fn part2(input: &str) -> u32 {
    sorted_calories(input).take(3).sum()
}

#[cfg(test)]
const LAST_MAX: &str = "\
0
1";

#[cfg(test)]
const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

aoc::tests!(
    fn part1:
    TEST_INPUT => 24000;
    LAST_MAX => 1;
    in => 69626;
    fn part2:
    TEST_INPUT => 45000;
    LAST_MAX => 1;
    in => 206780;
);

aoc::main!(part1, part2);
