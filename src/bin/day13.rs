fn fold(coordinates: &Vec<(i32, i32)>, fold: (char, i32)) -> Vec<(i32, i32)> {
    coordinates.iter().fold(vec![], |mut sum, (x, y)| {
        let coordinate = {
            if fold.0 == 'x' {
                if x > &fold.1 {
                    let x = fold.1 - (x - fold.1);
                    (x, *y)
                } else {
                    (*x, *y)
                }
            } else {
                if y > &fold.1 {
                    let y = fold.1 - (y - fold.1);
                    (*x, y)
                } else {
                    (*x, *y)
                }
            }
        };
        if !sum.contains(&coordinate) {
            sum.push(coordinate);
        }
        sum
    })
}

fn main() {
    let mut coordinates: Vec<(i32, i32)> = vec![];
    let mut folds: Vec<(char, i32)> = vec![];
    advent_of_code::read_lines("inputs/day13.txt")
        .unwrap()
        .into_iter()
        .for_each(|line| {
            let line = line.unwrap();
            if line == "" {
                return;
            }
            if line.contains("fold along") {
                let mut s = line.split("=");
                let axis = s.next().unwrap().chars().last().unwrap();
                let n: i32 = s.next().unwrap().parse().unwrap();
                folds.push((axis, n));
            } else {
                let mut s = line.split(",");
                let x: i32 = s.next().unwrap().parse().unwrap();
                let y: i32 = s.next().unwrap().parse().unwrap();
                coordinates.push((x, y));
            }
        });

    let p1 = fold(&coordinates, folds[0]).len();
    println!("Puzzle 1: {}", p1);

    for f in folds {
        coordinates = fold(&coordinates, f);
    }
    for y in 0..6 {
        for x in 0..5 * 8 {
            if coordinates.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
