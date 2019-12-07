const STARTING_FLOOR: i32 = 0;

fn step(floor: &mut i32, instr: char) {
    match instr {
        '(' => *floor += 1,
        ')' => *floor -= 1,
        _ => panic!("Invalid instruction: {}", instr),
    }
}

fn part1(input: &str) -> i32 {
    let mut floor = STARTING_FLOOR;
    for ch in input.chars() {
        step(&mut floor, ch);
    }
    floor
}

fn part2(input: &str) -> i32 {
    let mut floor = STARTING_FLOOR;
    for (i, ch) in input.chars().enumerate() {
        step(&mut floor, ch);
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
        [280]
    test2 for part2 requires:
        ")" = 1
        "()())" = 5
        [1797]
}
