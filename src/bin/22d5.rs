#![feature(get_many_mut, iter_array_chunks)]

type Crate = u8;

type CrateStack = Vec<Crate>;

#[derive(Default)]
struct Cargo {
    stacks: Vec<CrateStack>,
}

impl std::fmt::Debug for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (i, stack) in self.stacks.iter().enumerate() {
            writeln!(f, "{i}: {}", std::str::from_utf8(stack).unwrap())?;
        }
        Ok(())
    }
}

impl Cargo {
    fn move_(&mut self, amount: usize, src: usize, dst: usize) {
        let [src, dst] = self.stacks.get_many_mut([src, dst]).unwrap();
        for _ in 0..amount {
            let item = src.pop().unwrap();
            dst.push(item);
        }
    }
    fn move_9001(&mut self, amount: usize, src: usize, dst: usize) {
        let [src, dst] = self.stacks.get_many_mut([src, dst]).unwrap();
        let mut buf = Vec::new();
        for _ in 0..amount {
            let item = src.pop().unwrap();
            buf.insert(0, item);
        }
        dst.extend_from_slice(&buf);
    }
}

#[derive(Default, Debug)]
struct LineOffsets {
    cargo_enumerator: usize,
    first_instruction: usize,
}

fn line_discovery_pass(lines: &[&str]) -> LineOffsets {
    let mut offsets = LineOffsets::default();
    for (i, l) in lines.iter().enumerate() {
        if l.trim_start().starts_with('1') {
            offsets.cargo_enumerator = i;
        } else if l.is_empty() {
            offsets.first_instruction = i + 1;
            return offsets;
        }
    }
    panic!("Line discovery pass didn't find empty line")
}

fn parse_crate_column_offsets(line: &str) -> Vec<usize> {
    line.bytes()
        .enumerate()
        .filter_map(|(i, ch)| ch.is_ascii_digit().then_some(i))
        .collect()
}

fn parse_and_exec_cargo_input<const OVER_9000: bool>(input: &str) -> Cargo {
    let mut cargo = Cargo::default();
    let lines: Vec<&str> = input.lines().collect();
    let line_offsets = line_discovery_pass(&lines);
    let crate_column_offsets = parse_crate_column_offsets(lines[line_offsets.cargo_enumerator]);
    cargo.stacks = vec![Vec::new(); crate_column_offsets.len()];
    for l in &lines[..line_offsets.cargo_enumerator] {
        add_initial_bottom_row(&mut cargo, l, &crate_column_offsets);
    }
    for l in &lines[line_offsets.first_instruction..] {
        do_move::<OVER_9000>(l, &mut cargo);
    }
    cargo
}

fn do_move<const OVER_9000: bool>(l: &str, cargo: &mut Cargo) {
    let [n, src, dst] = l
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .array_chunks()
        .next()
        .unwrap();
    if OVER_9000 {
        cargo.move_9001(n, src - 1, dst - 1);
    } else {
        cargo.move_(n, src - 1, dst - 1);
    }
}

fn add_initial_bottom_row(cargo: &mut Cargo, line: &str, crate_column_offsets: &[usize]) {
    let mut col = 0;
    for (i, b) in line.bytes().enumerate() {
        if i == crate_column_offsets[col] {
            if b != b' ' {
                cargo.stacks[col].insert(0, b);
            } else {
            }
            col += 1;
            if col >= crate_column_offsets.len() {
                break;
            }
        }
    }
}

fn part1(input: &str) -> String {
    let cargo = parse_and_exec_cargo_input::<false>(input);
    cargo
        .stacks
        .into_iter()
        .map(|stack| stack.last().copied().unwrap() as char)
        .collect()
}

fn part2(input: &str) -> String {
    let cargo = parse_and_exec_cargo_input::<true>(input);
    cargo
        .stacks
        .into_iter()
        .map(|stack| stack.last().copied().unwrap() as char)
        .collect()
}

#[cfg(test)]
const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

aoc::tests! {
fn part1:
    TEST_INPUT => "CMZ";
    in => "LBLVVTVLP";
fn part2:
    TEST_INPUT => "MCD";
}

aoc::main!(part1, part2);
