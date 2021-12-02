fn main() {
    let lines: Vec<String> = advent_of_code::read_and_parse("inputs/day2.txt");

    let mut depth = 0;
    let mut horizontal = 0;
    for c in lines {
        let mut splits = c.split_whitespace();
        let command = splits.next().unwrap();
        let value: i32 = splits.next().unwrap().parse().unwrap();

        match command {
            "forward" => horizontal += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => unimplemented!(),
        };
    }
    println!("Puzzle 1: {}", depth * horizontal);
}
