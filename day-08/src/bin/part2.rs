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

fn part2(input: &str) -> i64{
    let mut answer: i64 = 0;
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            ""
        );
        dbg!(&result);
        assert_eq!(result, 0);
    }
}

