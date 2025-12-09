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

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn get_distance(&self, other: &Point) -> i64 {
        let delta_x = (other.x - self.x).pow(2) as f64;
        let delta_y = (other.y - self.y).pow(2) as f64;
        let delta_z = (other.z - self.z).pow(2) as f64;

        (delta_x + delta_y + delta_z).sqrt() as i64
    }
}

fn part1(input: &str) -> i64{
    let mut answer: i64 = 1;
    let mut all_points: Vec<Point> = vec![];
    let mut distances: Vec<(i64, (usize, usize))> = vec![];
    let mut junctions: Vec<Vec<usize>> = vec![];
    for line in input.lines().collect::<Vec<&str>>() {
        let values = line.split(",").collect::<Vec<&str>>();
        let x = values[0].parse::<i64>().unwrap();
        let y = values[1].parse::<i64>().unwrap();
        let z = values[2].parse::<i64>().unwrap();
        all_points.push(Point { x, y, z });
    }
    for i in 0..all_points.len() {
        for j in i+1..all_points.len() {
            distances.push((all_points[i].get_distance(&all_points[j]), (i, j)));
        }
    }
    distances.sort_by(|a, b| a.0.cmp(&b.0));
    let mut counter = 0;
    'outer: for (_, points) in distances {
        if counter >= 1000 {
            break;
        }
        counter += 1;
        let mut junctions_to_connect: Vec<usize> = vec![];
        let mut points_to_add: Vec<usize> = vec![];
        'inner: for junction_id in 0..junctions.len() {
            let junction = &junctions[junction_id];
            if junction.contains(&points.0) && junction.contains(&points.1) {
                continue 'outer;   
            } else if junction.contains(&points.0) {
                junctions_to_connect.push(junction_id);
                points_to_add.push(points.1);
                if junctions_to_connect.len() == 2 {
                    break 'inner;
                }
            } else if junction.contains(&points.1) {
                junctions_to_connect.push(junction_id);
                points_to_add.push(points.0);
                if junctions_to_connect.len() == 2 {
                    break 'inner;
                }
            }
        }
        if junctions_to_connect.len() == 2 {
            junctions_to_connect.sort();
            let mut junction = junctions.remove(junctions_to_connect[0]);
            let mut junction2 = junctions.remove(junctions_to_connect[1] - 1);
            junction.append(&mut junction2);
            junction.dedup();
            junctions.push(junction);
        } else if junctions_to_connect.len() == 1 {
            junctions[junctions_to_connect[0]].push(points_to_add[0]);
        } else {
            junctions.push(vec![points.0, points.1]);
        }
    }
    let mut temp = junctions.iter().map(|x| x.len() as i64).collect::<Vec<i64>>();
    temp.sort();
    temp.dedup();
    temp.reverse();
    answer *= temp[0];
    answer *= temp[1];
    answer *= temp[2];
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        );
        dbg!(&result);
        assert_eq!(result, 40);
    }
}

