//! To run this code, clone the rust_game repository and run the command:
//!
//!     cargo run --release --example music

//! This is an example of playing a music preset. For playing your own music file, please see the
//! `sound` example.

use rust_game::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();
    let msg = game.add_text(
        "msg",
        "You can play looping music presets that are included in the asset pack. For example:",
    );
    msg.translation.y = 50.0;

    let msg2 = game.add_text(
        "msg2",
        "engine.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);",
    );
    msg2.translation.y = -50.0;
    msg2.font = "font/FiraMono-Medium.ttf".to_string();

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);

    game.run(GameState {});
}
