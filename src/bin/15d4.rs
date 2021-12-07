use std::fmt::Write;

fn part1(input: &str) -> i32 {
    let mut num = 0;
    let mut hash_input = String::new();
    loop {
        let _ = write!(&mut hash_input, "{}{}", input, num);
        let digest = md5::compute(hash_input.as_bytes());
        hash_input.clear();
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            return num;
        }
        num += 1;
    }
}

fn part2(input: &str) -> i32 {
    let mut num = 0;
    let mut hash_input = String::new();
    loop {
        let _ = write!(&mut hash_input, "{}{}", input, num);
        let digest = md5::compute(hash_input.as_bytes());
        hash_input.clear();
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return num;
        }
        num += 1;
    }
}

aoc::tests! {
    fn part1:
    "abcdef" => 609043;
    "pqrstuv" => 1048970;
    => 117946;
    fn part2:
    => 3938038;
}

aoc::main!(part1, part2);
