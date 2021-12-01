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

aoc::tests!(
    fn part1:
"199
200
208
210
200
207
240
269
260
263" => 7
=> 1527
);

aoc::main!(part1);
