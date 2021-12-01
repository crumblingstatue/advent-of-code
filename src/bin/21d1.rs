use itertools::Itertools;

fn measurements(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.lines().map(|l| l.parse().unwrap())
}

fn part1(input: &str) -> i64 {
    let mut prev = None;
    let mut increases = 0;
    for m in measurements(input) {
        if let Some(prev) = prev {
            if m > prev {
                increases += 1;
            }
        }
        prev = Some(m);
    }
    increases
}

fn part2(input: &str) -> i64 {
    let mut prev = None;
    let mut increases = 0;
    for window @ (a, b, c) in measurements(input).tuple_windows() {
        if let Some((pa, pb, pc)) = prev {
            if a + b + c > pa + pb + pc {
                increases += 1;
            }
        }
        prev = Some(window);
    }
    increases
}

#[cfg(test)]
const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

aoc::tests!(
    fn part1:
        EXAMPLE => 7
        => 1527
    fn part2:
        EXAMPLE => 5
);

aoc::main!(part1, part2);
