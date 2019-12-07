const STARTING_FLOOR: i32 = 0;

fn part1(input: &str) -> i32 {
    let mut floor = STARTING_FLOOR;
    for ch in input.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid instruction: {}", ch),
        }
    }
    floor
}

fn part2(input: &str) -> i32 {
    let mut floor = STARTING_FLOOR;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid instruction: {}", ch),
        }
        if floor == -1 {
            return i as i32 + 1;
        }
    }
    panic!("Never reached basement level");
}

#[test]
fn test_part1() {
    assert_eq!(part1("(())"), 0);
    assert_eq!(part1("()()"), 0);
    assert_eq!(part1("((("), 3);
    assert_eq!(part1("(()(()("), 3);
    assert_eq!(part1("))((((("), 3);
    assert_eq!(part1("())"), -1);
    assert_eq!(part1("))("), -1);
    assert_eq!(part1(")))"), -3);
    assert_eq!(part1(")())())"), -3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(")"), 1);
    assert_eq!(part2("()())"), 5);
}

fn main() {
    const INPUT: &str = include_str!("15d1.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}
