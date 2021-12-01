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
}
