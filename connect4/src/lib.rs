pub struct Position {
    /// row major (im not about to bike shed bit encoding this shit)
    pub grid: [[Slot; 6]; 7]
}

pub enum Slot {
    Empty,
    Red,
    Yellow
}
