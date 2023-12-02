use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    birth_year: &'a str,
    issue_year: &'a str,
    exp_year: &'a str,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    pass_id: &'a str,
    country_id: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn from_str(string: &'a str) -> Option<Self> {
        let mut hm = HashMap::new();
        for pair in string.split_whitespace() {
            let mut kv = pair.split(':');
            let k = kv.next()?;
            let v = kv.next()?;
            hm.insert(k, v);
        }
        Some(Self {
            birth_year: hm.remove("byr")?,
            country_id: hm.remove("cid"),
            exp_year: hm.remove("eyr")?,
            eye_color: hm.remove("ecl")?,
            hair_color: hm.remove("hcl")?,
            height: hm.remove("hgt")?,
            issue_year: hm.remove("iyr")?,
            pass_id: hm.remove("pid")?,
        })
    }
    fn from_str_validate(string: &'a str) -> Option<Self> {
        let pass = Self::from_str(string)?;
        if validate_years(pass.birth_year, pass.issue_year, pass.exp_year)
            && validate_height(pass.height)
            && validate_hair_color(pass.hair_color)
            && validate_eye_color(pass.eye_color)
            && validate_pass_id(pass.pass_id)
        {
            Some(pass)
        } else {
            None
        }
    }
}

fn validate_years(birth: &str, issue: &str, exp: &str) -> bool {
    if birth.len() != 4 || issue.len() != 4 || exp.len() != 4 {
        false
    } else {
        matches!(birth.parse::<u16>(), Ok(num) if (1920..=2002).contains(&num))
            && matches!(issue.parse::<u16>(), Ok(num) if (2010..=2020).contains(&num))
            && matches!(exp.parse::<u16>(), Ok(num) if (2020..=2030).contains(&num))
    }
}

fn validate_height(input: &str) -> bool {
    if let Some(cm_pos) = input.find("cm") {
        let (num, rem) = (&input[..cm_pos], &input[cm_pos..]);
        matches!(num.parse::<u32>(), Ok(num) if (150..=193).contains(&num) && rem.len() == 2)
    } else if let Some(in_pos) = input.find("in") {
        let (num, rem) = (&input[..in_pos], &input[in_pos..]);
        matches!(num.parse::<u32>(), Ok(num) if (59..=76).contains(&num) && rem.len() == 2)
    } else {
        false
    }
}

fn validate_hair_color(input: &str) -> bool {
    let bytes = input.as_bytes();
    bytes.len() == 7
        && bytes[0] == b'#'
        && bytes[1..]
            .iter()
            .all(|&b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate_eye_color(input: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}

fn validate_pass_id(input: &str) -> bool {
    input.len() == 9 && input.parse::<u32>().is_ok()
}

fn part1(input: &str) -> usize {
    input.split("\n\n").flat_map(Passport::from_str).count()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .flat_map(Passport::from_str_validate)
        .count()
}

#[test]
fn test_passport_from_str() {
    assert_eq!(
        Passport::from_str(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm"
        ),
        Some(Passport {
            birth_year: "1937",
            country_id: Some("147"),
            exp_year: "2020",
            eye_color: "gry",
            hair_color: "#fffffd",
            height: "183cm",
            issue_year: "2017",
            pass_id: "860033327",
        })
    );
    assert_eq!(
        Passport::from_str(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929"
        ),
        None
    );
    assert_eq!(
        Passport::from_str(
            "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm"
        ),
        Some(Passport {
            birth_year: "1931",
            country_id: None,
            exp_year: "2024",
            eye_color: "brn",
            hair_color: "#ae17e1",
            height: "179cm",
            issue_year: "2013",
            pass_id: "760753108"
        })
    );
}

aoc::tests! {
    fn part1:
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in" => 2;
in => 228;
    fn part2:
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007" => 0;
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719" => 4;
in => 175;
}

aoc::main!(part1, part2);
