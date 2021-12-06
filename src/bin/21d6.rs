use std::fmt::Debug;

#[derive(PartialEq)]
struct School([u64; 10]);

impl Debug for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..=8 {
            write!(f, "{}:{}, ", i, self.0[i])?;
        }
        Ok(())
    }
}

fn school_str(input: &str) -> School {
    school(
        input
            .trim()
            .split(',')
            .map(|tok| tok.parse::<u64>().unwrap()),
    )
}

fn school(fishes: impl Iterator<Item = u64>) -> School {
    let mut school = [0; 10];
    for fish in fishes {
        school[fish as usize] += 1;
    }
    School(school)
}

fn simulate(school: &mut School, cycles: u32) {
    for _ in 0..cycles {
        sim_cycle(school);
    }
}

fn sim_cycle(School(school): &mut School) {
    let mut school_8_add = 0;
    // Each day, a 0 becomes a 6 and adds a new 8 to the end of the list,
    if school[0] > 0 {
        school[7] += school[0];
        school_8_add = school[0];
        school[0] = 0;
    }
    // while each other number decreases by 1 if it was present at the start of the day.
    for i in 0..=8 {
        school[i] = school[i + 1];
    }
    school[8] += school_8_add;
}

fn count(School(school): &School) -> u64 {
    school.iter().sum()
}

fn part1(input: &str) -> u64 {
    let mut school = school_str(input);
    simulate(&mut school, 80);
    count(&school)
}

#[cfg(test)]
const TEST_INPUT: &str = "3,4,3,1,2";

#[test]
fn cycle_test() {
    let test_days: [&[u64]; 19] = [
        &[3, 4, 3, 1, 2],
        &[2, 3, 2, 0, 1],
        &[1, 2, 1, 6, 0, 8],
        &[0, 1, 0, 5, 6, 7, 8],
        &[6, 0, 6, 4, 5, 6, 7, 8, 8],
        &[5, 6, 5, 3, 4, 5, 6, 7, 7, 8],
        &[4, 5, 4, 2, 3, 4, 5, 6, 6, 7],
        &[3, 4, 3, 1, 2, 3, 4, 5, 5, 6],
        &[2, 3, 2, 0, 1, 2, 3, 4, 4, 5],
        &[1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 8],
        &[0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 7, 8],
        &[6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 7, 8, 8, 8],
        &[5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8],
        &[4, 5, 4, 2, 3, 4, 5, 6, 6, 0, 4, 5, 6, 6, 6, 7, 7, 8, 8],
        &[3, 4, 3, 1, 2, 3, 4, 5, 5, 6, 3, 4, 5, 5, 5, 6, 6, 7, 7, 8],
        &[2, 3, 2, 0, 1, 2, 3, 4, 4, 5, 2, 3, 4, 4, 4, 5, 5, 6, 6, 7],
        &[
            1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 1, 2, 3, 3, 3, 4, 4, 5, 5, 6, 8,
        ],
        &[
            0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8,
        ],
        &[
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ],
    ];
    let mut test_school = school(test_days[0].iter().cloned());
    for (i, day) in test_days.into_iter().enumerate() {
        if i == 0 {
            eprint!("Initial state: ");
        } else {
            eprint!("After {} days: ", i);
        }
        assert_eq!(test_school, school(day.iter().cloned()));
        sim_cycle(&mut test_school);
        eprintln!("[OK]");
    }
}

aoc::tests! {
    fn part1:
    TEST_INPUT => 5934;
    => 374994;
}

aoc::main!(part1);
