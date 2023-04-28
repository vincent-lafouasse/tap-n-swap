#![allow(dead_code)]
#![allow(unused_variables)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod player;

use player::hit;
use player::Player;
use player::Side;

fn main() -> Result<(), String> {
    let mut player: Player = Player::new("JR");
    let mut opponent: Player = Player::new("The World");
    print_state(player, opponent);

    hit(player, &mut opponent, Side::Right, Side::Left);
    print_state(player, opponent);

    hit(opponent, &mut player, Side::Left, Side::Right);
    print_state(player, opponent);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Tap'n'Swap", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(74, 64, 99));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}

fn print_state(player: Player, opponent: Player) {
    println!("{opponent}");
    println!("{player}");
    println!();
}
