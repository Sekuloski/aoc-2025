fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut point = 50;
    let mut answer = 0;
    for line in input.to_string().lines() {
        if line.contains("L") {
            let movement: i32 = line.split("L").collect::<Vec<&str>>()[1].parse().unwrap();
            point -= movement;
            while point < 0 {
                point = 100 + point;
            }
        } else {
            let movement: i32 = line.split("R").collect::<Vec<&str>>()[1].parse().unwrap();
            point += movement;
            while point > 99 {
                point = point - 100;
            }
        }
        if point == 0 {
            answer += 1;
        }
        dbg!("{}", point);
    };
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
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
        assert_eq!(result, "3");
    }
}

