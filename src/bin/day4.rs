type BoardNumber = (i32, bool);
type BoardRow = [BoardNumber; 5];
type Board = [BoardRow; 5];

fn split_line_to_numbers(line: String, delimiter: &str) -> Vec<i32> {
    line.split(delimiter).map(|n| n.parse().unwrap()).collect()
}

fn check_board(board: Board) -> Option<BoardRow> {
    for i in 0..5 {
        if board[i].iter().all(|n| n.1) {
            return Some(board[i]);
        } else if board.iter().all(|r| r[i].1) {
            return Some(
                board.iter().map(|r| r[i]).collect::<Vec<BoardNumber>>()[0..5]
                    .try_into()
                    .unwrap(),
            );
        }
    }
    None
}

#[test]
fn test_check_board() {
    assert_eq!(
        check_board([
            [(0, false), (0, true), (0, false), (0, false), (0, false)],
            [(0, false), (1, true), (0, false), (0, false), (0, false)],
            [(0, false), (2, true), (0, false), (0, false), (0, false)],
            [(0, false), (3, true), (0, false), (0, false), (0, false)],
            [(0, false), (4, true), (0, false), (0, false), (0, false)],
        ]),
        Some([(0, true), (1, true), (2, true), (3, true), (4, true),])
    )
}

fn main() {
    let mut lines: Vec<String> = advent_of_code::read_and_parse("inputs/day4.txt");
    let numbers: Vec<i32> = split_line_to_numbers(lines.remove(0), ",");
    lines.remove(0);

    let mut boards: Vec<Board> = (0..lines.len())
        .step_by(6)
        .map(|index| {
            let to_array = |line: String| -> BoardRow {
                line.split_whitespace()
                    .map(|n| (n.parse::<i32>().unwrap(), false))
                    .collect::<Vec<BoardNumber>>()[0..5]
                    .try_into()
                    .unwrap()
            };
            [
                to_array(lines[index].clone()),
                to_array(lines[index + 1].clone()),
                to_array(lines[index + 2].clone()),
                to_array(lines[index + 3].clone()),
                to_array(lines[index + 4].clone()),
            ]
        })
        .collect();

    let mut found: Option<Board> = None;
    let mut trigger: Option<i32> = None;
    'a: for number in &numbers {
        for board in &mut boards {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j].0 == *number {
                        board[i][j].1 = true;
                        if let Some(_) = check_board(*board) {
                            found = Some(*board);
                            trigger = Some(*number);
                            break 'a;
                        }
                    }
                }
            }
        }
    }
    let found = found.unwrap();
    let trigger = trigger.unwrap();
    let sum = found.iter().fold(0, |sum, row| {
        sum + row
            .iter()
            .fold(0, |s, n| if n.1 == false { s + n.0 } else { s })
    });
    println!("Puzzle 1: {}", sum * trigger);
}
