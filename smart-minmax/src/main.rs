use std::{collections::BTreeMap, io::{stdin, stdout, Write}};

use connect4::{Color, Position};

fn main() {
    let mut board = Position::default();
    let mut input = stdin().lines();

    loop {
        print!("{}", board.stringify());
        stdout().flush().unwrap();
        let col = input.next().unwrap().unwrap().parse::<usize>().unwrap() - 1;
        board = board.with_move(Color::Red, col);
        let mut cache = BTreeMap::new();
        let bot_mve = smart_minmax(&board, Color::Yellow, 8, &mut cache).0;
        board = board.with_move(Color::Yellow, bot_mve);
    }
}

fn smart_minmax(board: &Position, turn: Color, depth: u32, cache: &mut BTreeMap<Position, (usize, f64)>) -> (usize, f64) {
    if let Some(best) = cache.get(&board) {
        return *best;
    }
    let mut best = (69, std::f64::NEG_INFINITY * turn.favour_mod());
    for mve in board.allowed_moves() {
        let new_board = board.with_move(turn, mve);
        let favour = new_board.naive_score().as_f64();
        let score = if depth > 0 {
            if favour == std::f64::INFINITY * turn.favour_mod() {
                return (mve, favour)
            }
            smart_minmax(&board.with_move(turn, mve), turn.opposite(), depth - 1, cache).1
        } else {
            favour
        };

        let should_replace = if turn == Color::Red { score > best.1 } else { score < best.1 };

        if should_replace {
            best = (mve, score);
        }
    }
    cache.insert(board.clone(), best);

    best
}

