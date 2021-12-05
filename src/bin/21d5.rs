use aoc::array_2d::Array2D;

#[derive(Debug, Clone)]
struct Line2D {
    p1: Point2D,
    p2: Point2D,
}

impl Line2D {
    fn segment_points(&self) -> impl Iterator<Item = Point2D> {
        let min_x = self.p1.x.min(self.p2.x);
        let min_y = self.p1.y.min(self.p2.y);
        let max_x = self.p1.x.max(self.p2.x);
        let max_y = self.p1.y.max(self.p2.y);
        zip_repeat(min_x..=max_x, min_y..=max_y).map(|(x, y)| Point2D { x, y })
    }
    fn aligned(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }
}

fn zip_repeat<It1: Iterator, It2: Iterator>(
    iter1: It1,
    iter2: It2,
) -> impl Iterator<Item = (It1::Item, It2::Item)>
where
    It1::Item: Default + Clone,
    It2::Item: Default + Clone,
{
    struct ZipRepeat<T: Iterator, U: Iterator> {
        iter1: T,
        iter2: U,
        iter1_last: T::Item,
        iter2_last: U::Item,
    }
    impl<T: Iterator, U: Iterator> Iterator for ZipRepeat<T, U>
    where
        T::Item: Clone,
        U::Item: Clone,
    {
        type Item = (T::Item, U::Item);

        fn next(&mut self) -> Option<Self::Item> {
            let (mut exhaust1, mut exhaust2) = (false, false);
            let it1 = match self.iter1.next() {
                Some(item) => {
                    self.iter1_last = item.clone();
                    item
                }
                None => {
                    exhaust1 = true;
                    self.iter1_last.clone()
                }
            };
            let it2 = match self.iter2.next() {
                Some(item) => {
                    self.iter2_last = item.clone();
                    item
                }
                None => {
                    exhaust2 = true;
                    self.iter2_last.clone()
                }
            };
            if exhaust1 && exhaust2 {
                None
            } else {
                Some((it1, it2))
            }
        }
    }
    ZipRepeat {
        iter1,
        iter2,
        iter1_last: Default::default(),
        iter2_last: Default::default(),
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
    => 3990;
}

aoc::main!(part1);
