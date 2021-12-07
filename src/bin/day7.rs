fn main() {
    let mut crabs: Vec<i64> = advent_of_code::read_lines("inputs/day7.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    crabs.sort();
    let target = crabs[crabs.len() / 2];

    let sum = crabs
        .iter()
        .fold(0, |sum, crab| sum + (crab - target).abs());
    println!("Puzzle 1: {:?}", sum);

    let max = crabs.iter().max();
    let mut counted = vec![];
    for i in 0..*max.unwrap() {
        let sum: i64 = crabs.iter().fold(0, |sum, crab| {
            let n = (crab - i).abs();
            sum + ((n * (n + 1)) / 2)
        });
        counted.push(sum);
    }

    println!("Puzzle 2: {:?}", counted.iter().min().unwrap());
}
