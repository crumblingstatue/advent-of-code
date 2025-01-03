use md5::Digest;

fn zerohash(input: &str, zerocheck: impl Fn(&Digest) -> bool) -> i32 {
    let mut num = 0;
    let orig_len = input.len();
    let mut hash_input = input.as_bytes().to_owned();
    let mut buf = itoa::Buffer::new();
    loop {
        let num_str = buf.format(num);
        hash_input.extend_from_slice(num_str.as_bytes());
        let digest = md5::compute(&hash_input);
        hash_input.truncate(orig_len);
        if zerocheck(&digest) {
            return num;
        }
        num += 1;
    }
}

fn part1(input: &str) -> i32 {
    zerohash(input, |digest| {
        digest[0] == 0 && digest[1] == 0 && digest[2] < 16
    })
}

fn part2(input: &str) -> i32 {
    zerohash(input, |digest| {
        digest[0] == 0 && digest[1] == 0 && digest[2] == 0
    })
}

aoc::tests! {
    fn part1:
    "abcdef" => 609043;
    "pqrstuv" => 1048970;
    in => 117946;
    fn part2:
    in => 3938038;
}

aoc::main!(part1, part2);
