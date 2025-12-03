fn main() {
    let input = include_str!("./input.txt");
    let output = part2(&remove_trailing_newline(input));
    dbg!(output);
}

fn remove_trailing_newline(input: &str) -> String {
    input.strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

fn part2(input: &str) -> i64 {
    let mut answer: i64 = 0;
    let max_bank_size = 12;
    for bank in input.lines().collect::<Vec<&str>>() {
        let mut characters = &bank.chars().collect::<Vec<char>>()[..];
        let mut highest_index: usize = 0;
        let mut jolts = String::new();
        let mut highest_number: u8 = 0;
        for bank_size in (0..max_bank_size).rev() {
            characters = if highest_number > 0 {
                &characters[highest_index+1..]
            }
            else {
                &characters[highest_index..]
            };
            let max_length = characters.into_iter().count() - 1;
            highest_number = 0;
            dbg!(bank_size, highest_index, max_length);
            dbg!(characters);
            for (i, c) in characters.into_iter().enumerate() {
                if i > max_length - bank_size {
                    break;
                }
                let number: u8 = c.to_digit(10).unwrap() as u8;
                if number > highest_number {
                    highest_number = number.clone();
                    highest_index = i.clone();
                }
            }
            jolts.push_str(&highest_number.clone().to_string());
        }
        dbg!(&jolts);
        answer += jolts.parse::<i64>().unwrap()
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "987654321111111
811111111111119
234234234234278
818181911112111"
        );
        dbg!(result);
        assert_eq!(result, 3121910778619);
    }
}

