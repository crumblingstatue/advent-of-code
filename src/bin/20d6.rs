fn is_az(b: &u8) -> bool {
    (b'a'..=b'z').contains(b)
}

type Ans = u8;

//                               Group              Form               Ans
fn group_iter(
    input: &'_ str,
) -> impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = Ans> + '_>> {
    input.split("\n\n").map(form_iter)
}

//                              Form               Ans
fn form_iter(input: &'_ str) -> impl Iterator<Item = impl Iterator<Item = Ans> + '_> {
    input.lines().map(ans_iter)
}

fn ans_iter(input: &'_ str) -> impl Iterator<Item = Ans> + '_ {
    input.bytes().filter(is_az)
}

fn count_unique(iter: impl Iterator<Item = Ans>) -> usize {
    let mut was = [false; 128];
    let mut count = 0;
    for ans in iter {
        let was = &mut was[ans as usize];
        if !(*was) {
            count += 1;
            *was = true;
        }
    }
    count
}

//                 Form               Ans
fn count_all(iter: impl Iterator<Item = impl Iterator<Item = Ans>>) -> usize {
    let mut counts = [0; 128];
    let mut forms = 0;
    for form in iter {
        for ans in form {
            counts[ans as usize] += 1;
        }
        forms += 1;
    }
    counts.iter().filter(|&&count| count == forms).count()
}

fn part1(input: &str) -> usize {
    group_iter(input)
        .map(|group| count_unique(group.flatten()))
        .sum()
}

fn part2(input: &str) -> usize {
    group_iter(input).map(count_all).sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

aoc::tests! {
    fn part1:
    TEST_INPUT => 11;
    in => 6683;
    fn part2:
    TEST_INPUT => 6;
    in => 3122;
}

aoc::main!(part1, part2);
