use std::collections::LinkedList;
use regex::Regex;

#[derive(Debug)]
pub struct Passport {
    lines : String
}

pub fn validate_passports(passports : LinkedList<Passport>) -> LinkedList<Passport> {
    let mut valid_passports : LinkedList<Passport> = LinkedList::new();

    for passport in passports {
        if check_for_validness(passport.lines.to_string()) {
            valid_passports.push_back(passport);
        }
    }

    valid_passports
}

pub fn check_for_validness(data : String) -> bool {
    let field_check : Vec<&str> = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    for field in field_check {
        if !data.contains(field) {
            return false
        }
    }
    true
}

pub fn check_field_validness(passports : LinkedList<Passport>) -> i32 {
    let mut valid_passports : i32 = 0;

    for passport in passports {
        let fields = passport.lines.trim_start().trim_end().split(" ");
        let mut validness : bool = true;
        for field in fields {
            let field_vec : Vec<&str> = field.split(":").collect();
            let check = "hcl";
            if field_vec[0] != "cid" && !is_field_valid(field_vec[0], field_vec[1]) {
                validness = false;
                break;
            }
        }
        if validness {
            valid_passports += 1;
        } else {
            print_passport(passport);
        }
    }
    
    valid_passports
}

pub fn print_passport(passport : Passport) {
    println!("{}", passport.lines);
}

pub fn is_field_valid(field : &str, data : &str) -> bool {
    match field {
        "byr" => check_byr(data),
        "iyr" => check_iyr(data),
        "eyr" => check_eyr(data),
        "hgt" => check_hgt(data),
        "hcl" => check_hcl(data),
        "ecl" => check_ecl(data),
        "pid" => check_pid(data),
        _ => false
    }
}

pub fn check_byr(data : &str) -> bool {
    let year : i32 = match data.parse::<i32>() {
        Ok(value) => value,
        Err(_value) => -1
    }; 
    year >= 1920 && year <= 2002
}

pub fn check_iyr(data : &str) -> bool {
    let year : i32 = match data.parse::<i32>() {
        Ok(value) => value,
        Err(_value) => -1
    };
    year >= 2010 && year <= 2020
}

pub fn check_eyr(data : &str) -> bool {
    let year : i32 = match data.parse::<i32>() {
        Ok(value) => value,
        Err(_value) => -1
    };
    year >= 2020 && year <= 2030
}

pub fn check_hgt(data : &str) -> bool {
    if data.contains("in") {
        let len = data.len();
        let numdata  = match String::from(data)[..(len-2)].parse::<i32>() {
            Ok(value) => value,
            Err(_value) => -1
        };
        numdata >= 59 && numdata <= 76

    } else if data.contains("cm"){
        let len = data.len();
        let numdata = match String::from(data)[..(len-2)].parse::<i32>() {
            Ok(value) => value,
            Err(_value) => -1
        };
        numdata >= 150 && numdata <= 193
    } else {
        false
    }
}

pub fn check_hcl(data : &str) -> bool {
    if &String::from(data)[..1] != "#" {
        return false;
    }

    let validate : &str = &String::from(data)[1..];
    let re = Regex::new(r"([0-9a-f]){6}").unwrap();

    re.is_match(validate)
}

pub fn check_ecl(data : &str) -> bool {
    let valid_colors : Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for color in valid_colors {
        if data.contains(color) {
            return true
        }
    }

    false
}

pub fn check_pid(data : &str) -> bool {
    let re = Regex::new(r"([0-9]){9}").unwrap();

    re.is_match(data)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut passports : LinkedList<Passport> = LinkedList::new();

    let mut passport_data : String = String::new();

    for line in input.lines() {
        if line.is_empty() {
            passports.push_back(Passport {
                lines : passport_data.to_string()
            });
            passport_data = String::new();
        } else {
            passport_data.insert_str(passport_data.len(), " ");
            passport_data.insert_str(passport_data.len(), line);
        }
    }
    passports.push_back(Passport {
        lines : passport_data.to_string()
    });

    validate_passports(passports).len() as i32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut passports : LinkedList<Passport> = LinkedList::new();

    let mut passport_data : String = String::new();

    for line in input.lines() {
        if line.is_empty() {
            passports.push_back(Passport {
                lines : passport_data.to_string()
            });
            passport_data = String::new();
        } else {
            passport_data.insert_str(passport_data.len(), " ");
            passport_data.insert_str(passport_data.len(), line);
        }
    }
    passports.push_back(Passport {
        lines : passport_data.to_string()
    });

    check_field_validness(passports)
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample1() {
        assert_eq!(part1("12"), 2);
        assert_eq!(part1("12\n14"), 4);
        assert_eq!(part1("1969"), 654);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2("1969"), 966);
        assert_eq!(part2("100756"), 50346);
    }
}
