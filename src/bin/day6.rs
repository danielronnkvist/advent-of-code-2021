fn main() {
    let mut fishes: Vec<u8> = advent_of_code::read_lines("inputs/day6.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let iterations = 80;
    for _ in 0..iterations {
        fishes = fishes.iter().fold(vec![], |mut tot, fish| {
            if *fish > 0 {
                tot.push(fish - 1);
            } else {
                tot.extend(vec![6, 8]);
            }
            return tot;
        });
    }
    println!("Puzzle 1: {}", fishes.len());
}
