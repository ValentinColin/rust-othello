use crate::board::Board;
use crate::player::Player;


struct Othello {
    board: Board,
    player_black: Player,
    player_white: Player,
}

impl Othello {
    pub fn new(player_black: Player, player_white: Player) -> Self {
        let board = Board::new().init();
        Othello {board, player_black, player_white}
    }
}
