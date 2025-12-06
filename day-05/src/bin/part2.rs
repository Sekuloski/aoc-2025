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
    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut temp_ranges: Vec<(i64, i64)> = vec![];
    let mut processed_ranges: Vec<(i64, i64)> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            break
        }
        let numbers = line.split("-").collect::<Vec<&str>>();
        ranges.push((numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap()));
    }
    while !&ranges.is_empty() {
        'outer: for original_range in &ranges {
            let mut range = (original_range.0, original_range.1);
            for second_range in &processed_ranges {
                if range.0 >= second_range.0 && range.1 <= second_range.1 {
                    continue 'outer;
                }
                if range.0 < second_range.0 && range.1 > second_range.1 {
                    temp_ranges.push((range.0, second_range.0 - 1));
                    temp_ranges.push((second_range.1 + 1, range.1));
                    continue 'outer;
                }
                else if &range.0 < &second_range.0 {
                    if &range.1 >= &second_range.0 && &range.1 <= &second_range.1 {
                        range = (range.0, second_range.0 - 1);
                    }
                }
                else if &range.1 > &second_range.1 {
                    if &range.0 >= &second_range.0 && &range.0 <= &second_range.1 {
                        range = (second_range.1 + 1, range.1);
                    }
                }
            }
            processed_ranges.push(range);
        }
        ranges = temp_ranges.clone();
        temp_ranges = vec![];
    }
    for range in &processed_ranges {
        answer += range.1 + 1 - range.0;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
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
        assert_eq!(result, 14);
    }
}

