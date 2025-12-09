fn main() {
    let input = include_str!("./input.txt");
    let output = part1(&remove_trailing_newline(input));
    dbg!(output);
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64 
}

impl Point {
    fn get_area(&self, other: &Point) -> i64 {
        let x = (self.x - other.x).abs() + 1;
        let y = (self.y - other.y).abs() + 1;
        x * y
    }
}

fn remove_trailing_newline(input: &str) -> String {
    input.strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

fn part1(input: &str) -> i64{
    let mut answer: i64 = 0;
    let mut points: Vec<Point> = vec![];
    for line in input.lines() {
        let parts = line.split(",").collect::<Vec<&str>>();
        let x = parts[0].parse::<i64>().unwrap();
        let y = parts[1].parse::<i64>().unwrap();
        points.push(Point {x, y});
    }
    for i in 0..points.len() {
        for j in i+1..points.len() {
            answer = points[i].get_area(&points[j]).max(answer);
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
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        );
        dbg!(&result);
        assert_eq!(result, 50);
    }
}

