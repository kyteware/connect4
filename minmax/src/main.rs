use std::{collections::BTreeMap, io::{stdin, stdout, Write}};

use connect4::{Color, Position, Slot};

fn main() {
    let mut board = Position::default();
    let mut input = stdin().lines();

    loop {
        print!("{}", board.stringify());
        stdout().flush().unwrap();
        let col = input.next().unwrap().unwrap().parse::<usize>().unwrap();

        board = board.with_move(Color::Red, col);
        let mut dupes = BTreeMap::new();
        let bot_mve = minmax(&board, Color::Yellow, 6, &mut dupes).0;
        let mut hist: BTreeMap<u64, u32> = BTreeMap::new();
        for cnt in dupes.values() {
            if let Some(count) = hist.get_mut(cnt) {
                *count += 1;
            } else {
                hist.insert(*cnt, 1);
            }
        }
        println!("{:?}", hist);
        let num_loners = hist[&1];
        let num_reps =  hist.iter().map(|(x, y)| *x as u32 * y).sum::<u32>() - num_loners;
        println!("{} positions considered, {} avoidable {}% possible savings", num_loners, num_reps, (num_reps as f64 / (num_loners as f64 + num_reps as f64)) * 100.);
        board = board.with_move(Color::Yellow, bot_mve);
    }
}

fn minmax(board: &Position, turn: Color, depth: u32, dupes: &mut BTreeMap<u64, u64>) -> (usize, f64) {
    if let Some(count) = dupes.get_mut(&board.inner) {
        *count += 1;
    } else {
        dupes.insert(board.inner, 1);
    }
    let mut best = (69, std::f64::NEG_INFINITY * turn.favour_mod());
    for mve in board.allowed_moves() {
        let new_board = board.with_move(turn, mve);
        let favour = new_board.naive_score().as_f64();
        let score = if depth > 0 {
            if favour == std::f64::INFINITY * turn.favour_mod() {
                return (mve, favour)
            }
            minmax(&board.with_move(turn, mve), turn.opposite(), depth - 1, dupes).1
        } else {
            favour
        };

        let should_replace = if turn == Color::Red { score > best.1 } else { score < best.1 };

        if should_replace {
            best = (mve, score);
        }
    }
    best
}

