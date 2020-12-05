use clap::{Arg, App};

mod aoc2020;

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
                1 => println!("part 1 answer: {}", aoc2020::day1::part1().unwrap()),
                2 => println!("part 2 answer: {}", aoc2020::day1::part2().unwrap()),
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
        3 => {
            match part {
                1 => println!("part 1 answer: {}", aoc2020::day3::part1().unwrap()),
                2 => println!("part 2 answer: {}", aoc2020::day3::part2().unwrap()),
                _ => println!("Part {} not found!", part),
            }
        },
        _ => println!("Day {} not found!", day),
    }

}
