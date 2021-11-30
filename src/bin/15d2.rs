fn parse_lwh(text: &str) -> (i32, i32, i32) {
    let mut splits = text.split('x');
    let l = splits.next().unwrap().parse().unwrap();
    let w = splits.next().unwrap().parse().unwrap();
    let h = splits.next().unwrap().parse().unwrap();
    (l, w, h)
}

fn paper_req_lwh(l: i32, w: i32, h: i32) -> i32 {
    let side1 = l * w;
    let side2 = w * h;
    let side3 = h * l;

    let surface_area = 2 * side1 + 2 * side2 + 2 * side3;
    let extra = [side1, side2, side3].into_iter().min().unwrap();
    surface_area + extra
}

fn paper_req_present_spec(spec: &str) -> i32 {
    let (l, w, h) = parse_lwh(spec);
    paper_req_lwh(l, w, h)
}

fn shortest_distance_around_sides(l: i32, w: i32, h: i32) -> i32 {
    let mut sides = [l, w, h];
    sides.sort_unstable();
    sides[0] * 2 + sides[1] * 2
}

fn ribbon_req_lwh(l: i32, w: i32, h: i32) -> i32 {
    let ribbon_req = shortest_distance_around_sides(l, w, h);
    let volume = l * w * h;
    ribbon_req + volume
}

fn ribbon_req_present_spec(spec: &str) -> i32 {
    let (l, w, h) = parse_lwh(spec);
    ribbon_req_lwh(l, w, h)
}

fn part1(input: &str) -> i32 {
    input.lines().map(paper_req_present_spec).sum()
}

fn part2(input: &str) -> i32 {
    input.lines().map(ribbon_req_present_spec).sum()
}

aoc::tests! {
    fn part1:
    "2x3x4" => 58
    "1x1x10" => 43
    => 1588178
    fn part2:
    "2x3x4" => 34
    "1x1x10" => 14
    => 3783758
}

aoc::main!(part1, part2);
