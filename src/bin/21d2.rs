fn commands(input: &str) -> impl Iterator<Item = (&str, u32)> {
    input.lines().map(|line| {
        let (fst, sec) = line.split_once(' ').unwrap();
        (fst, sec.parse().unwrap())
    })
}

fn part1(input: &str) -> u32 {
    let mut hpos = 0;
    let mut depth = 0;
    for (dir, amount) in commands(input) {
        match dir {
            "forward" => hpos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Invalid direction: {dir}"),
        }
    }
    hpos * depth
}

fn part2(input: &str) -> u32 {
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (dir, amount) in commands(input) {
        match dir {
            "forward" => {
                hpos += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Invalid direction: {dir}"),
        }
    }
    hpos * depth
}

#[cfg(test)]
const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

aoc::tests! {
    fn part1:
    INPUT => 150;
    => 1762050;
    fn part2:
    INPUT => 900;
    => 1855892637;
}

aoc::main!(part1, part2);
