fn main() {
    let result = advent_of_code::read_lines("inputs/day1.txt")
        .unwrap()
        .into_iter()
        .fold((0, i32::MAX), |(count, previous), depth| {
            let depth: i32 = depth.unwrap().parse().expect("parse line as i32");
            if depth > previous {
                (count + 1, depth)
            } else {
                (count, depth)
            }
        })
        .0;
    println!("Puzzle 1: {}", result);

    let lines = advent_of_code::read_lines("inputs/day1.txt")
        .unwrap()
        .into_iter()
        .map(|depth| depth.unwrap().parse::<i32>().expect("parse line as i32"))
        .collect::<Vec<i32>>();
    let result = lines
        .iter()
        .enumerate()
        .map(|(index, w1)| {
            if index + 2 == lines.len() {
                return None;
            }
            let w2 = lines.get(index + 1).unwrap();
            let w3 = lines.get(index + 2).unwrap();
            Some(w1 + w2 + w3)
        })
        .take_while(|x| x.is_some())
        .fold((0, i32::MAX), |(count, previous), depth| {
            if depth.unwrap() > previous {
                (count + 1, depth.unwrap())
            } else {
                (count, depth.unwrap())
            }
        })
        .0;
    println!("Puzzle 2: {}", result);
}
