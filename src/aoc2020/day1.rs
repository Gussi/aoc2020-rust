use std::collections::HashSet;

pub fn part1() -> Result<i32, &'static str> {
    let input = input().expect("Unable to get input");

    for x in &input {
        let y = 2020 - x;

        if input.contains(&y) {
            return Ok(x * y);
        }
    }

    Err("Didn't find the answer")
}

pub fn part2() -> Result<i32, &'static str> {
    let input = input().expect("Unable to get input");

    for x in &input {
        for y in &input {
            let z = 2020 - (x + y);

            if input.contains(&z) {
                return Ok(x * y * z);
            }
        }
    }

    Err("Didn't find the answer")
}

fn input() -> Result<HashSet<i32>, &'static str> {
    let mut set = HashSet::new();

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Unable to read line from stdin");

        if input.is_empty() {
            break;
        }

        let n: i32 = input.trim().parse().expect("Unable to cast input line to i32");

        set.insert(n);
    }

    Ok(set)
}
