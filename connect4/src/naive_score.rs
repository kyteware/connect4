use crate::{Color, Position, Slot};

impl Position {
    pub fn naive_score(&self) -> NaiveScore {
        let mut total_favour = 0.; // red is pos, yellow is neg

        // sideways
        for r in 0..6 {
            let mut scan = ScanState::default();
            for c in 0..7 {
                if scan.eat(self.get(r, c)) {
                    break;
                }
            }
            let score = scan.naive_score();
            if let NaiveScore::RedFavour(favour) = score {
                total_favour += favour;
            } else {
                return score;
            }
        }

        // vertical
        for c in 0..7 {
            let mut scan = ScanState::default();
            for r in 0..6 {
                if scan.eat(self.get(r, c)) {
                    break;
                }
            }
            let score = scan.naive_score();
            if let NaiveScore::RedFavour(favour) = score {
                total_favour += favour;
            } else {
                return score;
            }
        }

        // right-running-diag
        let starts = (3isize..6).map(|r| (r, 0)).into_iter().chain((1isize..4).map(|c| (5, c)));
        for mut pos in starts {
            let mut scan = ScanState::default();
            while pos.0 >= 0 && pos.1 <= 7 {
                if scan.eat(self.get(pos.0 as usize, pos.1 as usize)) {
                    break;
                }
                pos.0 -= 1;
                pos.1 += 1;
            }
            let score = scan.naive_score();
            if let NaiveScore::RedFavour(favour) = score {
                total_favour += favour;
            } else {
                return score;
            }
        }

        // left-running-diag
        let starts = (3isize..6).map(|r| (r, 6)).into_iter().chain((3isize..6).map(|c| (5, c)));
        for mut pos in starts {
            let mut scan = ScanState::default();
            while pos.0 >= 0 && pos.1 >= 0 {
                if scan.eat(self.get(pos.0 as usize, pos.1 as usize)) {
                    break;
                }
                pos.0 -= 1;
                pos.1 -= 1;
            }
            let score = scan.naive_score();
            if let NaiveScore::RedFavour(favour) = score {
                total_favour += favour;
            } else {
                return score;
            }
        }

        NaiveScore::RedFavour(total_favour)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NaiveScore {
    Winner(Color),
    RedFavour(f64)
}

impl NaiveScore {
    pub fn as_f64(self) -> f64 {
        match self {
            NaiveScore::Winner(winner) => std::f64::INFINITY * winner.favour_mod(),
            NaiveScore::RedFavour(favour) => favour,
        }
    }
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

    fn naive_score(&self) -> NaiveScore {
        if let Some(winner) = self.winner {
            NaiveScore::Winner(winner)
        } else {
            NaiveScore::RedFavour(self.favour)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{naive_score::NaiveScore, Position, Color::*};

    #[test]
    fn even() {
        assert_eq!(Position::default().naive_score(), NaiveScore::RedFavour(0.));
    }

    #[test]
    fn sideways_win() {
        let pos = Position::default()
            .with_move(Red, 3)
            .with_move(Red, 4)
            .with_move(Red, 5)
            .with_move(Red, 6);

        assert_eq!(pos.naive_score(), NaiveScore::Winner(Red));
    }

    #[test]
    fn vert_win() {
        let pos = Position::default()
            .with_move(Yellow, 2)
            .with_move(Yellow, 2)
            .with_move(Yellow, 2)
            .with_move(Yellow, 2);

        assert_eq!(pos.naive_score(), NaiveScore::Winner(Yellow));
    }

    #[test]
    fn right_diag_win() {
        let pos = Position::default()
            .with_move(Yellow, 2)
            .with_move(Yellow, 2)
            .with_move(Yellow, 2)
            .with_move(Red, 2)
            .with_move(Yellow, 3)
            .with_move(Yellow, 3)
            .with_move(Red, 3)
            .with_move(Yellow, 4)
            .with_move(Red, 4)
            .with_move(Red, 5);

        assert_eq!(pos.naive_score(), NaiveScore::Winner(Red));
    }

    #[test]
    fn left_diag_win() {
        let pos = Position::default()
            .with_move(Yellow, 6)
            .with_move(Yellow, 6)
            .with_move(Yellow, 6)
            .with_move(Red, 6)
            .with_move(Yellow, 5)
            .with_move(Yellow, 5)
            .with_move(Red, 5)
            .with_move(Yellow, 4)
            .with_move(Red, 4)
            .with_move(Red, 3);

        assert_eq!(pos.naive_score(), NaiveScore::Winner(Red));
    }
}
