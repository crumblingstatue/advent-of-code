const STARTING_FLOOR: i32 = 0;

fn part1(input: &str) -> i32 {
    let mut floor = STARTING_FLOOR;
    for ch in input.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid instruction: {}", ch),
        }
    }
    floor
}

fn part2(input: &str) -> i32 {
    let mut floor = STARTING_FLOOR;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid instruction: {}", ch),
        }
        if floor == -1 {
            return i as i32 + 1;
        }
    }
    panic!("Never reached basement level");
}

aoc::main!(1 = part1, 2 = part2);
aoc::tests! {
    test1 for part1 requires:
        "(())" = 0
        "()()" = 0
        "(((" = 3
        "(()(()(" = 3
        "))(((((" = 3
        "())" = -1
        "))(" = -1
        ")))" = -3
        ")())())" = -3
    test2 for part2 requires:
        ")" = 1
        "()())" = 5
}
