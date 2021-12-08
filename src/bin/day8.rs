fn main() {
    let lines: Vec<(Vec<String>, Vec<String>)> = advent_of_code::read_and_parse("inputs/day8.txt")
        .iter()
        .map(|line: &String| {
            let mut r = line
                .split(" | ")
                .map(|p: &str| p.split(" ").map(|a| a.to_owned()).collect::<Vec<String>>());
            (r.next().unwrap(), r.next().unwrap())
        })
        .collect();

    let mut p1 = 0;
    for (_, code) in lines {
        for c in code {
            p1 += match c.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }
    println!("Puzzle 1: {}", p1);
}
