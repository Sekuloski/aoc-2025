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
    let mut sign: char = '0';
    let lines = input.lines().collect::<Vec<&str>>();
    let mut numbers: Vec<String> = vec![];
    let mut num_length = 0;
    for x in 0..lines[0].len() {
        for y in (0..lines.len()).rev() {
            let c = lines[y].chars().collect::<Vec<char>>()[x];
            if c == '*' || c == '+' {
                if sign == '0' {
                    sign = c;
                    continue;
                }
                let mut total: i64;
                if sign == '*' {
                    total = 1;
                    for number in &numbers {
                        if number.chars().filter(|x| !x.is_whitespace()).collect::<Vec<char>>().is_empty() {
                            continue;
                        }
                        total *= number.chars().rev().filter(|x| !x.is_whitespace()).collect::<String>().parse::<i64>().unwrap();
                    }
                } else {
                    total = 0;
                    for number in &numbers {
                        if number.chars().filter(|x| !x.is_whitespace()).collect::<Vec<char>>().is_empty() {
                            continue;
                        }
                        total += number.chars().rev().filter(|x| !x.is_whitespace()).collect::<String>().parse::<i64>().unwrap();
                    }
                }
                answer += total;
                numbers = vec![];
                sign = c;
                num_length = 0;
                continue;
            }
            if num_length >= numbers.len() {
                numbers.push(String::from(c));
            } else {
                numbers[num_length].push(c);
            }
        }
        num_length += 1;
    }
    answer 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "123 328  51 64  
 45 64  387 23  
  6 98  215 314 
*   +   *   +  *" // Added an extra * and whitespace to end the cycle"
        );
        dbg!(&result);
        assert_eq!(result, 3263827);
    }
}

