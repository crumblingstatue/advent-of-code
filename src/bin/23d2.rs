#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    fn from_str(s: &str) -> Self {
        match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Stack {
    color: CubeColor,
    amount: u32,
}

type Set = Vec<Stack>;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}
impl Game {
    fn possible_with(&self, red: u32, green: u32, blue: u32) -> bool {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for set in &self.sets {
            for stack in set {
                let c = match stack.color {
                    CubeColor::Red => &mut red_max,
                    CubeColor::Green => &mut green_max,
                    CubeColor::Blue => &mut blue_max,
                };
                *c = std::cmp::max(*c, stack.amount);
            }
        }
        red >= red_max && green >= green_max && blue >= blue_max
    }
}

fn parse_game(line: &str) -> Game {
    let (game_id, sets) = line.split_once(':').unwrap();
    Game {
        id: extract_game_id(game_id),
        sets: extract_sets(sets),
    }
}

fn extract_sets(sets_str: &str) -> Vec<Set> {
    let mut sets = vec![];
    for set_str in sets_str.split(';') {
        let mut set = vec![];
        for stack in set_str.split(',') {
            let (count, color) = stack.trim().split_once(' ').unwrap();
            set.push(Stack {
                color: CubeColor::from_str(color),
                amount: count.parse().unwrap(),
            });
        }
        sets.push(set);
    }
    sets
}

fn extract_game_id(game_id: &str) -> u32 {
    game_id.strip_prefix("Game ").unwrap().parse().unwrap()
}

#[test]
fn test_parse_game() {
    use CubeColor::*;
    assert_eq!(
        parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        Game {
            id: 1,
            sets: vec![
                vec![
                    Stack {
                        color: Blue,
                        amount: 3
                    },
                    Stack {
                        color: Red,
                        amount: 4
                    },
                ],
                vec![
                    Stack {
                        color: Red,
                        amount: 1
                    },
                    Stack {
                        color: Green,
                        amount: 2
                    },
                    Stack {
                        color: Blue,
                        amount: 6
                    },
                ],
                vec![Stack {
                    color: Green,
                    amount: 2
                },],
            ]
        }
    )
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let game = parse_game(line);
            game.possible_with(12, 13, 14).then_some(game.id)
        })
        .sum()
}

aoc::main!(part1);

#[cfg(test)]
const TEST_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

aoc::tests! {
fn part1:
    TEST_INPUT => 8;
    in => 2795;
}
