fn main() {
    let fishes: Vec<usize> = advent_of_code::read_lines("inputs/day6.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let iterate = |iterations: i32| -> usize {
        let mut lifes: [usize; 9] = [0; 9];

        for fish in &fishes {
            lifes[*fish] += 1;
        }

        for _ in 0..iterations {
            lifes.rotate_left(1);
            lifes[6] += lifes[8];
        }

        lifes.iter().sum()
    };
    println!("Puzzle 1: {}", iterate(80));
    println!("Puzzle 2: {}", iterate(256));
}
