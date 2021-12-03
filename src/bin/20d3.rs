struct RightRepMap {
    buf: Vec<u8>,
    width: usize,
    height: usize,
}

impl RightRepMap {
    fn from_str(input: &str) -> Self {
        let mut buf = Vec::new();
        let mut y = 0;
        let mut width = 0;
        for line in input.lines() {
            let mut w = 0;
            for b in line.bytes() {
                buf.push(b);
                w += 1;
            }
            width = w;
            y += 1;
        }
        Self {
            buf,
            width,
            height: y,
        }
    }
    fn at(&self, x: usize, y: usize) -> u8 {
        self.buf[y * self.width + (x % self.width)]
    }
}

fn traverse(map: &RightRepMap, right: usize, down: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    loop {
        x += right;
        y += down;
        if y >= map.height {
            break;
        }
        let thing = map.at(x, y);
        if thing == b'#' {
            trees += 1;
        }
    }
    trees
}

fn part1(input: &str) -> u32 {
    let map = RightRepMap::from_str(input);
    traverse(&map, 3, 1)
}

fn part2(input: &str) -> u64 {
    let map = RightRepMap::from_str(input);
    traverse(&map, 1, 1) as u64
        * traverse(&map, 3, 1) as u64
        * traverse(&map, 5, 1) as u64
        * traverse(&map, 7, 1) as u64
        * traverse(&map, 1, 2) as u64
}

#[cfg(test)]
fn repmap_to_string(repmap: &RightRepMap) -> String {
    let mut vec = Vec::new();
    for y in 0..repmap.width {
        for x in 0..repmap.height {
            vec.push(repmap.at(x, y));
        }
        vec.push(b'\n');
    }
    // Remove last newline
    vec.pop();
    String::from_utf8(vec).unwrap()
}

#[cfg(test)]
const TEST_MAP: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

#[test]
fn test_repmap_from_str() {
    assert_eq!(
        &repmap_to_string(&RightRepMap::from_str(TEST_MAP)),
        TEST_MAP
    )
}

aoc::tests! {
    fn part1:
    TEST_MAP => 7;
    => 274;
    fn part2:
    TEST_MAP => 336;
    => 6050183040;
}

aoc::main!(part1, part2);
