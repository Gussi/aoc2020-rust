use std::vec::Vec;
use regex::Regex;

struct PasswordPolicy {
    a: usize,
    b: usize,
    chr: char,
    passwd: String,
}

impl PasswordPolicy {
    fn valid_first_rule(&self) -> bool {
        let count = self.passwd
            .matches(self.chr)
            .count();

        let min = self.a;
        let max = self.b;

        min <= count && count <= max
    }

    fn valid_second_rule(&self) -> bool {
        let mut passwd = self.passwd.chars();

        let a = passwd.nth(self.a) == Some(self.chr);
        let b = passwd.nth(self.b) == Some(self.chr);

        a ^ b
    }
}

pub fn part1() -> Result<i32, &'static str> {
    let mut valid = 0;

    let input = input();

    for policy in &input {
        if policy.valid_first_rule() {
            valid += 1;
        }
    }

    Ok(valid)
}

pub fn part2() -> Result<i32, &'static str> {
    let mut valid = 0;

    let input = input();

    for policy in &input {
        if policy.valid_second_rule() {
            valid += 1;
        }
    }

    Ok(valid)
}

fn input() -> Vec<PasswordPolicy> {
    let mut vec: Vec<PasswordPolicy> = Vec::new();

    let re = Regex::new(r"(\d+)-(\d+) (\w): (.+)").unwrap();

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        if input.trim().is_empty() {
            break;
        }

        let caps = re.captures(&input).unwrap();

        let policy = PasswordPolicy {
            a       : caps.get(1).unwrap().as_str().parse().unwrap(),
            b       : caps.get(2).unwrap().as_str().parse().unwrap(),
            chr     : caps.get(3).unwrap().as_str().parse().unwrap(),
            passwd  : caps.get(4).unwrap().as_str().parse().unwrap(),
        };

        vec.push(policy);
    }

    vec
}
