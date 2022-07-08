use crate::color::Color;

pub struct Player {
    name: &str,
    color: Color,
    mode: PlayerMode,
}

impl Player {
    pub fn new(name: &str, color: Color, mode: PlayerMode) -> Self {
        Player {name, color, mode}
    }
}


/// Temporary mode of player for debug
/// - User: classic player
/// - Dev: god mode that have the possibility to not respect the rule of the game
pub enum PlayerMode {
    User,
    Dev,
}
