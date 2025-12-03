fn main() {
    let input = include_str!("./input.txt");
    let output = part1(&remove_trailing_newline(input));
    dbg!(output);
}

fn remove_trailing_newline(input: &str) -> String {
    input.strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

fn part1(input: &str) -> i32 {
    let mut answer: i32 = 0;
    for bank in input.lines().collect::<Vec<&str>>() {
        let max_length = bank.chars().count() - 1;
        let mut highest_number = 0;
        let mut highest_index = 0;
        let mut second_highest_number = 0;
        for (i, c) in bank.chars().enumerate() {
            let number = c.to_digit(10).unwrap();
            if number > highest_number {
                highest_number = number.clone();
                highest_index = i.clone();
            }
        }
        if highest_index == max_length {
            second_highest_number = highest_number.clone();
            highest_number = 0;
            for (i, c) in bank.chars().enumerate() {
                if i == max_length {
                    break;
                }
                let number = c.to_digit(10).unwrap();
                if number > highest_number {
                    highest_number = number.clone();
                }   
            }
        } else {
            for (i, c) in bank.chars().enumerate() {
                if i <= highest_index {
                    continue;
                }
                let number = c.to_digit(10).unwrap();
                if number > second_highest_number {
                    second_highest_number = number.clone();
                }   
            }
        }
        let jolts = format!("{}{}", highest_number, second_highest_number).parse::<i32>().unwrap();
        answer += jolts;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "987654321111111
811111111111119
234234234234278
818181911112111"
        );
        assert_eq!(result, 357);
    }
}

