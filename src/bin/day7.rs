fn main() {
    let mut crabs: Vec<i32> = advent_of_code::read_lines("inputs/day7.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    crabs.sort();
    let target = crabs[crabs.len() / 2];

    let sum = crabs
        .iter()
        .fold(0, |sum, crab| sum + (crab - target).abs());
    println!("Puzzle 1: {:?}", sum);
}
