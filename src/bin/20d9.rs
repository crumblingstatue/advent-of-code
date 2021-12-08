fn first_nonsum(preamble_len: usize, nums: &[u64]) -> u64 {
    for i in preamble_len.. {
        if !sum_of_prev2(i, preamble_len, nums) {
            return nums[i];
        }
    }
    panic!("No nonsum");
}

fn sum_of_prev2(idx: usize, n_prev: usize, nums: &[u64]) -> bool {
    find_sum2(nums[idx], &nums[idx - n_prev..idx])
}

fn find_sum2(sum: u64, nums: &[u64]) -> bool {
    for n in nums {
        for n2 in nums {
            if n != n2 && n + n2 == sum {
                return true;
            }
        }
    }
    false
}

fn parse_nums(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(input: &str) -> u64 {
    let nums = parse_nums(input);
    first_nonsum(25, &nums)
}

fn part2(input: &str) -> u64 {
    let nums = parse_nums(input);
    let sum = first_nonsum(25, &nums);
    let mut contig = find_sum_contig(sum, &nums).to_owned();
    contig.sort_unstable();
    contig.first().unwrap() + contig.last().unwrap()
}

fn find_sum_contig(sum: u64, nums: &[u64]) -> &[u64] {
    let mut first_cursor = 0;
    loop {
        let mut accum = 0;
        let mut added = 0;
        for n in &nums[first_cursor..] {
            accum += n;
            added += 1;
            if accum == sum && added > 1 {
                return &nums[first_cursor..first_cursor + added];
            }
        }
        first_cursor += 1;
    }
}

#[test]
fn test_first_nonsum() {
    let input = parse_nums(TEST_INPUT);
    assert_eq!(first_nonsum(5, &input), 127)
}

#[cfg(test)]
const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

#[test]
fn test_part2_pre5() {
    let nums = parse_nums(TEST_INPUT);
    let sum = first_nonsum(5, &nums);
    let mut contig = find_sum_contig(sum, &nums).to_owned();
    contig.sort_unstable();
    assert_eq!(contig.first().unwrap() + contig.last().unwrap(), 62)
}

aoc::tests! {
    fn part1:
    in => 27911108;
    fn part2:
    in => 4023754;
}

aoc::main!(part1, part2);
