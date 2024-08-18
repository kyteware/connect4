#[derive(Clone, Default)]
pub struct Position {
    /// row major,
    /// rows top to bottom
    /// cols left to right 
    /// 
    /// (im not about to bike shed bit encoding this shit)
    pub grid: [[Slot; 7]; 6]
}

impl Position {
    pub fn get(&self, row: usize, col: usize) -> Slot {
        self.grid[row][col]
    }

    pub fn with_move(&self, color: Color, col: usize) -> Position {
        for row in (0..6).rev() {
            if self.get(row, col) == Slot::Empty {
                let mut new_pos = self.clone();
                new_pos.grid[row][col] = Slot::Color(color);

                return new_pos;
            }
        }

        panic!("row already full") // stfu this is incredibly idiomatic
    }

    pub fn allowed_moves(&self) -> Vec<usize> {
        let mut moves = vec![];

        for c in 0..7 {
            if self.get(0, c) == Slot::Empty {
                moves.push(c);
            }
        }

        moves
    }

    pub fn stringify(&self) -> String {
        let mut res = String::new();
        for row in &self.grid {
            for col in row.iter() {
                res.push_str(match col {
                    Slot::Empty => "(-)",
                    Slot::Color(Color::Red) => "(R)",
                    Slot::Color(Color::Yellow) => "(Y)"
                })
            }
            res.push('\n');
        }
        res
    }

    pub fn hash(&self) -> u64 {
        let mut res = 0;
        for c in 0..7 {
            let mut len = 0;
            for r in (0..6).rev() {
                match self.get(r, c) {
                    Slot::Color(Color::Red) => {
                        res <<= 1;
                        res ^= 1;
                    }
                    Slot::Color(Color::Yellow) => {
                        res <<= 1;
                    }
                    Slot::Empty => {
                        break;
                    }
                }
                len += 1;
            }
            res <<= 3 + 6 - len;
            res ^= len;
        }

        res
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Slot {
    #[default]
    Empty,
    Color(Color)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Yellow
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::Red => Color::Yellow,
            Color::Yellow => Color::Red
        }
    }

    pub fn favour_mod(self) -> f64 {
        match self {
            Color::Red => 1.,
            Color::Yellow => -1.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Position};

    #[test]
    fn test_hashes() {
        let mut board = Position::default();

        assert_eq!(board.hash(), 0);
        board = board.with_move(Color::Yellow, 0);
        assert_eq!(board.hash(), 1 << 54);
        board = board.with_move(Color::Red, 6);
        assert_eq!(board.hash(), (1 << 54) ^ (1 << 8) ^ 1);
    }
}
