pub fn string_to_matrix(input: &str) -> Vec<Vec<char>> {
    
    let lines : Vec<&str> = input.lines().collect();
    let size_x : usize = lines.len();
    let size_y : usize = lines[0].len();

    let mut slope_map : Vec<Vec<char>> = vec![vec!['1'; size_y]; size_x];

    for rot_x in 0..size_x {
        for rot_y in 0..size_y {
            slope_map[rot_x][rot_y] = lines[rot_x].chars().nth(rot_y).unwrap();
        }
    }

    slope_map
}


#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let slope_map : Vec<Vec<char>> = string_to_matrix(input);
    let slope_map_x_size : usize = slope_map[0].len();

    let mut x_alignment : usize = 0;
    let mut tree_count : i32 = 0;
    let mut first_slope : bool = true;

    for line in slope_map {

        if first_slope {
            first_slope = false;
        } else {
            x_alignment = (x_alignment + 3) % (slope_map_x_size);
            println!("pos {}: {}", x_alignment, line[x_alignment]);
            if line[x_alignment] == '#' {
                tree_count += 1;
            }
        }
    }

    tree_count
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let slope_map : Vec<Vec<char>> = string_to_matrix(input);
    let slope_map_x_size : usize = slope_map[0].len();

    let mut x_alignments : Vec<usize> = vec![0; 5];
    let mut tree_counts : Vec<usize> = vec![0; 5];
    let mut second_slope : bool = false;
    let mut first_slope : bool = true;

    for line in slope_map {

        if first_slope {
            first_slope = false;
        } else {
            x_alignments[0] = (x_alignments[0] + 1) % (slope_map_x_size);
            x_alignments[1] = (x_alignments[1] + 3) % (slope_map_x_size);
            x_alignments[2] = (x_alignments[2] + 5) % (slope_map_x_size);
            x_alignments[3] = (x_alignments[3] + 7) % (slope_map_x_size);
            if line[x_alignments[0]] == '#' {
                tree_counts[0] += 1;
            }
            if line[x_alignments[1]] == '#' {
                tree_counts[1] += 1;
            }
            if line[x_alignments[2]] == '#' {
                tree_counts[2] += 1;
            }
            if line[x_alignments[3]] == '#' {
                tree_counts[3] += 1;
            }

            if second_slope {
                x_alignments[4] = (x_alignments[4] + 1) % (slope_map_x_size);
                if line[x_alignments[4]] == '#' {
                    tree_counts[4] += 1;
                }
            }
            second_slope = !second_slope;
        }
    }

    println!("{}", tree_counts[0]);
    println!("{}", tree_counts[1]);
    println!("{}", tree_counts[2]);
    println!("{}", tree_counts[3]);
    println!("{}", tree_counts[4]);
    (tree_counts[0] * tree_counts[1] * tree_counts[2] * tree_counts[3] * tree_counts[4]) as i64
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
