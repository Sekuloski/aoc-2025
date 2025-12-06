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

fn part1(input: &str) -> i64 {
    let mut values: Vec<Vec<&str>> = vec![];
    for _ in 0..input.lines().collect::<Vec<&str>>()[0].split_whitespace().filter(|x| !x.is_empty()).collect::<Vec<&str>>().len() {
        values.push(vec![]);
    }
    for line in input.lines() {
        for (i, value) in line.split_whitespace().filter(|x| !x.is_empty()).enumerate() {
            values[i].push(value);
        }
    }
    let mut answer: i64 = 0;
    for problem in values {
        if *(problem.last().unwrap()) == "*" {
            let mut total: i64 = 1;
            for value in problem[..problem.len() - 1].iter() {
                total *= value.parse::<i64>().unwrap();
            }
            answer += total;
        } else {
            let mut total: i64 = 0;
            for value in problem[..problem.len() - 1].iter() {
                total += value.parse::<i64>().unwrap();
            }
            answer += total;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + "
        );
        dbg!(&result);
        assert_eq!(result, 4277556);
    }
}

