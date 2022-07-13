pub use board::{Board, Piece};
pub use config::*;
pub use grid_position::GridPosition;
pub use othello::Othello;
pub use player::Player;
pub use theme::*;

pub mod othello;
pub mod board;
pub mod player;
pub mod config;
pub mod grid_position;
pub mod theme;
