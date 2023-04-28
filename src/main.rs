#![allow(dead_code)]
#![allow(unused_variables)]

mod player;

use player::hit;
use player::Player;
use player::Side;

fn main() {
    let mut player_1: Player = Player::new();
    let mut player_2: Player = Player::new();
    print_state(player_1, player_2);

    hit(player_1, &mut player_2, Side::Right, Side::Left);
    print_state(player_1, player_2);

    hit(player_2, &mut player_1, Side::Left, Side::Right);
    print_state(player_1, player_2);
}

fn print_state(player1: Player, player2: Player) {
    println!("{:?}", player1);
    println!("{:?}", player2);
    println!();
}
