/// 012345len
/// c6,c5...c0
#[derive(Clone, Default)]
pub struct Position {
    pub inner: u64
}

impl Position {
    pub fn get(&self, row: usize, col: usize) -> Slot {
        let col_data = self.inner >> (9 * col);
        let len_mask = 7;
        let len = (col_data & len_mask) as usize;

        if row >= len {
            Slot::Empty
        } else {
            let bit = (col_data >> (8 - row)) & 1;
            match bit {
                0 => Slot::Color(Color::Yellow),
                1 => Slot::Color(Color::Red),
                _ => { panic!("bitshift mishap") }
            }
        }
    }

    pub fn with_move(&self, color: Color, col: usize) -> Position {
        let col_data = self.inner >> (9 * col);
        let len_mask = 7;
        let len = col_data & len_mask;

        if len < 6 {
            let new_len = len + 1;
            let color_bit = (color == Color::Red) as u64;
            let mask = !((7 | (1 << 9 - new_len)) << col * 9);
            let edit = (new_len | (color_bit << (9 - new_len))) << col * 9;
            
            let new_inner = (self.inner & mask) | edit;
            Position { inner: new_inner }

        } else {
            panic!("row already full") // stfu this is incredibly idiomatic
        }
    }

    pub fn allowed_moves(&self) -> Vec<usize> {
        let mut moves = vec![];

        for c in 0..7 {
            if self.get(5, c) == Slot::Empty {
                moves.push(c);
            }
        }

        moves
    }

    pub fn stringify(&self) -> String {
        let mut res = String::new();
        for r in (0..6).rev() {
            for c in 0..7 {
                let slot = self.get(r, c);
                res.push_str(match slot {
                    Slot::Empty => "(-)",
                    Slot::Color(Color::Red) => "(R)",
                    Slot::Color(Color::Yellow) => "(Y)"
                })
            }
            res.push('\n');
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
    use crate::{Color, Position, Slot};

    #[test]
    fn test_encoding() {
        let mut board = Position::default();

        assert_eq!(board.inner, 0);
        board = board.with_move(Color::Yellow, 0);
        assert_eq!(board.inner, 1);
        board = board.with_move(Color::Red, 1);
        assert_eq!(board.inner, (1) | (1 << 9) | (1 << 17));
    }

    #[test]
    fn test_getting() {
        use Color::*;
        let board = Position::default().with_move(Red, 0).with_move(Yellow, 0).with_move(Red, 0).with_move(Yellow, 3).with_move(Red, 3);

        assert_eq!(board.get(0, 0), Slot::Color(Red));
        assert_eq!(board.get(0, 3), Slot::Color(Yellow));
        assert_eq!(board.get(1, 0), Slot::Color(Yellow));
        assert_eq!(board.get(1, 3), Slot::Color(Red));
        assert_eq!(board.get(2, 0), Slot::Color(Red));
    }

    #[test]
    fn test_allowed_moves() {
        let mut board = Position::default();
        for _ in 0..6 {
            board = board.with_move(Color::Red, 1).with_move(Color::Yellow, 3).with_move(Color::Red, 6);
        }

        assert_eq!(board.allowed_moves(), vec![0,2,4,5]);

        let board = Position { inner: 0b01100000001000000001100000001100000001000000001100000001 };

        assert_eq!(board.allowed_moves(), (0..7usize).into_iter().collect::<Vec<usize>>())
    }
}
