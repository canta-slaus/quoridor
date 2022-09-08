pub struct Player {
    pub x: usize,
    pub y: usize,
    pub end_y: usize,
    pub walls: usize,
}

impl Player {
    pub(crate) fn new(x: usize, y: usize, end_y: usize) -> Self {
        Self {
            x,
            y,
            end_y,
            walls: 10,
        }
    }
}
