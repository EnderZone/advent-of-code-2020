#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut largest_seat : i32 = 0;
    for line in input.lines() {
        let mut seat = line.chars();
        let mut seat_row : i32 = 0;
        let mut seat_col : i32 = 0;
        for i in 0..7 {
            let code = seat.nth(0).unwrap();
            match code {
                'F' => seat_row += 0,
                'B' => seat_row += (2 as i32).pow(6 - i),
                _ => println!("???")
            }
        }
        for i in 0..3 {
            let code = seat.nth(0).unwrap();
            match code {
                'L' => seat_col += 0,
                'R' => seat_col += (2 as i32).pow(2 - i),
                _ => println!("???")
            }
        }
        let seat_id = seat_row * 8 + seat_col;
        if seat_id > largest_seat {
            largest_seat = seat_id;
        }
    }

    largest_seat
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut input_line : Vec<&str> = input.lines().collect(); 
    let mut seat_numbers : Vec<i32> = vec![1; input_line.len()];
    let mut line_no : usize = 0;
    for line in input_line {
        let mut seat = line.chars();
        let mut seat_row : i32 = 0;
        let mut seat_col : i32 = 0;
        for i in 0..7 {
            let code = seat.nth(0).unwrap();
            match code {
                'F' => seat_row += 0,
                'B' => seat_row += (2 as i32).pow(6 - i),
                _ => println!("???")
            }
        }
        for i in 0..3 {
            let code = seat.nth(0).unwrap();
            match code {
                'L' => seat_col += 0,
                'R' => seat_col += (2 as i32).pow(2 - i),
                _ => println!("???")
            }
        }
        seat_numbers[line_no] = seat_row * 8 + seat_col;
        line_no += 1;
    }

    seat_numbers.sort();

    let mut check_seat : i32 = seat_numbers[0];
    for vec_index in 1..seat_numbers.len() {
        if seat_numbers[vec_index] == check_seat + 1 {
            check_seat = seat_numbers[vec_index];
        } else {
            return check_seat + 1
        }
    }

    -1
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
