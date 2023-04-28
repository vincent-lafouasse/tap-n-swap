#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    static PLAYER_1: Player = Player::new();
    static PLAYER_2: Player = Player::new();
    println!("{:?}", PLAYER_1);
    println!("Player 1 has {} hands", PLAYER_1.n_hands());
    println!("{:?}", PLAYER_2);
    println!("Player 2 has {} hands", PLAYER_2.n_hands());
}

enum Side {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    left: usize,
    right: usize,
}

impl Player {
    pub const fn new() -> Self {
        Player { left: 1, right: 1 }
    }

    pub fn n_hands(&self) -> usize {
        (self.left > 0) as usize + (self.right > 0) as usize
    }

    fn get_hand(&self, side: Side) -> usize {
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
