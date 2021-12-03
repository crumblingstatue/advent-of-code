use std::collections::HashSet;

fn move_(pos: &mut (i32, i32), instr: char) {
    match instr {
        '^' => pos.1 -= 1,
        'v' => pos.1 += 1,
        '>' => pos.0 += 1,
        '<' => pos.0 -= 1,
        _ => panic!("Unknown instruction, you groggy elf!"),
    }
}

fn part1(input: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut pos = (0, 0);
    visited.insert(pos);
    for c in input.chars() {
        move_(&mut pos, c);
        visited.insert(pos);
    }
    visited.len() as i32
}

enum Turn {
    Santa,
    Robo,
}

fn part2(input: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut santa = (0, 0);
    let mut robo = (0, 0);
    let mut turn = Turn::Santa;
    visited.insert((0, 0));
    for c in input.chars() {
        let pos = match turn {
            Turn::Santa => {
                turn = Turn::Robo;
                &mut santa
            }
            Turn::Robo => {
                turn = Turn::Santa;
                &mut robo
            }
        };
        move_(pos, c);
        visited.insert(*pos);
    }
    visited.len() as i32
}

aoc::tests! {
    fn part1:
    ">" => 2;
    "^>v<" => 4;
    "^v^v^v^v^v" => 2;
    => 2572;
    fn part2:
    "^v" => 3;
    "^>v<" => 3;
    "^v^v^v^v^v" => 11;
    => 2631;
}

aoc::main!(part1, part2);
