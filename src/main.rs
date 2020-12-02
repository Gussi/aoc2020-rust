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
}

