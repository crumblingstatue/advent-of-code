enum State {
    /// Interpret characters as normal
    Normal,
    /// A backslash escape was detected
    Escape,
    /// \x was detected
    Hex1,
    /// First digit was parsed
    Hex2,
}

fn decode(source: &[u8]) -> Vec<u8> {
    use State::*;
    let mut state = Normal;
    let mut buf = Vec::new();
    let mut d1: u8 = 0;
    let mut d2: u8;
    for &ch in source.iter() {
        match state {
            Normal => {
                match ch {
                    b'\\' => state = Escape,
                    b'"' => {} // Just ignore quotes. What could possibly go wrong?
                    _ => buf.push(ch),
                }
            }
            Escape => match ch {
                c @ b'\\' | c @ b'"' => {
                    buf.push(c);
                    state = Normal;
                }
                b'x' => {
                    state = Hex1;
                }
                _ => panic!("Invalid escape: {}", ch),
            },
            Hex1 => {
                d1 = ch;
                state = Hex2;
            }
            Hex2 => {
                d2 = ch;
                state = Normal;
                // Lol inefficient
                buf.push(u8::from_str_radix(&format!("{}{}", d1 as char, d2 as char), 16).unwrap());
            }
        }
    }
    buf
}

fn encode(source: &[u8]) -> Vec<u8> {
    let mut buf = vec![b'"'];
    for &b in source.iter() {
        if b == b'"' || b == b'\\' {
            buf.push(b'\\');
        }
        buf.push(b);
    }
    // Finally lol
    buf.push(b'"');
    buf
}

fn part1(input: &str) -> usize {
    let mut src_len = 0;
    let mut repr_len = 0;
    for line in input.lines() {
        src_len += line.len();
        let decoded = decode(line.as_bytes());
        repr_len += decoded.len();
    }
    src_len - repr_len
}

fn part2(input: &str) -> usize {
    let mut src_len = 0;
    let mut encoded_len = 0;
    for line in input.lines() {
        src_len += line.len();
        let encoded = encode(line.as_bytes());
        encoded_len += encoded.len();
    }
    encoded_len - src_len
}

aoc::tests! {
    fn decode:
    b"\"\"" => b"";
    b"\"abc\"" => b"abc";
    b"\"aaa\\\"aaa\"" => b"aaa\"aaa";
    b"\\x27" => b"'";
    fn part1:
    r#"
    ""
    "abc"
    "aaa\"aaa"
    "\x27"
    "# => 12;
    in => 1350;
    fn encode:
    br#""""# => br#""\"\"""#;
    br#""abc""# => br#""\"abc\"""#;
    br#""aaa\"aaa""# => br#""\"aaa\\\"aaa\"""#;
    br#""\x27""# => br#""\"\\x27\"""#;
    fn part2:
    in => 2085;
}

aoc::main!(part1, part2);
