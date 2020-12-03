use aoc::array_2d::Array2D;

struct Instruction {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    kind: InstrKind,
}

enum InstrKind {
    On,
    Off,
    Toggle,
}

fn parse_instruction(input: &str) -> Instruction {
    use InstrKind::*;
    let mut words = input.split(' ');
    let w1 = words.next().unwrap();
    let kind;
    match w1 {
        "turn" => {
            let w2 = words.next().unwrap();
            match w2 {
                "on" => {
                    kind = On;
                }
                "off" => {
                    kind = Off;
                }
                etc => panic!("Unexpected: {}", etc),
            }
        }
        "toggle" => {
            kind = Toggle;
        }
        etc => panic!("Unexpected: {}", etc),
    }
    let mut pair1 = words.next().unwrap().split(',');
    let x1 = pair1.next().unwrap().parse().unwrap();
    let y1 = pair1.next().unwrap().parse().unwrap();
    // Skip "through"
    words.next().unwrap();
    let mut pair2 = words.next().unwrap().split(',');
    let x2 = pair2.next().unwrap().parse().unwrap();
    let y2 = pair2.next().unwrap().parse().unwrap();
    Instruction {
        x1,
        y1,
        x2,
        y2,
        kind,
    }
}

fn part1(input: &str) -> i32 {
    let mut grid = Array2D::new_filled(1000, 1000, false);
    for line in input.lines() {
        use InstrKind::*;
        let instr = parse_instruction(line);
        match instr.kind {
            On => {
                for y in instr.y1..=instr.y2 {
                    for x in instr.x1..=instr.x2 {
                        *grid.get_mut(x, y) = true;
                    }
                }
            }
            Off => {
                for y in instr.y1..=instr.y2 {
                    for x in instr.x1..=instr.x2 {
                        *grid.get_mut(x, y) = false;
                    }
                }
            }
            Toggle => {
                for y in instr.y1..=instr.y2 {
                    for x in instr.x1..=instr.x2 {
                        let state = *grid.get(x, y);
                        *grid.get_mut(x, y) = !state;
                    }
                }
            }
        }
    }
    grid.flat_iter().filter(|l| **l).count() as i32
}

fn part2(input: &str) -> i32 {
    let mut grid = Array2D::new_filled(1000, 1000, 0);
    for line in input.lines() {
        use InstrKind::*;
        let instr = parse_instruction(line);
        match instr.kind {
            On => {
                for y in instr.y1..=instr.y2 {
                    for x in instr.x1..=instr.x2 {
                        *grid.get_mut(x, y) += 1;
                    }
                }
            }
            Off => {
                for y in instr.y1..=instr.y2 {
                    for x in instr.x1..=instr.x2 {
                        let value = *grid.get(x, y);
                        if value > 0 {
                            *grid.get_mut(x, y) -= 1;
                        }
                    }
                }
            }
            Toggle => {
                for y in instr.y1..=instr.y2 {
                    for x in instr.x1..=instr.x2 {
                        *grid.get_mut(x, y) += 2;
                    }
                }
            }
        }
    }
    grid.flat_iter().sum()
}

aoc::tests! {
    fn part1:
    => 377891
    fn part2:
    => 14110788
}

aoc::main!(part1, part2);
