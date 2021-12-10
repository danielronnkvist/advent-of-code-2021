use std::collections::HashMap;

fn main() {
    let mut end_chunks = HashMap::new();
    end_chunks.insert(')', 3);
    end_chunks.insert(']', 57);
    end_chunks.insert('}', 1197);
    end_chunks.insert('>', 25137);

    let mut autocomplete_score = HashMap::new();
    autocomplete_score.insert(')', 1);
    autocomplete_score.insert(']', 2);
    autocomplete_score.insert('}', 3);
    autocomplete_score.insert('>', 4);

    let chunk_pairs = vec!["()", "[]", "{}", "<>"];
    let mut p1 = 0;
    let mut p2 = vec![];

    advent_of_code::read_lines("inputs/day10.txt")
        .unwrap()
        .into_iter()
        .filter_map(|line| {
            let line = line.unwrap();
            let mut open: Vec<char> = vec![];
            for c in line.chars() {
                for chunk in &chunk_pairs {
                    if let Some(index) = chunk.chars().position(|i| i == c) {
                        if index == 0 {
                            open.push(c);
                        } else if index == 1 {
                            if let Some(pop) = open.pop() {
                                if pop.to_string() + &c.to_string() != **chunk {
                                    p1 += end_chunks.get(&c).unwrap();
                                    return None;
                                }
                            }
                        }
                    }
                }
            }
            Some(line)
        })
        .for_each(|line| {
            let mut open: Vec<char> = vec![];
            for c in line.chars() {
                for chunk in &chunk_pairs {
                    if let Some(index) = chunk.chars().position(|i| i == c) {
                        if index == 0 {
                            open.push(c);
                        } else if index == 1 {
                            open.pop();
                        }
                    }
                }
            }
            let mut autocomplete: Vec<char> = vec![];
            while let Some(pop) = open.pop() {
                for chunk in &chunk_pairs {
                    if let Some(index) = chunk.chars().position(|i| i == pop) {
                        if index == 0 {
                            autocomplete.push(chunk.chars().nth(1).unwrap());
                        }
                    }
                }
            }
            let mut line_sum: i64 = 0;
            for c in autocomplete {
                line_sum *= 5;
                line_sum += autocomplete_score.get(&c).unwrap();
            }
            p2.push(line_sum);
        });
    println!("Puzzle 1: {}", p1);

    p2.sort();

    let p2 = p2[p2.len() / 2];
    println!("Puzzle 2: {}", p2);
}
