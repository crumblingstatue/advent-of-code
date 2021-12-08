use itertools::Itertools;

struct Display {
    digits: [Digit; 4],
}

struct Digit {
    segments: [bool; 7],
}

impl Digit {}

#[derive(Debug)]
struct SignalNote<'a> {
    patterns: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'a> SignalNote<'a> {
    fn from_str(input: &'a str) -> Self {
        let (patterns, output) = input.split_once(" | ").unwrap();
        Self {
            patterns: patterns.split(' ').collect(),
            output: output.split(' ').collect(),
        }
    }
}

fn notes(input: &str) -> Vec<SignalNote> {
    input.lines().map(SignalNote::from_str).collect_vec()
}

#[cfg(test)]
const TEST_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | \
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | \
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | \
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | \
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | \
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | \
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | \
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | \
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | \
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | \
fgae cfgab fg bagce";

const fn unique_digit_from_len(len: u8) -> Option<u8> {
    Some(match len {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => return None,
    })
}

fn part1(input: &str) -> usize {
    notes(input)
        .iter()
        .flat_map(|note| &note.output)
        .filter(|output| unique_digit_from_len(output.len() as u8).is_some())
        .count()
}

aoc::tests! {
    fn part1:
    TEST_INPUT => 26;
    in => 543;
}

aoc::main!(part1);
