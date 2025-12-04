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
    let elements = [-1, 0, 1];
    let mut combinations: Vec<Vec<i32>> = vec![];
    for i in 0..elements.len() {
        for j in 0..elements.len() {
            combinations.push(vec![elements[i], elements[j]]);
        }
    }
    let mut data: Vec<Vec<bool>> = vec![];
    let mut temp_data: Vec<Vec<bool>> = vec![];
    let mut answer = 0;
    for (y, line) in input.lines().enumerate() {
        data.push(Vec::new());
        for c in line.chars() {
            data[y].push(c == '@');
        }
    }
    let mut found_paper = true;
    loop {
        if !found_paper {
            break;
        }
        found_paper = false;
        for y in 0..data.len() {
            temp_data.push(vec![]);
            for x in 0..data[y].len() {
                if !data[y][x] {
                    print!(".");
                    temp_data[y].push(false);
                    continue;
                }
                let mut surrounding = 0;
                for combination in combinations.iter() {
                    if combination[0] == 0 && combination[1] == 0 {
                        continue;
                    }
                    if y as i32 + combination[0] < 0 || x as i32 + combination[1] < 0 {
                        continue;
                    }

                    if y as i32 + combination[0] == data.len() as i32 || x as i32 + combination[1] == data[y].len() as i32 {
                        continue;
                    }
                    let y = (y as i32 + combination[0]) as usize;
                    let x = (x as i32 + combination[1]) as usize;
                    if data[y][x] {
                        surrounding += 1;
                    }
                }
                if surrounding < 4 {
                    answer += 1;
                    found_paper = true;
                    print!("x");
                    temp_data[y].push(false);
                } else {
                    print!("@");
                    temp_data[y].push(true);
                }
            }
            println!();
        }
        data = temp_data.clone();
        temp_data = vec![];
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        );
        dbg!(&result);
        assert_eq!(result, 43);
    }
}

