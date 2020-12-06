use std::collections::LinkedList;
use std::collections::HashSet;
use std::collections::HashMap;


pub fn create_hash_set_of_answers(answers : String) -> HashSet<char> {
    
    let mut hash_set : HashSet<char> = HashSet::new();

    for answer in answers.chars() {
        if answer != ' ' {
            hash_set.insert(answer);
        }
    }

    hash_set
}

pub fn create_hash_set_of_answers_part_two(answers : String, party_size : usize) -> HashSet<char> {
    let mut hash_set : HashSet<char> = HashSet::new();
    let mut hash_map : HashMap<char, i32> = HashMap::new();

    for answer in answers.chars() {
        if answer != ' ' {
            if hash_map.contains_key(&answer) {
                let old_value : i32 = hash_map.remove(&answer).unwrap();                
                hash_map.insert(answer, old_value + 1);

            } else {
                hash_map.insert(answer, 1);
            }
        }
    }

    for entry in hash_map.keys() {
        if hash_map.get(&entry).unwrap() == &(party_size as i32) {
            hash_set.insert(*entry);
        }
    }

    hash_set
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut answer_sheets : LinkedList<String> = LinkedList::new();

    let mut answer_sheet_data : String = String::new();

    for line in input.lines() {
        if line.is_empty() {
            answer_sheets.push_back(answer_sheet_data);
            answer_sheet_data = String::new();
        } else {
            answer_sheet_data.insert_str(answer_sheet_data.len(), " ");
            answer_sheet_data.insert_str(answer_sheet_data.len(), line);
        }
    }
    answer_sheets.push_back(answer_sheet_data);

    let mut answer_sum : i32 = 0;
    for answer_sheet in answer_sheets {
        answer_sum += create_hash_set_of_answers(answer_sheet).len() as i32;
    }

    answer_sum
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut answer_sheets : LinkedList<HashSet<char>> = LinkedList::new();

    let mut answer_sheet_data : String = String::new();
    let mut party_count : usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            answer_sheets.push_back(create_hash_set_of_answers_part_two(answer_sheet_data, party_count));
            answer_sheet_data = String::new();
            party_count = 0;
        } else {
            answer_sheet_data.insert_str(answer_sheet_data.len(), " ");
            answer_sheet_data.insert_str(answer_sheet_data.len(), line);
            party_count += 1;
        }
    }
    answer_sheets.push_back(create_hash_set_of_answers_part_two(answer_sheet_data, party_count));

    let mut answer_sum : i32 = 0;
    for answer_sheet in answer_sheets {
        answer_sum += answer_sheet.len() as i32;
    }

    answer_sum
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
