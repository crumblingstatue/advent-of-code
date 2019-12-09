fn part1(input: &str) -> i32 {
    let mut num = 0;
    loop {
        let hash_input = format!("{}{}", input, num);
        let digest = md5::compute(hash_input.as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            return num;
        }
        num += 1;
    }
}

fn part2(input: &str) -> i32 {
    let mut num = 0;
    loop {
        let hash_input = format!("{}{}", input, num);
        let digest = md5::compute(hash_input.as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return num;
        }
        num += 1;
    }
}

aoc::tests! {
    test1 for part1:
    "abcdef" = 609043
    "pqrstuv" = 1048970
    [117946]
    test2 for part2:
    [3938038]
}

aoc::main!(1 = part1, 2 = part2);