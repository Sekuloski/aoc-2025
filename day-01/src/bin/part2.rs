use num::integer::div_floor;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut point = 50;
    let mut started_at_zero: bool;
    let mut answer = 0;
    let input = input.to_string();
    let lines = &input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>();
    for line in lines {
        if line.contains("L") {
            let mut movement: i32 = line.split("L").collect::<Vec<&str>>()[1].parse().unwrap();
            answer += div_floor(movement, 100);
            movement = movement % 100;
            started_at_zero = point == 0;
            point -= movement;
            if point == 0 {
                answer += 1;
                continue;
            } else if point < 0 {
                point = 100 + point;
                if !started_at_zero {
                    answer += 1;
                }
            }
        } else {
            let mut movement: i32 = line.split("R").collect::<Vec<&str>>()[1].parse().unwrap();
            answer += div_floor(movement, 100);
            movement = movement % 100;
            point += movement;
            if point > 99 {
                point = point - 100;
                answer += 1;
            }
        }
    };
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        );
        assert_eq!(result, "6");
    }
}

