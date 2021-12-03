#[derive(Debug)]
enum InsKind {
    Nop,
    Acc,
    Jmp,
}

impl InsKind {
    fn from_name(name: &str) -> Self {
        match name {
            "acc" => InsKind::Acc,
            "nop" => InsKind::Nop,
            "jmp" => InsKind::Jmp,
            _ => panic!("Unknown ins name"),
        }
    }
}

#[derive(Debug)]
struct Ins {
    kind: InsKind,
    arg: i32,
}

fn parse(input: &str) -> Vec<Ins> {
    input.lines().map(parse_instr).collect()
}

fn parse_instr(input: &str) -> Ins {
    let mut splits = input.split(' ');
    let name = splits.next().unwrap();
    let arg = splits.next().unwrap().parse().unwrap();
    Ins {
        kind: InsKind::from_name(name),
        arg,
    }
}

struct Vm<'a> {
    acc: i32,
    pc: i32,
    instrs: &'a [Ins],
}

impl<'a> Vm<'a> {
    fn new(instrs: &'a [Ins]) -> Self {
        Self {
            instrs,
            acc: 0,
            pc: 0,
        }
    }
    fn cycle(&mut self) {
        let ins = &self.instrs[self.pc as usize];
        self.pc += 1;
        match ins.kind {
            InsKind::Acc => self.acc += ins.arg,
            InsKind::Jmp => {
                self.pc -= 1;
                self.pc += ins.arg
            }
            InsKind::Nop => {}
        }
    }
}

enum ExecResult {
    Terminate,
    InfiLoop,
}

fn exec(vm: &mut Vm) -> ExecResult {
    let mut visited = vec![false; vm.instrs.len()];
    loop {
        if visited[vm.pc as usize] {
            return ExecResult::InfiLoop;
        }
        visited[vm.pc as usize] = true;
        let last_ins = vm.pc as usize == vm.instrs.len() - 1;
        vm.cycle();
        if last_ins {
            return ExecResult::Terminate;
        }
    }
}

fn part1(input: &str) -> i32 {
    let instrs = parse(input);
    let mut vm = Vm::new(&instrs);
    match exec(&mut vm) {
        ExecResult::InfiLoop => vm.acc,
        _ => unreachable!(),
    }
}

fn swap(ins: &mut Ins) {
    ins.kind = match ins.kind {
        InsKind::Jmp => InsKind::Nop,
        InsKind::Nop => InsKind::Jmp,
        InsKind::Acc => InsKind::Acc,
    }
}

fn part2(input: &str) -> i32 {
    let mut instrs = parse(input);
    let mut swap_idx = 0;
    loop {
        swap(&mut instrs[swap_idx]);
        let mut vm = Vm::new(&instrs);
        match exec(&mut vm) {
            ExecResult::InfiLoop => {}
            ExecResult::Terminate => return vm.acc,
        }
        swap(&mut instrs[swap_idx]);
        swap_idx += 1;
    }
}

#[cfg(test)]
const TEST_INSTRS: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

aoc::tests! {
    fn part1:
    TEST_INSTRS => 5;
    => 1859;
    fn part2:
    TEST_INSTRS => 8;
}

aoc::main!(part1, part2);
