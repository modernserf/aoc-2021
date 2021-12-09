use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let seq = sections[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let boards = sections
        .iter()
        .skip(1)
        .map(|board| {
            let mut map = HashMap::new();
            for (row, col_str) in board.split("\n").enumerate() {
                for (col, cell) in col_str.split_whitespace().enumerate() {
                    let value = cell.parse::<usize>().unwrap();
                    map.insert(value, (row, col));
                }
            }
            return map;
        })
        .collect::<Vec<_>>();

    let scored_boards = boards
        .iter()
        .filter_map(|board| score_board(&seq, &board))
        .collect::<Vec<_>>();

    let first_winner = scored_boards
        .iter()
        .min_by(|l, r| l.win_turn.cmp(&r.win_turn))
        .unwrap();

    let last_winner = scored_boards
        .iter()
        .max_by(|l, r| l.win_turn.cmp(&r.win_turn))
        .unwrap();

    println!("Part 1: {}", first_winner.score);
    println!("Part 2: {}", last_winner.score);
}

struct BingoResult {
    score: usize,
    win_turn: usize,
}

type BingoBoard = HashMap<usize, (usize, usize)>;

fn score_board(seq: &[usize], board: &BingoBoard) -> Option<BingoResult> {
    let mut rows = vec![0; 5];
    let mut cols = vec![0; 5];
    let mut remaining = board.keys().cloned().collect::<HashSet<usize>>();

    for turn in 0..seq.len() {
        let val = seq[turn];
        if let Some((row, col)) = board.get(&val) {
            remaining.remove(&val);
            rows[*row] += 1;
            cols[*col] += 1;
            if rows[*row] == 5 || cols[*col] == 5 {
                let sum = remaining.iter().sum::<usize>();

                return Some(BingoResult {
                    score: sum * val,
                    win_turn: turn,
                });
            }
        }
    }

    None
}
