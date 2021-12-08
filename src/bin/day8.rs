fn contains(v: &Vec<char>, code: &str) -> Vec<char> {
    v.iter()
        .filter(|&&c| code.contains(c))
        .map(|c| c.to_owned())
        .collect()
}

fn filter(v: &Vec<char>, code: &str) -> Vec<char> {
    v.iter()
        .filter(|&&c| !code.contains(c))
        .map(|c| c.to_owned())
        .collect()
}

fn main() {
    let lines: Vec<(Vec<String>, Vec<String>)> = advent_of_code::read_and_parse("inputs/day8.txt")
        .iter()
        .map(|line: &String| {
            let mut r = line
                .split(" | ")
                .map(|p: &str| p.split(" ").map(|a| a.to_owned()).collect::<Vec<String>>());
            (r.next().unwrap(), r.next().unwrap())
        })
        .collect();

    let mut p1 = 0;
    for (_, code) in &lines {
        for c in code {
            p1 += match c.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }
    println!("Puzzle 1: {}", p1);

    /*
    ----0----
    1       2
    ----3----
    4       5
    ----6----
    */
    let mut p2 = 0;
    for line in lines {
        let mut ans = vec![vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']; 7];
        let mut iter = line.0.iter().cycle();
        'inner: while let Some(code) = iter.next() {
            match code.len() {
                // 1
                2 => {
                    for x in 0..7 {
                        if x == 2 || x == 5 {
                            ans[x] = contains(&ans[x], code);
                        } else {
                            ans[x] = filter(&ans[x], code);
                        }
                    }
                }
                // 7
                3 => {
                    ans[2] = contains(&ans[2], code);
                    ans[5] = contains(&ans[5], code);
                    if ans[2].len() == 2 {
                        let code = code.replace(ans[2][0], "");
                        let code = code.replace(ans[2][1], "");
                        ans[0] = contains(&ans[0], &code);
                    }
                }
                // 4
                4 => {
                    for x in 0..7 {
                        if x == 1 || x == 2 || x == 3 || x == 5 {
                            ans[x] = contains(&ans[x], code);
                        } else {
                            ans[x] = filter(&ans[x], code);
                        }
                    }
                }
                6 => {
                    if ans[0].len() > 1 {
                        continue;
                    }
                    // 0
                    if ans[1].len() == 2
                        && ans[1].iter().filter(|&&curr| code.contains(curr)).count() == 1
                    {
                        ans[1] = contains(&ans[1], code);
                    }
                    // 6
                    if ans[5].len() == 2
                        && ans[5].iter().filter(|&&curr| code.contains(curr)).count() == 1
                    {
                        ans[5] = contains(&ans[5], code);
                    }
                    // 9
                    if ans[6].len() == 2
                        && ans[6].iter().filter(|&&curr| code.contains(curr)).count() == 1
                    {
                        ans[6] = contains(&ans[6], code);
                    }
                }
                _ => (),
            };
            for i in 0..ans.len() {
                if ans[i].len() == 1 {
                    for j in 0..ans.len() {
                        if j != i {
                            ans[j] = filter(&ans[j], &ans[i][0].to_string());
                        }
                    }
                }
            }
            if ans.iter().all(|a| a.len() == 1) {
                break 'inner;
            }
        }
        let ans: Vec<&char> = ans.iter().flatten().collect();

        let s = line
            .1
            .iter()
            .map(|l| {
                let mut l = l
                    .chars()
                    .map(|c| ans.iter().position(|&&a| a == c).unwrap() as i32)
                    .collect::<Vec<i32>>();
                l.sort();
                let l = l
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                l
            })
            .map(|l| match l.as_str() {
                "012456" => 0,
                "25" => 1,
                "02346" => 2,
                "02356" => 3,
                "1235" => 4,
                "01356" => 5,
                "013456" => 6,
                "025" => 7,
                "0123456" => 8,
                "012356" => 9,
                _ => unimplemented!(),
            })
            .fold(String::new(), |sum, i| sum + &i.to_string());
        p2 += s.parse::<i32>().unwrap();
    }
    println!("Puzzle 2: {}", p2);
}
