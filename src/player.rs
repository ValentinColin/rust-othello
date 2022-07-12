//! A module that describe players

use std::fmt;

use crate::board::Piece;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Player {
    pub(crate) name: &'static str,
    pub(crate) piece: Piece,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.piece)
    }
}
