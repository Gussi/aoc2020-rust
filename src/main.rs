use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of Code 2020 solver")
        .version("2020.12.01") // 🤡
        .author("Gunnsteinn Þórisson <gussi@gussi.is>")
        .about("Solves advent of code problems")
        .arg(Arg::with_name("DAY")
             .help("Day to solve")
             .required(true)
             .index(1))
        .arg(Arg::with_name("PART")
             .help("Part to solve")
             .required(true)
             .index(2))
        .get_matches();

    let day: i32 = matches.value_of("DAY")
        .unwrap()
        .parse()
        .unwrap();

    let part: i32 = matches.value_of("PART")
        .unwrap()
        .parse()
        .unwrap();

    match day {
        1 => {
            match part {
                1 => println!("part 1 answer: {}", aoc2020::day1::part1()),
                2 => println!("part 2 answer: {}", aoc2020::day1::part2()),
                _ => println!("Part {} not found!", part),
            }
        },
        2 => {
            match part {
                1 => println!("part 1 answer: {}", aoc2020::day2::part1().unwrap()),
                2 => println!("part 2 answer: {}", aoc2020::day2::part2().unwrap()),
                _ => println!("Part {} not found!", part),
            }
        },
        _ => println!("Day {} not found!", day),
    }

}

mod aoc2020 {
    pub mod day1 {
        use std::collections::HashSet;

        pub fn part1() -> i32 {
            let input = input();

            for x in &input {
                let y = 2020 - x;

                if input.contains(&y) {
                    return x * y;
                }
            }

            0
        }

        pub fn part2() -> i32 {
            let input = input();

            for x in &input {
                for y in &input {
                    let z = 2020 - (x + y);

                    if input.contains(&z) {
                        return x * y * z;
                    }
                }
            }

            0
        }

        fn input() -> HashSet<i32> {
            let mut set = HashSet::new();

            loop {
                let mut input = String::new();

                std::io::stdin()
                    .read_line(&mut input)
                    .unwrap();

                let n = input
                    .trim()
                    .parse::<i32>();

                let t = match n {
                    Ok(v) => v,
                    Err(_e) => 0,
                };

                if t == 0 {
                    break;
                }

                set.insert(t);
            }

            set
        }
    }

    pub mod day2 {
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
    }
}

