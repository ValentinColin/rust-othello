//! The Othello game

// Good practice: use these attributes
#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]


use crate::player::{Player, PlayerMode};

mod othello;
mod player;
mod board;
mod color;

fn main() {

    let player_one = Player::new("Valentin Colin", color::WHITE, PlayerMode::USER);
    let player_two = Player::new("Angèle Lafond", color::BLACK, PlayerMode::USER);

    let mut game = Othello::new(player_one, player_two);
    game.start();
}
