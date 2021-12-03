fn main() {
    let lines: Vec<String> = advent_of_code::read_and_parse("inputs/day3.txt");

    let mut commons: Vec<(usize, usize)> = vec![(0, 0); lines.get(0).unwrap().len()];
    for line in &lines {
        for (index, c) in line.chars().enumerate() {
            match &c {
                '0' => commons[index].0 += 1,
                '1' => commons[index].1 += 1,
                _ => unimplemented!(),
            };
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    commons.iter().for_each(|bit| {
        if bit.0 > bit.1 {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    });
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Puzzle 1: {}", gamma * epsilon);
}
