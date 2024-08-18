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
