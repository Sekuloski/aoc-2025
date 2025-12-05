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
    let mut found_empty = false;
    let mut answer = 0;
    let mut fresh: Vec<(i64, i64)> = vec![];
    'outer: for line in input.lines() {
        if !found_empty {
            if line.is_empty() {
                found_empty = true;
                fresh.dedup();
                continue;
            }
            let numbers = line.split("-").collect::<Vec<&str>>();
            fresh.push((numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap()));
        } else {
            let number = &line.parse::<i64>().unwrap();
            for range in &fresh {
                if number < &range.0 || number > &range.1 {
                    continue;
                }
                if number >= &range.0 && number <= &range.1 {
                    answer += 1;
                    continue 'outer;
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
        let result = part1(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        );
        dbg!(&result);
        assert_eq!(result, 3);
    }
}

