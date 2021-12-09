fn is_lower(curr: i32, map: &Vec<Vec<i32>>, x: i32, y: i32) -> Option<bool> {
    if x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32 {
        return None;
    }
    if let Some(row) = map.get(y as usize) {
        if let Some(t) = row.get(x as usize) {
            return Some(curr < *t);
        }
    }
    return Some(false);
}

fn main() {
    let heightmap: Vec<Vec<i32>> = advent_of_code::read_and_parse("inputs/day9.txt")
        .iter()
        .map(|line: &String| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut p1 = 0;
    for y in 0..heightmap.len() as i32 {
        for x in 0..heightmap[0].len() as i32 {
            let curr = heightmap[y as usize][x as usize];
            let is_lowest = vec![
                is_lower(curr, &heightmap, x, y - 1),
                is_lower(curr, &heightmap, x, y + 1),
                is_lower(curr, &heightmap, x - 1, y),
                is_lower(curr, &heightmap, x + 1, y),
            ]
            .iter()
            .all(|a| a.is_none() || (a.is_some() && a.unwrap() == true));

            if is_lowest {
                p1 += curr + 1;
            }
        }
    }
    println!("Puzzle 1: {}", p1);
}
