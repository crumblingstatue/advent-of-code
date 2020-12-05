#[derive(PartialEq, Eq, Debug, Ord, PartialOrd)]
struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn id(&self) -> u16 {
        self.row as u16 * 8 + self.column as u16
    }
}

fn resolve_seat(command_stream: &[u8]) -> Seat {
    let mut row_lower = 0;
    let mut row_upper = 127;
    let mut col_lower = 0;
    let mut col_upper = 7;
    for &c in command_stream {
        match c {
            b'F' => row_upper = median(row_lower, row_upper),
            b'B' => row_lower = median(row_lower, row_upper) + 1,
            b'L' => col_upper = median(col_lower, col_upper),
            b'R' => col_lower = median(col_lower, col_upper) + 1,
            _ => panic!("Invalid command: {}", c),
        }
    }
    assert_eq!(row_lower, row_upper);
    assert_eq!(col_lower, col_upper);
    Seat {
        row: row_lower,
        column: col_lower,
    }
}

fn median(lower: u8, upper: u8) -> u8 {
    assert!(upper >= lower);
    lower + ((upper - lower) / 2)
}

fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|l| resolve_seat(l.as_bytes()))
        .max_by_key(|s| s.id())
        .unwrap()
        .id()
}

fn part2(input: &str) -> u16 {
    let mut seats: Vec<Seat> = input.lines().map(|l| resolve_seat(l.as_bytes())).collect();
    seats.sort_unstable();
    // Find a gap in the seats
    let mut row_counter = 1;
    let mut col_counter = 0;
    for seat in seats.into_iter().skip_while(|seat| seat.row == 0) {
        if !(seat.row == row_counter && seat.column == col_counter) {
            return Seat {
                row: row_counter,
                column: col_counter,
            }
            .id();
        }
        col_counter += 1;
        if col_counter > 7 {
            row_counter += 1;
            col_counter = 0;
        }
    }
    panic!("Couldn't find any gaps!");
}

#[test]
fn test_resolve_seat() {
    assert_eq!(resolve_seat(b"FBFBBFFRLR"), Seat { row: 44, column: 5 });
    assert_eq!(resolve_seat(b"BFFFBBFRRR"), Seat { row: 70, column: 7 });
    assert_eq!(resolve_seat(b"FFFBBBFRRR"), Seat { row: 14, column: 7 });
    assert_eq!(
        resolve_seat(b"BBFFBBFRLL"),
        Seat {
            row: 102,
            column: 4
        }
    );
}

aoc::tests! {
    fn part1:
    => 894
    fn part2:
    => 579
}

aoc::main!(part1, part2);
