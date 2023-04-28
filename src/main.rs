#![allow(dead_code)]
#![allow(unused_variables)]

mod player;

use player::hit;
use player::Player;
use player::Side;

fn main() {
    let mut player_1: Player = Player::new("Alice");
    let mut player_2: Player = Player::new("Bob");
    print_state(player_1, player_2);

    hit(player_1, &mut player_2, Side::Right, Side::Left);
    print_state(player_1, player_2);

    hit(player_2, &mut player_1, Side::Left, Side::Right);
    print_state(player_1, player_2);
}

fn pretty_print_player(player: Player, numeral_mode: bool) {
    match numeral_mode {
        true => println!("{}\t\t{}\t\t{}", player.name, player.left, player.right),
        false => {
            println!(
                "{}\t\t{}\t\t{}",
                player.name,
                "|".repeat(player.left),
                "|".repeat(player.right),
            )
        }
    }
}

fn print_state(player1: Player, player2: Player) {
    let numeral_mode = false;
    pretty_print_player(player1, numeral_mode);
    pretty_print_player(player2, numeral_mode);
    println!();
}
