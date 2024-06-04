use advent::prelude::*;
use crate::PassportField::{Byr, Cid, Ecl, Eyr, Hcl, Hgt, Iyr, Pid};

#[derive(Clone, Debug)]
enum PassportField {
    Byr { info: String },
    Iyr { info: String },
    Eyr { info: String },
    Hgt { info: String },
    Hcl { info: String },
    Ecl { info: String },
    Pid { info: String },
    Cid,
}

impl PassportField {

    fn new(field: &str, info: &str) -> PassportField {
        let info = String::from(info);
        match field {
            "byr" => Byr { info },
            "iyr" => Iyr { info },
            "eyr" => Eyr { info },
            "hgt" => Hgt { info },
            "hcl" => Hcl { info },
            "ecl" => Ecl { info },
            "pid" => Pid { info },
            _ => Cid,
        }
    }
    fn is_valid(&self) -> bool {
        match self {
            Byr { info } => {
                (1920usize..=2002).contains(&info.parse::<usize>().unwrap())
            }
            Iyr { info } => {
                (2010usize..=2020).contains(&info.parse::<usize>().unwrap())
            }
            Eyr { info } => {
                (2020usize..=2030).contains(&info.parse::<usize>().unwrap())
            }
            Hgt { info } => {
                info[..info.len() - 2]
                    .parse::<usize>()
                    .map_or(false, |amt| {
                        if &info[info.len() - 2..] == "cm" {
                            (150..=193).contains(&amt)
                        } else {
                            (59..=76).contains(&amt)
                        }
                    })
            }
            Hcl { info } => {
                let rx = regex!(r"#[a-f\d]{6}");
                rx.is_match(info)
            }
            Ecl { info } => {
                let rx = regex!(r"amb|blu|brn|gry|grn|hzl|oth");
                rx.is_match(info)
            }
            Pid { info } => {
                info.len() == 9 && info.as_bytes().iter().all(|c| c.is_ascii_digit())
            }
            Cid => { true }
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<PassportField>> {
    let rx = regex!(r"([a-z]{3}):([^ \r\n]+)");
    input
        .split("\n\n")
        .map(|passport| {
            rx.captures_iter(passport)
                .map(|cap| {
                    let field = &cap[1];
                    let info = &cap[2];
                    PassportField::new(field, info)
                })
                .filter(|field| {
                    if let &Cid = field { false } else { true }
                })
                .collect()
        })
        .filter(|passport: &Vec<_>| passport.len() == 7)
        .collect()
}

fn default_input() -> Vec<Vec<PassportField>> {
    parse_input(include_input!(2020 / 04))
}

fn part1(passports: Vec<Vec<PassportField>>) -> usize {
    passports.len()
}

fn part2(passports: Vec<Vec<PassportField>>) -> usize {
    passports.into_iter()
        .filter(|passport| {
            passport.iter().all(|field| field.is_valid())
        })
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#,
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 2);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#,
    );
    assert_eq!(part1(input.clone()), 4);
    assert_eq!(part2(input), 0);
}

#[test]
fn example3() {
    let input = parse_input(
        r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#,
    );
    assert_eq!(part1(input.clone()), 4);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 242);
    assert_eq!(part2(input), 186);
}
