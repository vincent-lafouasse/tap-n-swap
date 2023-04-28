use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub name: &'static str,
    pub left: usize,
    pub right: usize,
}

pub enum Side {
    Left,
    Right,
}

pub fn hit(player: Player, opponent: &mut Player, from: Side, to: Side) {
    let amount: usize = player.get_hand(from);
    opponent.is_hit(to, amount);
}

impl Player {
    pub const fn new(name: &'static str) -> Self {
        Player {
            left: 1,
            right: 1,
            name,
        }
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
        Self {
            left: 1,
            right: 1,
            name: "JR",
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width: usize = 12;
        write!(
            f,
            "{:width$}\t{}\t{}",
            self.name,
            "|".repeat(self.left),
            "|".repeat(self.right),
            width = width,
        )
    }
}
