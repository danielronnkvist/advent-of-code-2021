fn main() {
    let lines: Vec<i32> = advent_of_code::read_and_parse("inputs/day1.txt");

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
