#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|input| input.parse::<i32>().unwrap())
        .collect();

    let mut i = 0;
    let mut j = 1;

    while i < nums.len() {
        while j < nums.len() {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j]
            }
            j += 1;
        }
        i += 1;
        j = i + 1;
    }

    -1 as i32
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|input| input.parse::<i32>().unwrap())
        .collect();

    let mut i = 0;
    let mut j = 1;
    let mut k = 3;

    while i < nums.len() {
        while j < nums.len() {
            while k < nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return nums[i] * nums[j] * nums[k]
                }
                k += 1;
            }
            j += 1;
            k = j + 1;
        }
        i += 1;
        j = i + 1;
        k = j + 1;
    }

    -1 as i32
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
