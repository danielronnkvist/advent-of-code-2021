use std::collections::HashMap;

fn main() {
    let mut end_chunks = HashMap::new();
    end_chunks.insert(')', 3);
    end_chunks.insert(']', 57);
    end_chunks.insert('}', 1197);
    end_chunks.insert('>', 25137);

    let chunk_pairs = vec!["()", "[]", "{}", "<>"];
    let mut p1 = 0;

    advent_of_code::read_lines("inputs/day10.txt")
        .unwrap()
        .into_iter()
        .for_each(|line| {
            let line = line.unwrap();
            let mut open: Vec<char> = vec![];
            'line: for c in line.chars() {
                for chunk in &chunk_pairs {
                    if let Some(index) = chunk.chars().position(|i| i == c) {
                        if index == 0 {
                            open.push(c);
                        } else if index == 1 {
                            if let Some(pop) = open.pop() {
                                if pop.to_string() + &c.to_string() != **chunk {
                                    p1 += end_chunks.get(&c).unwrap();
                                    break 'line;
                                }
                            }
                        }
                    }
                }
            }
        });
    println!("Puzzle 1: {}", p1);
}
