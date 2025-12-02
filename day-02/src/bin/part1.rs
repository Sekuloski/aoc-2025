fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn split_string_in_middle(s: &str) -> (String, String) {
    let char_count = s.chars().count();
    let middle_char_index = char_count / 2;

    let mut left_part = String::new();
    let mut right_part = String::new();

    for (i, c) in s.chars().enumerate() {
        if i < middle_char_index {
            left_part.push(c);
        } else {
            right_part.push(c);
        }
    }

    (left_part, right_part)
}

fn part1(input: &str) -> i64 {
    let mut answer: i64 = 0;
    let mut input = input.to_string();
    input.pop();
    for number_range in input.split(",").collect::<Vec<&str>>() {
        let number_range = number_range.split("-").collect::<Vec<&str>>();
        let start = number_range[0].parse::<i64>().unwrap();
        let end = number_range[1].parse::<i64>().unwrap();
        for number in start..=end {
            let number_txt = number.to_string();
            if number_txt.len() % 2 == 0 {
                let (left, right) = split_string_in_middle(number_txt.as_str());
                if left == right {
                    answer += number;
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
    fn test_part1() {
        let result = part1("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        assert_eq!(result, 1227775554);
    }
}
