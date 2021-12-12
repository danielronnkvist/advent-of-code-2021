use std::collections::HashMap;

fn find_paths_to_end(
    path: &Vec<String>,
    start: &str,
    caves: &HashMap<String, Vec<String>>,
) -> usize {
    let mut count = 0;

    for cave in caves.get(start).unwrap() {
        if cave == "end" {
            count += 1;
        } else if &cave.to_lowercase() == cave && !path.contains(cave) {
            count += find_paths_to_end(
                &vec![path.clone(), vec![cave.to_string()]].concat(),
                cave,
                caves,
            );
        } else if &cave.to_uppercase() == cave {
            count += find_paths_to_end(
                &vec![path.clone(), vec![cave.to_string()]].concat(),
                cave,
                caves,
            );
        }
    }

    count
}

fn find_paths_to_end_p2(
    path: &Vec<String>,
    start: &str,
    caves: &HashMap<String, Vec<String>>,
) -> usize {
    let mut count = 0;

    for cave in caves.get(start).unwrap() {
        if cave == "start" {
            continue;
        } else if cave == "end" {
            count += 1;
        } else if &cave.to_lowercase() == cave
            && (!path
                .iter()
                .fold(HashMap::new(), |mut sum, c| {
                    if &c.to_lowercase() == c {
                        if let Some(a) = sum.get_mut(c) {
                            *a += 1;
                        } else {
                            sum.insert(c, 1);
                        }
                    }
                    sum
                })
                .values()
                .any(|&v| v == 2)
                || !path.contains(cave))
        {
            count += find_paths_to_end_p2(
                &vec![path.clone(), vec![cave.to_string()]].concat(),
                cave,
                caves,
            );
        } else if &cave.to_uppercase() == cave {
            count += find_paths_to_end_p2(
                &vec![path.clone(), vec![cave.to_string()]].concat(),
                cave,
                caves,
            );
        }
    }

    count
}

fn main() {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();
    advent_of_code::read_lines("inputs/day12.txt")
        .unwrap()
        .into_iter()
        .for_each(|line| {
            let line = line.unwrap();
            let split: Vec<&str> = line.split("-").collect();
            if let Some(cave) = caves.get_mut(&split[0].to_string()) {
                cave.push(split[1].to_string());
            } else {
                caves.insert(split[0].to_string(), vec![split[1].to_string()]);
            }

            if let Some(cave) = caves.get_mut(&split[1].to_string()) {
                cave.push(split[0].to_string());
            } else {
                caves.insert(split[1].to_string(), vec![split[0].to_string()]);
            }
        });

    let p1 = find_paths_to_end(&vec!["start".to_string()], "start", &caves);
    println!("Puzzle 1: {}", p1);

    let p2 = find_paths_to_end_p2(&vec!["start".to_string()], "start", &caves);
    println!("Puzzle 2: {}", p2);
}
