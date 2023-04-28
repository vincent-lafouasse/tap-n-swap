#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut player_1: Player = Player::new();
    let mut player_2: Player = Player::new();
    print_state(&player_1, &player_2);

    player_2.is_hit(Side::Left, player_1.get_hand(Side::Right));
    print_state(&player_1, &player_2);

    player_1.is_hit(Side::Right, player_2.get_hand(Side::Left));
    print_state(&player_1, &player_2);
}

fn print_state(player1: &Player, player2: &Player) {
    println!("{:?}", player1);
    println!("{:?}", player2);
    println!();
}

pub enum Side {
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
