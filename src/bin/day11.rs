fn bump_neighbours(map: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let x = x as i32;
    let y = y as i32;
    for y1 in (y - 1).max(0)..=(y + 1).min(9) {
        for x1 in (x - 1).max(0)..=(x + 1).min(9) {
            if !(x1 == x && y1 == y) {
                let x1 = x1 as usize;
                let y1 = y1 as usize;
                map[y1][x1] += 1;
                if map[y1][x1] == 10 {
                    bump_neighbours(map, x1, y1);
                }
            }
        }
    }
}

fn main() {
    let mut map: Vec<Vec<i32>> = advent_of_code::read_lines("inputs/day11.txt")
        .unwrap()
        .into_iter()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut p1 = 0;
    let mut i = 0;
    loop {
        for y in 0..10 {
            for x in 0..10 {
                // Step 1
                map[y][x] += 1;

                // Step 2
                if map[y][x] == 10 {
                    bump_neighbours(&mut map, x, y);
                }
            }
        }

        // Step 3
        let mut all = true;
        for y in 0..10 {
            for x in 0..10 {
                if map[y][x] > 9 {
                    map[y][x] = 0;
                    if i < 100 {
                        p1 += 1;
                    }
                } else {
                    all = false;
                }
            }
        }

        if all {
            break;
        }

        i += 1;
    }
    println!("Puzzle 1: {}", p1);
    println!("Puzzle 2: {}", i + 1);
}
