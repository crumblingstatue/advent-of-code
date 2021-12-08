use std::cmp::Ordering::{Equal, Greater, Less};

use aoc::array_2d::Array2D;

#[derive(Debug, Clone)]
struct Line2D {
    p1: Point2D,
    p2: Point2D,
}

impl Line2D {
    fn segment_points(&self) -> impl Iterator<Item = Point2D> {
        // Example:
        // 8,0 -> 0,8 would yield:
        // 8,0
        // 7,1
        // 6,2
        // 5,3
        // 4,4
        // 3,5
        // 2,6
        // 1,7
        // 0,8

        let mut points = Vec::new();
        let mut x = self.p1.x;
        let mut y = self.p1.y;
        let mut x_exhaust = false;
        let mut y_exhaust = false;
        loop {
            points.push(Point2D { x, y });
            match x.cmp(&self.p2.x) {
                Less => x += 1,
                Equal => x_exhaust = true,
                Greater => x -= 1,
            }
            match y.cmp(&self.p2.y) {
                Less => y += 1,
                Equal => y_exhaust = true,
                Greater => y -= 1,
            }
            if x_exhaust && y_exhaust {
                break;
            }
        }
        points.into_iter()
    }
    fn aligned(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }
}

#[derive(Debug, Clone)]
struct Point2D {
    x: u32,
    y: u32,
}

impl Line2D {
    fn from_str(input: &str) -> Self {
        let (p1, p2) = input.split_once(" -> ").unwrap();
        Self {
            p1: Point2D::from_str(p1),
            p2: Point2D::from_str(p2),
        }
    }
}

impl Point2D {
    fn from_str(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

fn lines(input: &str) -> Vec<Line2D> {
    input.lines().map(Line2D::from_str).collect()
}

fn line_grid(lines: impl Iterator<Item = Line2D> + Clone) -> Array2D<u32> {
    // Determine biggest positions
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines.clone() {
        max_x = max_x.max(line.p1.x);
        max_x = max_x.max(line.p2.x);
        max_y = max_y.max(line.p1.y);
        max_y = max_y.max(line.p2.y);
    }
    let mut grid = Array2D::new_filled(max_x as usize + 1, max_y as usize + 1, 0);
    // Apply lines
    for line in lines {
        for point in line.segment_points() {
            *grid.get_mut(point.x as usize, point.y as usize) += 1;
        }
    }
    grid
}

fn part1(input: &str) -> usize {
    line_grid(lines(input).iter().filter(|l| l.aligned()).cloned())
        .flat_iter()
        .filter(|&&cell| cell > 1)
        .count()
}

fn part2(input: &str) -> usize {
    line_grid(lines(input).iter().cloned())
        .flat_iter()
        .filter(|&&cell| cell > 1)
        .count()
}

#[cfg(test)]
const TEST_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

aoc::tests! {
    fn part1:
    TEST_INPUT => 5;
    in => 3990;
    fn part2:
    TEST_INPUT => 12;
    in => 21305;
}

aoc::main!(part1, part2);
