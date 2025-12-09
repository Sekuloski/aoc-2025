fn main() {
    let input = include_str!("./input.txt");
    let output = part2(&remove_trailing_newline(input));
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

fn part2(input: &str) -> i64{
    let mut answer: i64 = 0;
    let mut points: Vec<Point> = vec![];
    for line in input.lines() {
        let parts = line.split(",").collect::<Vec<&str>>();
        let x = parts[0].parse::<i64>().unwrap();
        let y = parts[1].parse::<i64>().unwrap();
        points.push(Point {x, y});
    }
    for i in 0..points.len() {
        'outer: for j in i+1..points.len() {
            let area = points[i].get_area(&points[j]);

            if area > answer {
                println!("Checking {}", area);
                // println!("Points: {}, {},  {}, {}", points[i].x, points[i].y, points[j].x, points[j].y);
                let min_x = points[i].x.min(points[j].x);
                let min_y = points[i].y.min(points[j].y);
                let max_x = points[i].x.max(points[j].x);
                let max_y = points[i].y.max(points[j].y);
                let mut points_to_check: Vec<(i64, i64)> = vec![];
                for y in min_y..=max_y {
                    points_to_check.push((min_x, y));
                    points_to_check.push((max_x, y));
                }
                for x in min_x..=max_x {
                    points_to_check.push((x, min_y));
                    points_to_check.push((x, max_y));
                }
                'inner: for (x, y) in points_to_check {
                    // println!("Testing point {}, {}", x, y);
                    let mut intersections = 0;
                    for p in 0..points.len() {
                        let point_1 = &points[p];
                        let point_2 = &points[(p + 1) % points.len()];
                        if point_1.x == point_2.x {
                            if x == point_1.x {
                                if (point_1.y >= y && y >= point_2.y) || (point_2.y >= y && y >= point_1.y) {
                                    continue 'inner;
                                }
                            } else if x < point_1.x {
                                if (point_1.y >= y && y >= point_2.y) || (point_2.y >= y && y >= point_1.y) {
                                    // println!("Found intersaction: {}, {},  {}, {}", point_1.x, point_1.y, point_2.x, point_2.y);
                                    intersections += 1;
                                }
                            }
                        } else {
                            if y == point_1.y {
                                if (point_1.x >= x && x >= point_2.x) || (point_2.x >= x && x >= point_1.x) {
                                    continue 'inner;
                                }
                            }
                        }
                    }
                    if intersections % 2 == 0 {
                        // println!("Found point outside of area: {}, {}, {}", x, y, intersections);
                        continue 'outer;
                    }
                }
                answer = area;
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
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
        assert_eq!(result, 24);
    }
}

