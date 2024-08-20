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
}

#[derive(Clone, Copy)]
pub enum NaiveScore {
    Winner(Color),
    RedFavour(f64)
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
            Score::Winner(winner)
        } else {
            Score::Favour(self.favour)
        }
    }
}
