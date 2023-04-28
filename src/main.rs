#![allow(dead_code)]
#![allow(unused_variables)]

mod player;

use crate::player::Player;
use crate::player::Side;

fn main() {
    let mut player_1: Player = Player::new();
    let mut player_2: Player = Player::new();
    print_state(player_1, player_2);

    player_2.is_hit(Side::Left, player_1.get_hand(Side::Right));
    print_state(player_1, player_2);

    player_1.is_hit(Side::Right, player_2.get_hand(Side::Left));
    print_state(player_1, player_2);
}

fn print_state(player1: Player, player2: Player) {
    println!("{:?}", player1);
    println!("{:?}", player2);
    println!();
}
