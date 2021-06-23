use std::collections::HashMap;

fn part1(input: &str) -> u16 {
    let circ = assemble_circuit(input, None);
    match circ["a"] {
        WireOutput::Known(what) => what,
        WireOutput::Unknown => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum WireOutput {
    Known(u16),
    Unknown,
}

type Wires<'a> = HashMap<&'a str, WireOutput>;
type WireId<'a> = &'a str;

#[derive(Debug)]
enum UnaryExpr<'a> {
    Num(u16),
    Wire(WireId<'a>),
}

#[derive(Debug)]
struct BinOp<'a> {
    lhs: UnaryExpr<'a>,
    rhs: UnaryExpr<'a>,
    op: Op,
}
#[derive(Debug)]
enum Op {
    And,
    Lshift,
    Rshift,
    Not,
    Or,
}

#[derive(Debug)]
enum Expr<'a> {
    Unary(UnaryExpr<'a>),
    BinOp(BinOp<'a>),
    Not(UnaryExpr<'a>),
}

#[derive(Debug)]
struct Instruction<'a> {
    src: Expr<'a>,
    dest: WireId<'a>,
}

#[derive(Debug)]
enum Token<'a> {
    Unary(UnaryExpr<'a>),
    Op(Op),
    /// The -> thingy
    Terminator,
}

fn parse_single_token(word: &str) -> Token {
    let first_char = word.chars().next().unwrap();
    if word == "->" {
        Token::Terminator
    } else if first_char.is_digit(10) {
        Token::Unary(UnaryExpr::Num(word.parse().unwrap()))
    } else if first_char.is_uppercase() {
        Token::Op(parse_op(word))
    } else {
        Token::Unary(UnaryExpr::Wire(word))
    }
}

fn parse_op(opname: &str) -> Op {
    match opname {
        "AND" => Op::And,
        "OR" => Op::Or,
        "NOT" => Op::Not,
        "LSHIFT" => Op::Lshift,
        "RSHIFT" => Op::Rshift,
        _ => panic!("Unhandled opname: {}", opname),
    }
}

fn parse_expr<'a>(words: &mut impl Iterator<Item = &'a str>) -> Expr<'a> {
    let fst = parse_single_token(words.next().unwrap());
    match fst {
        Token::Unary(unary) => {
            let next = parse_single_token(words.next().unwrap());
            match next {
                Token::Terminator => Expr::Unary(unary),
                Token::Op(op) => {
                    let rhs = parse_single_token(words.next().unwrap());
                    // "Eat up the terminator token"
                    parse_single_token(words.next().unwrap());
                    match rhs {
                        Token::Unary(right_unary) => Expr::BinOp(BinOp {
                            lhs: unary,
                            op,
                            rhs: right_unary,
                        }),
                        _ => panic!("Invalid right hand side: {:?}", rhs),
                    }
                }
                Token::Unary(expr) => {
                    panic!("Unexpected unary expression after another one: {:?}", expr)
                }
            }
        }
        Token::Op(op) => {
            match op {
                Op::Not => {
                    let next = parse_single_token(words.next().unwrap());
                    // "Eat up" the terminator token
                    parse_single_token(words.next().unwrap());
                    match next {
                        Token::Unary(unary) => Expr::Not(unary),
                        _ => panic!("NOT needs a unary expression following it"),
                    }
                }
                _ => panic!("Only NOT is supported as a unary expression."),
            }
        }
        Token::Terminator => panic!("Empty expression!"),
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut words = line.split_whitespace();
    let src = parse_expr(&mut words);
    let dest = words.next().unwrap();
    Instruction { src, dest }
}

fn unary_value(expr: UnaryExpr, wires: &Wires) -> WireOutput {
    match expr {
        UnaryExpr::Num(n) => WireOutput::Known(n),
        UnaryExpr::Wire(w) => *wires.get(w).unwrap_or(&WireOutput::Unknown),
    }
}

fn get_val(src: Expr, wires: &Wires) -> WireOutput {
    match src {
        Expr::BinOp(BinOp { lhs, rhs, op }) => {
            let lhs = match unary_value(lhs, wires) {
                WireOutput::Known(val) => val,
                WireOutput::Unknown => return WireOutput::Unknown,
            };
            let rhs = match unary_value(rhs, wires) {
                WireOutput::Known(val) => val,
                WireOutput::Unknown => return WireOutput::Unknown,
            };
            let result = match op {
                Op::And => lhs & rhs,
                Op::Or => lhs | rhs,
                Op::Not => panic!("NOT operator in binary context"),
                Op::Lshift => lhs << rhs,
                Op::Rshift => lhs >> rhs,
            };
            WireOutput::Known(result)
        }
        Expr::Not(what) => {
            let what = match unary_value(what, wires) {
                WireOutput::Known(val) => val,
                WireOutput::Unknown => return WireOutput::Unknown,
            };
            WireOutput::Known(!what)
        }
        Expr::Unary(what) => unary_value(what, wires),
    }
}

fn assemble_circuit<'a>(
    input: &'a str,
    override_: Option<(WireId<'static>, WireOutput)>,
) -> Wires<'a> {
    let mut wires: Wires = Wires::default();
    if let Some((k, v)) = &override_ {
        wires.insert(k, *v);
    }
    loop {
        for Instruction { src, dest } in input.lines().map(parse_instruction) {
            let val = get_val(src, &wires);
            if let Some((wire_id, _)) = &override_ {
                if wire_id == &dest {
                    continue;
                }
            }
            wires.insert(dest, val);
        }
        if !wires.values().any(|v| *v == WireOutput::Unknown) {
            return wires;
        }
    }
}

fn part2(input: &str) -> u16 {
    let signal = part1(input);
    let circ = assemble_circuit(input, Some(("b", WireOutput::Known(signal))));
    match circ["a"] {
        WireOutput::Known(v) => v,
        WireOutput::Unknown => unreachable!(),
    }
}

#[test]
fn test_assemble_circuit() {
    const TESTCIRCUIT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
    const TEDDY_TEST: &str = "x AND y -> z
    15 -> x
    24 -> y";
    let wires = assemble_circuit(TESTCIRCUIT, None);
    assert_eq!(wires["d"], WireOutput::Known(72));
    let wires = assemble_circuit(TEDDY_TEST, None);
    assert_eq!(wires["z"], WireOutput::Known(8));
}

aoc::tests! {
    fn part1:
    => 3176
    fn part2:
    => 14710
}

aoc::main!(part1, part2);
