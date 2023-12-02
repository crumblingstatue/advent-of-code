#[cfg(test)]
const TEST_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[cfg(test)]
const TEST_INPUT_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

fn calib_val(text: &[u8]) -> u32 {
    let first = *text.iter().find(|b| b.is_ascii_digit()).unwrap();
    let last = *text.iter().rev().find(|b| b.is_ascii_digit()).unwrap();
    let first = first - b'0';
    let last = last - b'0';
    (first * 10 + last) as u32
}

fn calib_val_2(text: &str) -> u32 {
    let textnums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut i = 0;
    let first_textnum = textnums
        .map(|txtnum| {
            i += 1;
            text.find(txtnum).map(|pos| (pos, i))
        })
        .into_iter()
        .flatten()
        .min();
    i = 0;
    let last_textnum = textnums
        .map(|txtnum| {
            i += 1;
            text.rfind(txtnum).map(|pos| (pos, i))
        })
        .into_iter()
        .flatten()
        .max();
    let first_digit = text.find(|c: char| c.is_ascii_digit());
    let last_digit = text.rfind(|c: char| c.is_ascii_digit());
    let first = match (first_textnum, first_digit) {
        (None, None) => panic!("No first digit found"),
        (None, Some(pos)) => text.as_bytes()[pos] - b'0',
        (Some(textnum), None) => textnum.1,
        (Some(textnum), Some(dig_pos)) => {
            if textnum.0 < dig_pos {
                textnum.1
            } else {
                text.as_bytes()[dig_pos] - b'0'
            }
        }
    };
    let last = match (last_textnum, last_digit) {
        (None, None) => panic!("No first digit found"),
        (None, Some(pos)) => text.as_bytes()[pos] - b'0',
        (Some(textnum), None) => textnum.1,
        (Some(textnum), Some(dig_pos)) => {
            if textnum.0 > dig_pos {
                textnum.1
            } else {
                text.as_bytes()[dig_pos] - b'0'
            }
        }
    };
    (first * 10 + last) as u32
}

fn part1(input: &str) -> u32 {
    input.lines().map(|l| calib_val(l.as_bytes())).sum()
}

fn part2(input: &str) -> u32 {
    input.lines().map(calib_val_2).sum()
}

aoc::tests! {
fn calib_val:
    b"1abc2" => 12;
fn calib_val_2:
    "two1nine" => 29;
    "eightwothree" => 83;
    "abcone2threexyz" => 13;
    "xtwone3four" => 24;
    "4nineeightseven2" => 42;
    "zoneight234" => 14;
    "7pqrstsixteen" => 76;
fn part1:
    TEST_INPUT => 142;
    in => 52974;
fn part2:
    TEST_INPUT_2 => 281;
    in => 53340;
}

aoc::main!(part1, part2);
