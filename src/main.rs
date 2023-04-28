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

fn print_state(player: Player, opponent: Player) {
    let numeral_mode = false;
    pretty_print_player(opponent, numeral_mode);
    pretty_print_player(player, numeral_mode);
    println!();
}
