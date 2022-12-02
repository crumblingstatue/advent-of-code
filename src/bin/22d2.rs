const OPP_ROCK: u8 = b'A';
const OPP_PAPER: u8 = b'B';
const OPP_SCISSORS: u8 = b'C';

const MY_ROCK: u8 = b'X';
const MY_PAPER: u8 = b'Y';
const MY_SCISSORS: u8 = b'Z';

const ROCK_SCORE: u8 = 1;
const PAPER_SCORE: u8 = 2;
const SCISSORS_SCORE: u8 = 3;

const WIN_SCORE: u8 = 6;
const DRAW_SCORE: u8 = 3;

const LOSE_CHAR: u8 = b'X';
const DRAW_CHAR: u8 = b'Y';
const WIN_CHAR: u8 = b'Z';

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_letter(letter: u8) -> Self {
        match letter {
            OPP_ROCK | MY_ROCK => Self::Rock,
            OPP_PAPER | MY_PAPER => Self::Paper,
            OPP_SCISSORS | MY_SCISSORS => Self::Scissors,
            _ => panic!("Invalid letter: '{letter}'"),
        }
    }
    fn score(&self) -> u8 {
        match self {
            Move::Rock => ROCK_SCORE,
            Move::Paper => PAPER_SCORE,
            Move::Scissors => SCISSORS_SCORE,
        }
    }
}

struct Round {
    opponent: Move,
    me: Move,
}

impl Round {
    fn score(&self) -> u8 {
        match (&self.opponent, &self.me) {
            (Move::Rock, Move::Rock) => DRAW_SCORE + ROCK_SCORE,
            (Move::Rock, Move::Paper) => WIN_SCORE + PAPER_SCORE,
            (Move::Rock, Move::Scissors) => SCISSORS_SCORE,
            (Move::Paper, Move::Rock) => ROCK_SCORE,
            (Move::Paper, Move::Paper) => DRAW_SCORE + PAPER_SCORE,
            (Move::Paper, Move::Scissors) => WIN_SCORE + SCISSORS_SCORE,
            (Move::Scissors, Move::Rock) => WIN_SCORE + ROCK_SCORE,
            (Move::Scissors, Move::Paper) => PAPER_SCORE,
            (Move::Scissors, Move::Scissors) => DRAW_SCORE + SCISSORS_SCORE,
        }
    }
}

fn rounds(input: &str) -> impl Iterator<Item = Round> + '_ {
    input.lines().map(|line| {
        let bytes = line.as_bytes();
        Round {
            opponent: Move::from_letter(bytes[0]),
            me: Move::from_letter(bytes[2]),
        }
    })
}

fn part1(input: &str) -> u32 {
    rounds(input).map(|r| r.score() as u32).sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "\
A Y
B X
C Z";

enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl RoundResult {
    fn from_letter(letter: u8) -> Self {
        match letter {
            LOSE_CHAR => Self::Lose,
            DRAW_CHAR => Self::Draw,
            WIN_CHAR => Self::Win,
            _ => panic!("Invalid letter: '{letter}'"),
        }
    }
    fn score(&self) -> u8 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => DRAW_SCORE,
            RoundResult::Win => WIN_SCORE,
        }
    }
}

fn move_to_play(opp_move: &Move, result: &RoundResult) -> Move {
    match (opp_move, result) {
        (Move::Rock, RoundResult::Lose) => Move::Scissors,
        (Move::Rock, RoundResult::Draw) => Move::Rock,
        (Move::Rock, RoundResult::Win) => Move::Paper,
        (Move::Paper, RoundResult::Lose) => Move::Rock,
        (Move::Paper, RoundResult::Draw) => Move::Paper,
        (Move::Paper, RoundResult::Win) => Move::Scissors,
        (Move::Scissors, RoundResult::Lose) => Move::Paper,
        (Move::Scissors, RoundResult::Draw) => Move::Scissors,
        (Move::Scissors, RoundResult::Win) => Move::Rock,
    }
}

struct Part2Round {
    opp_move: Move,
    result: RoundResult,
}

impl Part2Round {
    fn score(&self) -> u8 {
        let my_move = move_to_play(&self.opp_move, &self.result);
        self.result.score() + my_move.score()
    }
}

fn part2_rounds(input: &str) -> impl Iterator<Item = Part2Round> + '_ {
    input.lines().map(|line| {
        let bytes = line.as_bytes();
        Part2Round {
            opp_move: Move::from_letter(bytes[0]),
            result: RoundResult::from_letter(bytes[2]),
        }
    })
}

fn part2(input: &str) -> u32 {
    part2_rounds(input).map(|r| r.score() as u32).sum()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 15;
    in => 13446;
fn part2:
    TEST_INPUT => 12;
    in => 13509;
}

aoc::main!(part1, part2);
