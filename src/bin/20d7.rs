#[derive(Debug)]
struct BagContent<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct BagRule<'a> {
    color: &'a str,
    contents: Vec<BagContent<'a>>,
}

fn parse_rules(input: &str) -> Vec<BagRule> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_rule)
        .collect()
}

fn parse_rule(input: &str) -> BagRule {
    let mut cursor = input.find("bags contain").unwrap();
    let color = &input[..cursor - 1];
    cursor += 13;
    let mut contents = Vec::new();
    while let Some(comma) = input[cursor..].find(&[',', '.'][..]) {
        if let Some(content) = parse_content(&input[cursor..cursor + comma]) {
            contents.push(content);
        }
        cursor += comma + 1;
    }
    BagRule { color, contents }
}

fn parse_content(mut input: &str) -> Option<BagContent> {
    input = input.trim_start();
    let first_space = input.find(' ').unwrap();
    let n = &input[..first_space];
    if n == "no" {
        return None;
    }
    let amount = n.parse().unwrap();
    let bag = input.find(" bag").unwrap();
    let color = &input[first_space + 1..bag];
    Some(BagContent { color, amount })
}

fn part1(input: &str) -> usize {
    let rules = parse_rules(input);
    let mut count = 0;
    for rule in &rules {
        if rule.color != "shiny gold" && count_color("shiny gold", rule, &rules) > 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let rules = parse_rules(input);
    let rule = rules.iter().find(|r| r.color == "shiny gold").unwrap();
    count_colors(rule, &rules)
}

fn count_color(name: &str, rule: &BagRule, rules: &[BagRule]) -> usize {
    let mut count = 0;
    for content in &rule.contents {
        if content.color == name {
            count += 1;
        } else {
            let rule = rules
                .iter()
                .find(|rule| rule.color == content.color)
                .unwrap();
            count += count_color(name, rule, rules);
        }
    }
    count
}

fn count_colors(rule: &BagRule, rules: &[BagRule]) -> usize {
    let mut count = 0;
    for content in &rule.contents {
        count += content.amount as usize;
        let rule = rules.iter().find(|r| r.color == content.color).unwrap();
        count += content.amount as usize * count_colors(rule, rules);
    }
    count
}

#[cfg(test)]
const TEST_RULES: &str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

aoc::tests! {
    fn part1:
    TEST_RULES => 4;
    => 316;
    fn part2:
    TEST_RULES => 32;
    => 11310;
}

aoc::main!(part1, part2);
