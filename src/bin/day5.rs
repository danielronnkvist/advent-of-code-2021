use std::collections::HashMap;

type Point = [i32; 2];
type Line = [Point; 2];

fn insert(map: &mut HashMap<Point, i32>, key: Point) {
    if let Some(p) = map.get_mut(&key) {
        *p += 1;
    } else {
        map.insert(key, 1);
    }
}

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

    let mut straight_lines: HashMap<Point, i32> = HashMap::new();
    let mut diagonal_lines: HashMap<Point, i32> = HashMap::new();
    for line in &lines {
        let x1 = line[0][0];
        let x2 = line[1][0];
        let y1 = line[0][1];
        let y2 = line[1][1];
        if line[0][0] == line[1][0] {
            for y in y1.min(y2)..=y1.max(y2) {
                let key = [line[0][0], y];
                insert(&mut straight_lines, key);
                insert(&mut diagonal_lines, key);
            }
        } else if line[0][1] == line[1][1] {
            for x in x1.min(x2)..=x1.max(x2) {
                let key = [x, line[0][1]];
                insert(&mut straight_lines, key);
                insert(&mut diagonal_lines, key);
            }
        } else {
            let x_step = match x1 < x2 {
                true => 1,
                false => -1,
            };
            let y_step = match y1 < y2 {
                true => 1,
                false => -1,
            };

            for i in 0..=(x1.max(x2) - x1.min(x2)) {
                let key = [x1 + (x_step * i), y1 + (y_step * i)];
                insert(&mut diagonal_lines, key);
            }
        }
    }
    let sum = straight_lines.values().filter(|&&v| v > 1).count();
    println!("Puzzle 1: {}", sum);

    let sum = diagonal_lines.values().filter(|&&v| v > 1).count();
    println!("Puzzle 2: {}", sum);
}
