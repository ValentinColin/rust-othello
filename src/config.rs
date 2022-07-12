//! Config file that defines every constant
//!
//! | Component | axes   |
//! |-----------|--------|
//! | foo.0     | x-axes |
//! | foo.1     | y-axes |

use crate::{Piece, Player};

/// Here we define the size of the screen (in pixel) for displaying the game
pub const SCREEN_SIZE: (u32, u32) = (800, 800);

/// Here we define the size of the grid in term of how many cells we will have
pub const GRID_SIZE: (i16, i16) = (8, 8);

/// Here we calculate the size of a cell (in pixel) in the grid
pub const GRID_CELL_SIZE: (i16, i16) = (SCREEN_SIZE.0 as i16 / GRID_SIZE.0,
                                        SCREEN_SIZE.1 as i16 / GRID_SIZE.1);

/// Define the two player who will played
pub const PLAYER_ONE: Player = Player { name: "Albert Einstein", piece: Piece::BLACK};
pub const PLAYER_TWO: Player = Player { name: "Marie Curie", piece: Piece::WHITE};
