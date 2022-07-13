//! A module that describe players

use std::fmt;

use crate::board::Piece;

/// Struct for manage player
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Player {
    pub name: &'static str,
    pub piece: Piece,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.piece)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Piece, Player};

    #[test]
    fn fmt_player() {
        let player_one = Player { name: "Valentin Colin", piece: Piece::WHITE };
        let player_two = Player { name: "Colin Valentin", piece: Piece::BLACK };

        assert_eq!("Valentin Colin (WHITE)", format!("{}", player_one).as_str());
        assert_eq!("Colin Valentin (BLACK)", format!("{}", player_two).as_str());
    }
}
