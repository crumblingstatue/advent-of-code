fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let nums = parse(input);
    let (a, b) = find_sum_operands_2(&nums, 2020);
    a * b
}

fn part2(input: &str) -> u32 {
    let nums = parse(input);
    let (a, b, c) = find_sum_operands_3(&nums, 2020);
    a * b * c
}

fn find_sum_operands_2(nums: &[u32], sum: u32) -> (u32, u32) {
    for &n1 in nums {
        for &n2 in nums {
            if n1 + n2 == sum {
                return (n1, n2);
            }
        }
    }
    panic!("Could not find sum operands");
}

fn find_sum_operands_3(nums: &[u32], sum: u32) -> (u32, u32, u32) {
    for &n1 in nums {
        for &n2 in nums {
            for &n3 in nums {
                if n1 + n2 + n3 == sum {
                    return (n1, n2, n3);
                }
            }
        }
    }
    panic!("Could not find sum operands");
}

#[cfg(test)]
const TEST_INPUT: &str = "1721
979
366
299
675
1456";

#[cfg(test)]
const TEST_NUMS: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

#[test]
fn test_find_sum_operands_3() {
    assert_eq!(find_sum_operands_3(&TEST_NUMS, 2020), (979, 366, 675));
}

#[test]
fn test_find_sum_operands_2() {
    assert_eq!(find_sum_operands_2(&TEST_NUMS, 2020), (1721, 299));
}

aoc::tests! {
    fn part1:
    TEST_INPUT => 514579;
    => 974304;
    fn part2:
    => 236430480;
}

aoc::main!(part1, part2);
