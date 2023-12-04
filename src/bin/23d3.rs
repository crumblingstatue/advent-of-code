use aoc::array_2d::Array2D;

#[cfg(test)]
const TEST_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[derive(Debug)]
struct SchematicAnalysis {
    parts: Vec<Part>,
    numbers: Vec<Number>,
}

enum ParseStatus {
    Init,
    InNum,
}

impl SchematicAnalysis {
    fn from_array(arr: &Array2D<u8>) -> Self {
        let mut status = ParseStatus::Init;
        let mut num_begin_x = 0;
        let mut parts = vec![];
        let mut nums = vec![];
        for (y, row) in arr.rows().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                match status {
                    ParseStatus::Init => match *byte {
                        b'.' => {}
                        b'0'..=b'9' => {
                            num_begin_x = x;
                            status = ParseStatus::InNum;
                        }
                        ch => {
                            parts.push(Part { ch, x, y });
                        }
                    },
                    ParseStatus::InNum => {
                        if !byte.is_ascii_digit() {
                            nums.push(Number {
                                value: std::str::from_utf8(&row[num_begin_x..x])
                                    .unwrap()
                                    .parse()
                                    .unwrap(),
                                x: num_begin_x,
                                y,
                                len: x - num_begin_x,
                            });
                            status = ParseStatus::Init;
                            if *byte != b'.' {
                                parts.push(Part { ch: *byte, x, y });
                            }
                        }
                    }
                }
            }
            // Finish parsing numbers at end of row
            match status {
                ParseStatus::Init => {}
                ParseStatus::InNum => {
                    nums.push(Number {
                        value: std::str::from_utf8(&row[num_begin_x..row.len()])
                            .unwrap()
                            .parse()
                            .unwrap(),
                        x: num_begin_x,
                        y,
                        len: row.len() - num_begin_x,
                    });
                    status = ParseStatus::Init;
                }
            }
        }
        Self {
            parts,
            numbers: nums,
        }
    }
    fn part_numbers(&self) -> impl Iterator<Item = &Number> {
        self.numbers.iter().filter(|num| {
            self.parts
                .iter()
                .any(|part| part.adjacent_to_linespan(num.y, num.x, num.x + num.len))
        })
    }
}

struct Part {
    ch: u8,
    x: usize,
    y: usize,
}
impl Part {
    fn adjacent_to_linespan(&self, y: usize, x_begin: usize, x_end: usize) -> bool {
        (x_begin..x_end).any(|x| points_adjacent(x, y, self.x, self.y))
    }
}

fn points_adjacent(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    x1.abs_diff(x2) <= 1 && y1.abs_diff(y2) <= 1
}

impl std::fmt::Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Part")
            .field("ch", &(self.ch as char))
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    x: usize,
    y: usize,
    len: usize,
}

fn part1(input: &str) -> u32 {
    let arr = Array2D::from_ascii_lines(input.as_bytes());
    let analysis = SchematicAnalysis::from_array(&arr);
    analysis.part_numbers().map(|pn| pn.value).sum()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 4361;
    in => 525119;
}

aoc::main!(part1);
