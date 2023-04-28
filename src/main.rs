#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let player = Player::new();
    println!("{:?}", player);
}

#[derive(Debug)]
pub struct Player {
    left: usize,
    right: usize,
}

impl Player {
    pub fn new() -> Self {
        Player { left: 5, right: 5 }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
