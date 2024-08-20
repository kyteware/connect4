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
        let mut cache = BTreeMap::new();
        let bot_mve = smart_minmax(&board, Color::Yellow, 10, &mut cache).0;
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
        let favour = red_favour(&new_board);
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

fn red_favour(position: &Position) -> f64 {
    let mut total_favour = 0.; // red is pos, yellow is neg

    // sideways
    for r in 0..6 {
        let mut scan = ScanState::default();
        for c in 0..7 {
            if scan.eat(position.get(r, c)) {
                break;
            }
        }
        let score = scan.score();
        match score {
            Score::Winner(Color::Red) => {
                return std::f64::INFINITY
            },
            Score::Winner(Color::Yellow) => {
                return std::f64::NEG_INFINITY
            }
            Score::Favour(favour) => {
                total_favour += favour;
            },
        }
    }

    // vertical
    for c in 0..7 {
        let mut scan = ScanState::default();
        for r in 0..6 {
            if scan.eat(position.get(r, c)) {
                break;
            }
        }
        let score = scan.score();
        match score {
            Score::Winner(Color::Red) => {
                return std::f64::INFINITY
            },
            Score::Winner(Color::Yellow) => {
                return std::f64::NEG_INFINITY
            }
            Score::Favour(favour) => {
                total_favour += favour;
            },
        }
    }

    total_favour
}

#[derive(Default)]
struct ScanState {
    pub winner: Option<Color>,
    pub favour: f64,
    slot_window: [Slot; 4],
    slot_fill_index: usize,
}

impl ScanState {
    fn eat(&mut self, slot: Slot) -> bool {
        if self.slot_fill_index < 4 {
            self.slot_window[self.slot_fill_index] = slot;
            self.slot_fill_index += 1;
            if self.slot_fill_index < 4 {
                return false;
            }
        } else {
            self.slot_window[0] = self.slot_window[1];
            self.slot_window[1] = self.slot_window[2];
            self.slot_window[2] = self.slot_window[3];
            self.slot_window[3] = slot;
        }

        let red_cnt = self.slot_window.iter().fold(0, |acc, x| acc + (x == &Slot::Color(Color::Red)) as usize);
        let ylw_cnt = self.slot_window.iter().fold(0, |acc, x| acc + (x == &Slot::Color(Color::Yellow)) as usize);

        if red_cnt == 4 {
            self.winner = Some(Color::Red);
            return true;
        }
        if ylw_cnt == 4 {
            self.winner = Some(Color::Yellow);
            return true;
        }
        if red_cnt > 0 && ylw_cnt == 0 {
            self.favour += (red_cnt.pow(3) as f64) / 27.
        } else if ylw_cnt > 0 && red_cnt == 0 {
            self.favour -= (ylw_cnt.pow(3) as f64) / 27.
        }
        false
    }

    fn score(&self) -> Score {
        if let Some(winner) = self.winner {
            Score::Winner(winner)
        } else {
            Score::Favour(self.favour)
        }
    }
}

enum Score {
    Winner(Color),
    Favour(f64)
}
