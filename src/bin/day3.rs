fn main() {
    let lines: Vec<String> = advent_of_code::read_and_parse("inputs/day3.txt");

    let mut commons: Vec<(Vec<String>, Vec<String>)> =
        vec![(vec![], vec![]); lines.get(0).unwrap().len()];
    for line in &lines {
        for (index, c) in line.chars().enumerate() {
            match &c {
                '0' => commons[index].0.push(line.clone()),
                '1' => commons[index].1.push(line.clone()),
                _ => unimplemented!(),
            };
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();

    let mut oxygen: Vec<String> = lines.clone();
    let mut scrubber: Vec<String> = lines.clone();

    commons.iter().for_each(|bit| {
        if bit.0.len() > bit.1.len() {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }

        if oxygen.len() > 1 {
            if bit.0.iter().filter(|&a| oxygen.contains(a)).count()
                > bit.1.iter().filter(|&a| oxygen.contains(a)).count()
            {
                oxygen = bit
                    .0
                    .iter()
                    .filter(|a| oxygen.iter().find(|b| b == a).is_some())
                    .map(|a| a.clone())
                    .collect();
            } else {
                oxygen = bit
                    .1
                    .iter()
                    .filter(|a| oxygen.iter().find(|b| b == a).is_some())
                    .map(|a| a.clone())
                    .collect();
            }
        }
        if scrubber.len() > 1 {
            if bit.0.iter().filter(|&a| scrubber.contains(a)).count()
                > bit.1.iter().filter(|&a| scrubber.contains(a)).count()
            {
                scrubber = bit
                    .1
                    .iter()
                    .filter(|a| scrubber.iter().find(|b| b == a).is_some())
                    .map(|a| a.clone())
                    .collect();
            } else {
                scrubber = bit
                    .0
                    .iter()
                    .filter(|a| scrubber.iter().find(|b| b == a).is_some())
                    .map(|a| a.clone())
                    .collect();
            }
        }
    });
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Puzzle 1: {}", gamma * epsilon);

    let oxygen = isize::from_str_radix(&oxygen[0], 2).unwrap();
    let scrubber = isize::from_str_radix(&scrubber[0], 2).unwrap();
    println!("Puzzle 2: {}", oxygen * scrubber);
}
