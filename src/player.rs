use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub name: &'static str,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn other(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

pub fn hit(player: Player, opponent: &mut Player, from: Side, to: Side) {
    let amount: usize = player.get_hand(from);
    opponent.is_hit(to, amount);
    println!(
        "{} hit {}'s {to} hand with their {from} hand",
        player.name, opponent.name
    );
    println!("{amount} was given");
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

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width: usize = 12;
        write!(
            f,
            "{}",
            match self {
                Side::Left => "Left",
                Side::Right => "Right",
            }
        )
    }
}
