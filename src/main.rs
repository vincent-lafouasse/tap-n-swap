#![allow(dead_code)]
#![allow(unused_variables)]

mod player;

use player::hit;
use player::Player;
use player::Side;

fn main() {
    let mut player: Player = Player::new("JR");
    let mut opponent: Player = Player::new("The World");
    print_state(player, opponent);

    hit(player, &mut opponent, Side::Right, Side::Left);
    print_state(player, opponent);

    hit(opponent, &mut player, Side::Left, Side::Right);
    print_state(player, opponent);
}

fn print_state(player: Player, opponent: Player) {
    println!("{opponent}");
    println!("{player}");
    println!();
}
