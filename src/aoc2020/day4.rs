use std::io::Read;
use regex::Regex;

struct Passport {
    byr: Option<String>, // Birth Year
    iyr: Option<String>, // Issue Year
    eyr: Option<String>, // Expiration Year
    hgt: Option<String>, // Height
    hcl: Option<String>, // Hair Color
    ecl: Option<String>, // Eye Color
    pid: Option<String>, // Passport ID
    cid: Option<String>, // Country ID
}

impl Passport {

    pub fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn from_attributes(attributes: String) -> Self {
        let mut passport = Passport::new();

        for attr in attributes.split(' ') {
            let mut key_val = attr.split(':');
            let key = key_val.next().unwrap();
            let val = key_val.next().unwrap().to_string();

            match key {
                "byr"   => passport.byr = Some(val),
                "iyr"   => passport.iyr = Some(val),
                "eyr"   => passport.eyr = Some(val),
                "hgt"   => passport.hgt = Some(val),
                "hcl"   => passport.hcl = Some(val),
                "ecl"   => passport.ecl = Some(val),
                "pid"   => passport.pid = Some(val),
                "cid"   => passport.cid = Some(val),
                _       => panic!("Invalid key"),
            }
        }

        return passport;
    }

    pub fn validate_part_one(&self) -> bool {
        return !(
            self.byr.is_none() ||
            self.iyr.is_none() ||
            self.eyr.is_none() ||
            self.hgt.is_none() ||
            self.hcl.is_none() ||
            self.ecl.is_none() ||
            self.pid.is_none()
        )
    }

    pub fn validate_part_two(&self) -> bool {
        return
            self.validate_part_one() &&
            self.validate_birth_year() &&
            self.validate_issue_year() &&
            self.validate_expiration_year() &&
            self.validate_height() &&
            self.validate_hair_color() &&
            self.validate_eye_color() &&
            self.validate_passport_id()
    }

    fn validate_birth_year(&self) -> bool {
        let byr: usize = self.byr.as_ref().unwrap().parse().unwrap();

        byr >= 1920 && byr <= 2002
    }

    fn validate_issue_year(&self) -> bool {
        let iyr: usize = self.iyr.as_ref().unwrap().parse().unwrap();

        iyr >= 2010 && iyr <= 2020
    }

    fn validate_expiration_year(&self) -> bool {
        let eyr: usize = self.eyr.as_ref().unwrap().parse().unwrap();

        eyr >= 2020 && eyr <= 2030
    }

    fn validate_height(&self) -> bool {
        let hgt = self.hgt.as_ref().unwrap();
        let len = hgt.len() - 2;

        if hgt.ends_with("cm") {
            let centimeters = hgt.get(..len).unwrap().parse::<usize>().unwrap();
            return centimeters >= 150 && centimeters <= 193;
        }

        if hgt.ends_with("in") {
            let inches = hgt.get(..len).unwrap().parse::<usize>().unwrap();
            return inches >= 59 && inches <= 76;
        }

        false
    }

    fn validate_hair_color(&self) -> bool {
        let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
        re.is_match(&self.hcl.as_ref().unwrap())
    }

    fn validate_eye_color(&self) -> bool {
        let valid_colors = [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string()
        ];

        valid_colors.contains(&self.ecl.as_ref().unwrap())
    }

    fn validate_passport_id(&self) -> bool {
        let re = Regex::new("^[0-9]{9}$").unwrap();
        re.is_match(&self.pid.as_ref().unwrap())
    }
}

pub fn part1() -> Result<usize, &'static str> {
    let mut valid = 0;

    for passport in input().unwrap() {
        if passport.validate_part_one() {
            valid += 1;
        }
    }

    Ok(valid)
}

pub fn part2() -> Result<usize, &'static str> {
    let mut valid = 0;

    for passport in input().unwrap() {
        if passport.validate_part_two() {
            valid += 1;
        }
    }

    Ok(valid)
}

fn input() -> Result<Vec<Passport>, &'static str> {
    let mut passports: Vec<Passport> = Vec::new();

    loop {
        let mut buffer = String::new();

        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Unable to read from stdin");

        let passport_attributes: Vec<String> = buffer
            .split("\n\n")
            .map(|attrs| attrs.replace("\n", " "))
            .filter(|attrs| !attrs.is_empty())
            .collect();

        for attributes in passport_attributes {
            passports.push(Passport::from_attributes(attributes));
        }

        break;
    }

    Ok(passports)
}
