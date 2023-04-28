#[derive(Debug, Copy, Clone)]
pub struct Player {
    left: usize,
    right: usize,
}

pub enum Side {
    Left,
    Right,
}

impl Player {
    pub const fn new() -> Self {
        Player { left: 1, right: 1 }
    }

    pub fn is_hit(&mut self, side: Side, amount: usize) {
        match side {
            Side::Left => self.left += amount,
            Side::Right => self.right += amount,
        };
        self.left = match self.left {
            0..=4 => self.left,
            _ => 0,
        };
        self.right = match self.right {
            0..=4 => self.right,
            _ => 0,
        };
    }

    pub fn n_hands(&self) -> usize {
        (self.left > 0) as usize + (self.right > 0) as usize
    }

    pub fn get_hand(&self, side: Side) -> usize {
        match side {
            Side::Left => self.left,
            Side::Right => self.right,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
