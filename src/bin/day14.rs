use std::collections::HashMap;

fn insert<K>(map: &mut HashMap<K, usize>, key: K, value: usize)
where
    K: Eq + std::hash::Hash,
{
    if let Some(count) = map.get_mut(&key) {
        *count += value;
    } else {
        map.insert(key, value);
    }
}

fn iterate(polymer: String, pairs: &HashMap<String, String>, iterations: usize) -> usize {
    let mut chars: HashMap<char, usize> = HashMap::new();
    let mut pair_count: HashMap<String, usize> = HashMap::new();
    for i in 0..polymer.len() - 1 {
        insert(&mut chars, polymer.chars().nth(i).unwrap(), 1);
        let pair = &polymer[i..=i + 1];
        insert(&mut pair_count, pair.to_string(), 1);
    }
    insert(
        &mut chars,
        polymer.chars().nth(polymer.len() - 1).unwrap(),
        1,
    );
    for _ in 0..iterations {
        let pc = pair_count.clone();
        for (pair, c) in pc {
            if let Some(m) = pairs.get(&pair) {
                if let Some(count) = pair_count.get_mut(&pair) {
                    if *count > 0 {
                        *count -= c;
                    }
                    if *count == 0 {
                        pair_count.remove(&pair);
                    }
                }
                let a = pair.chars().nth(0).unwrap().to_string() + m;
                let b = m.to_owned() + &pair.chars().nth(1).unwrap().to_string();
                insert(&mut pair_count, a, c);
                insert(&mut pair_count, b, c);
                insert(&mut chars, m.chars().nth(0).unwrap(), c);
            }
        }
    }
    chars.values().max().unwrap() - chars.values().min().unwrap()
}

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

    let p1 = iterate(polymer.clone(), &pairs, 10);
    // assert_eq!(p1, 1588);
    println!("Puzzle 1: {}", p1);
    let p2 = iterate(polymer, &pairs, 40);
    // assert_eq!(p2, 2188189693529);
    println!("Puzzle 2: {}", p2);
}
