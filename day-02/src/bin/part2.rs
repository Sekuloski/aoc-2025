use itertools::{self, Itertools};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn check_chunk_size(s: &str, chunk_size: i32) -> bool {
    if s.chars().count() as i32 % chunk_size != 0 || chunk_size > (s.chars().count() as i32) / 2 {
        return false;
    }
    let mut left_part = String::new();
    let mut right_part = String::new();

    for c in s.chars() {
        if left_part.len() != chunk_size as usize {
            left_part.push(c);
            continue;
        } else {
            right_part.push(c);
        }
        if right_part.len() == chunk_size as usize {
            if left_part != right_part {
                return false;
            }
            right_part = String::new();
        }
    }
    return right_part.len() == 0
}

fn part2(input: &str) -> i64 {
    let mut answer: i64 = 0;
    let mut input = input.to_string();
    input.pop();
    for number_range in input.split(",").collect::<Vec<&str>>() {
        let number_range = number_range.split("-").collect::<Vec<&str>>();
        let start = number_range[0].parse::<i64>().unwrap();
        let end = number_range[1].parse::<i64>().unwrap();
        'outer: for number in start..=end {
            let number_txt = number.to_string();
            if number_txt.chars().unique().collect::<Vec<char>>().len() == 1 && number_txt.chars().count() > 1 {
                answer += number;
            } else {
                let char_count = number_txt.chars().count() as i32;
                let middle_char_index = char_count / 2;
                for n in 2..=middle_char_index{
                    if check_chunk_size(number_txt.as_str(), n) {
                        answer += number;
                        continue 'outer;
                    }
                }
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_size() {
        let result = check_chunk_size("2121212128", 2);
        assert_eq!(result, false);

        let result = check_chunk_size("1010", 2);
        assert_eq!(result, true);

        let result = check_chunk_size("824824824", 3);
        assert_eq!(result, true);

        let result = check_chunk_size("2121212121", 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_part2() {
        let result = part2("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n");
        assert_eq!(result, 4174379265);
    }
}
