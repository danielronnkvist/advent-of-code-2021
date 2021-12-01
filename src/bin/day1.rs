fn main() {
    let lines = advent_of_code::read_lines("inputs/day1.txt")
        .unwrap()
        .into_iter()
        .map(|depth| depth.unwrap().parse::<i32>().expect("parse line as i32"))
        .collect::<Vec<i32>>();

    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..lines.len() {
        if i >= 1 && lines[i] > lines[i - 1] {
            p1 += 1;
        }
        if i >= 3
            && lines[i] + lines[i - 1] + lines[i - 2] > lines[i - 1] + lines[i - 2] + lines[i - 3]
        {
            p2 += 1;
        }
    }
    println!("Puzzle 1: {}", p1);
    println!("Puzzle 2: {}", p2);
}
