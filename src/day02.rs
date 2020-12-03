pub fn is_valid(min: i32, max: i32, check: char, password: &str) -> bool {
    let count : i32 = char_count(check, password);
    max >= count && count >= min
}

pub fn char_count(check: char, password: &str) -> i32 {
    password.matches(check).count() as i32
}

pub fn is_valid_part_two(pos_one: i32, pos_two: i32, check: char, password: &str) -> bool {
    
    let pos_one_check: bool = match password.chars().nth(pos_one as usize - 1) {
        None => false,
        Some(value) => value == check
    };

    let pos_two_check: bool = match password.chars().nth(pos_two as usize - 1) {
        None => false,
        Some(value) => value == check
    };

    (pos_one_check && !pos_two_check) || (!pos_one_check && pos_two_check)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {

    let mut valid_passwords : i32 = 0;
    
    for line in input.lines() {
        let mut line_vec : Vec<&str> = line.split(" ").collect();
        let valid_count : Vec<i32> = line_vec[0].split("-")
                                                .map(|input| input.parse::<i32>().unwrap())
                                                .collect();
        let valid_char : char = line_vec[1].chars().next().unwrap();
        if is_valid(valid_count[0], valid_count[1], valid_char, line_vec[2]) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut valid_passwords : i32 = 0;
    
    for line in input.lines() {
        let mut line_vec : Vec<&str> = line.split(" ").collect();
        let valid_count : Vec<i32> = line_vec[0].split("-")
                                                .map(|input| input.parse::<i32>().unwrap())
                                                .collect();
        let valid_char : char = line_vec[1].chars().next().unwrap();

        if is_valid_part_two(valid_count[0], valid_count[1], valid_char, line_vec[2]) {
            valid_passwords += 1;
        }
    }

    valid_passwords
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
