fn main() {
    let lines: Vec<String> = advent_of_code::read_and_parse("inputs/day2.txt");

    let mut p1_depth = 0;
    let mut p2_depth = 0;
    let mut horizontal = 0;
    for c in &lines {
        let mut splits = c.split_whitespace();
        let command = splits.next().unwrap();
        let value: i32 = splits.next().unwrap().parse().unwrap();

        match command {
            "forward" => {
                horizontal += value;
                p2_depth += p1_depth * value;
            }
            "up" => p1_depth -= value,
            "down" => p1_depth += value,
            _ => unimplemented!(),
        };
    }
    println!("Puzzle 1: {}", p1_depth * horizontal);
    println!("Puzzle 2: {}", p2_depth * horizontal);
}
