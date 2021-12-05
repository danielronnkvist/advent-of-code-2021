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
    let mut straight_lines: HashMap<Point, i32> = HashMap::new();
    let mut diagonal_lines: HashMap<Point, i32> = HashMap::new();
    advent_of_code::read_lines("inputs/day5.txt")
        .unwrap()
        .into_iter()
        .for_each(|line| {
            let line = line.unwrap();
            let line: Line = line
                .split(" -> ")
                .map(|p| {
                    p.split(",")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()[0..2]
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<Point>>()[0..2]
                .try_into()
                .unwrap();
            let x1 = line[0][0];
            let x2 = line[1][0];
            let y1 = line[0][1];
            let y2 = line[1][1];

            let x_step = x1.cmp(&x2) as i32;
            let y_step = y1.cmp(&y2) as i32;
            let range_target = (x1 - x2).abs().max((y1 - y2).abs());

            for i in 0..=range_target {
                let key = [x1 - (x_step * i), y1 - (y_step * i)];
                insert(&mut diagonal_lines, key);
                if x_step == 0 || y_step == 0 {
                    insert(&mut straight_lines, key);
                }
            }
        });

    let sum = straight_lines.values().filter(|&&v| v > 1).count();
    println!("Puzzle 1: {}", sum);

    let sum = diagonal_lines.values().filter(|&&v| v > 1).count();
    println!("Puzzle 2: {}", sum);
}
