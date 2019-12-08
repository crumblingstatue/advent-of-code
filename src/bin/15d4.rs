fn part1(input: &str) -> i32 {
    let mut num = 0;
    loop {
        let hash_input = format!("{}{}", input, num);
        let digest = md5::compute(hash_input.as_bytes());
        let digest_hex = format!("{:x}", digest);
        if digest_hex.starts_with("00000") {
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
        let digest_hex = format!("{:x}", digest);
        if digest_hex.starts_with("000000") {
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
