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
