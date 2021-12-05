use std::collections::HashMap;

type Point = [i32; 2];
type Line = [Point; 2];

fn main() {
    let lines: Vec<Line> = advent_of_code::read_and_parse("inputs/day5.txt")
        .iter()
        .map(|line: &String| {
            line.split(" -> ")
                .map(|p| {
                    p.split(",")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()[0..2]
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<Point>>()[0..2]
                .try_into()
                .unwrap()
        })
        .collect();

    let mut targets: HashMap<Point, i32> = HashMap::new();
    for line in &lines {
        if line[0][0] == line[1][0] {
            let y1 = line[0][1];
            let y2 = line[1][1];
            for y in y1.min(y2)..=y1.max(y2) {
                let key = [line[0][0], y];
                if let Some(p) = targets.get_mut(&key) {
                    *p += 1;
                } else {
                    targets.insert(key, 1);
                }
            }
        } else if line[0][1] == line[1][1] {
            let x1 = line[0][0];
            let x2 = line[1][0];
            for x in x1.min(x2)..=x1.max(x2) {
                let key = [x, line[0][1]];
                if let Some(p) = targets.get_mut(&key) {
                    *p += 1;
                } else {
                    targets.insert(key, 1);
                }
            }
        }
    }
    let sum = targets.values().filter(|&&v| v > 1).count();
    println!("Puzzle 1: {}", sum);
}
