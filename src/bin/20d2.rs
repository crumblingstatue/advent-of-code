#[derive(PartialEq, Debug)]
struct Policy {
    byte: u8,
    min: u8,
    max: u8,
}

type ByteOccurences = [u8; 256];

fn parse_policy(policy: &str) -> Policy {
    let dash = policy.find('-').unwrap();
    let space = policy.find(' ').unwrap();
    let min: u8 = policy[..dash].parse().unwrap();
    let max: u8 = policy[dash + 1..space].parse().unwrap();
    let byte = policy.as_bytes()[space + 1];
    Policy { byte, min, max }
}

fn validate(line: &str, f: impl Fn(Policy, &str) -> bool) -> bool {
    let mut splits = line.split(':');
    let pol = splits.next().unwrap();
    let pw = &splits.next().unwrap()[1..];
    let pol = parse_policy(pol);
    f(pol, pw)
}

fn valid1(policy: Policy, password: &str) -> bool {
    let ocs = count(password.as_bytes());
    let count = ocs[policy.byte as usize];
    count >= policy.min && count <= policy.max
}

fn valid2(policy: Policy, password: &str) -> bool {
    let bytes = password.as_bytes();
    let mut count = 0;
    if bytes[policy.min as usize - 1] == policy.byte {
        count += 1;
    }
    if bytes[policy.max as usize - 1] == policy.byte {
        count += 1;
    }
    count == 1
}

fn count(bytes: &[u8]) -> ByteOccurences {
    let mut ocs = [0; 256];
    for &b in bytes {
        ocs[b as usize] += 1;
    }
    ocs
}

fn part1(input: &str) -> usize {
    input.lines().filter(|s| validate(s, valid1)).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|s| validate(s, valid2)).count()
}

#[test]
fn test_valid2() {
    assert!(valid2(
        Policy {
            min: 1,
            max: 3,
            byte: b'a'
        },
        "abcde"
    ));
    assert!(!valid2(
        Policy {
            min: 1,
            max: 3,
            byte: b'b'
        },
        "cdefg"
    ));
    assert!(!valid2(
        Policy {
            min: 2,
            max: 9,
            byte: b'c'
        },
        "ccccccccc"
    ));
}

#[test]
fn test_parse_policy() {
    assert_eq!(
        parse_policy("1-3 a"),
        Policy {
            min: 1,
            max: 3,
            byte: b'a'
        }
    );
}

aoc::tests! {
    fn part1:
    "1-3 a: abcde\n\
    1-3 b: cdefg\n\
    2-9 c: ccccccccc" => 2;
    in => 591;
    fn part2:
    in => 335;
}

aoc::main!(part1, part2);
