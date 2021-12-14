use std::collections::HashMap;

fn main() {
    let mut pairs: HashMap<String, String> = HashMap::new();
    let mut polymer: String = String::new();
    advent_of_code::read_lines("inputs/day14.txt")
        .unwrap()
        .into_iter()
        .for_each(|line| {
            let line = line.unwrap();
            if line == "" {
                return;
            }
            if polymer == "" {
                polymer = line;
                return;
            }
            let s: Vec<&str> = line.split(" -> ").collect();
            pairs.insert(s[0].to_string(), s[1].to_string());
        });

    for _ in 0..10 {
        let mut p = String::new();
        for i in 0..polymer.len() - 1 {
            let mut pair = polymer[i..=i + 1].to_string();
            if let Some(m) = pairs.get(&pair) {
                pair.insert_str(1, m);
            }
            if p.len() > 0 {
                p = p[0..p.len() - 1].to_string();
            }
            p += &pair;
        }
        polymer = p;
    }
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        if let Some(co) = count.get_mut(&c) {
            *co += 1;
        } else {
            count.insert(c, 1);
        }
    }
    let p1 = count.values().max().unwrap() - count.values().min().unwrap();
    println!("Puzzle 1: {}", p1);
}
