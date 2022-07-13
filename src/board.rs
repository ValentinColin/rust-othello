//! The board module that manage all interaction with the board

// std crates
use std::cmp::{max, min};
use std::fmt;

// extern crates
use ggez::{graphics, Context, GameResult};
use glam::Vec2;
use grid::Grid;
use log::{debug, error};

// intern crates
use crate::*;


/// Describe a case of the Board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    BLACK,
    WHITE,
    EMPTY,
}

impl Piece {
    /// Return the other color for self
    ///
    /// - BLACK => WHITE
    /// - WHITE => BLACK
    /// - EMPTY => EMPTY (a log error is send)
    pub fn next(&self) -> Self {
        match self {
            Piece::BLACK => Piece::WHITE,
            Piece::WHITE => Piece::BLACK,
            Piece::EMPTY => {
                error!("You must not use next() on Piece::EMPTY");
                Piece::EMPTY
            }
        }
    }

    /// Draw a Piece of the Board at a certain point
    ///
    /// - Piece::BLACK => Render by a `black` circle of 80% of the GRID_CELL_SIZE
    /// - Piece::WHITE => Render by a `white` circle of 80% of the GRID_CELL_SIZE
    /// - Piece::EMPTY => Not drawn
    fn draw<P>(&self, ctx: &mut Context, theme: Theme, point: P) -> GameResult
    where
        P: Into<mint::Point2<f32>>,
    {
        let color = match self {
            Piece::BLACK => theme.piece_colors.0,
            Piece::WHITE => theme.piece_colors.1,
            Piece::EMPTY => return Ok(()),
        };
        let mesh = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                point,
                2.0 * min(GRID_CELL_SIZE.0, GRID_CELL_SIZE.1) as f32 / 5.0,
                1.0,
                color,
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::EMPTY
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::BLACK => write!(f, "BLACK"),
            Piece::WHITE => write!(f, "WHITE"),
            Piece::EMPTY => write!(f, "EMPTY"),
        }
    }
}

pub struct Board {
    board: Grid<Piece>,
    //historic: Vec<GridPosition>,
    //theme: Theme,
}

impl Board {
    /// Create a new Board
    pub fn new() -> Self {
        Board {
            board: Grid::new(GRID_SIZE.1 as usize, GRID_SIZE.0 as usize),
        }
    }

    /// Initialize the board for a new game
    ///
    /// Set the 4 case in the middle of the board
    pub fn init(&mut self) {
        let mid_x = self.board.cols() / 2 - 1;
        let mid_y = self.board.rows() / 2 - 1;
        self.board[mid_x][mid_y + 1] = Piece::BLACK;
        self.board[mid_x + 1][mid_y] = Piece::BLACK;
        self.board[mid_x][mid_y] = Piece::WHITE;
        self.board[mid_x + 1][mid_y + 1] = Piece::WHITE;
    }

    /// Reset the content of the board
    /// i.e. clear then init the board
    pub fn reset(&mut self) {
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                self.set(GridPosition::new(x, y), Piece::default())
            }
        }
        self.init();
    }

    /// Manually set the board
    ///
    /// ```
    /// use grid::*;
    /// use rust_othello::{Board, Piece};
    ///
    /// const B: Piece = Piece::BLACK;
    /// const W: Piece = Piece::WHITE;
    /// const E: Piece = Piece::EMPTY;
    ///
    /// let grid = grid![[E,B,B,B,B,B,B,E]
    ///                  [B,B,B,B,B,B,B,B]
    ///                  [B,B,B,B,B,B,B,B]
    ///                  [B,B,B,B,B,B,B,B]
    ///                  [B,B,B,W,B,B,B,B]
    ///                  [B,B,B,B,B,B,B,B]
    ///                  [B,B,B,B,B,B,B,B]
    ///                  [E,B,B,B,B,B,B,E]];
    /// let board = Board::set_board(grid);
    /// ```
    pub fn set_board(board: Grid<Piece>) -> Self {
        Board { board }
    }

    /// Set a specific case of the board
    pub fn set(&mut self, position: GridPosition, piece: Piece) {
        self.board[position.y as usize][position.x as usize] = piece;
    }

    /// Get a specific case of the board
    pub fn get(&self, position: GridPosition) -> Piece {
        self.board[position.y as usize][position.x as usize]
    }

    /// Return the line from the start position to the bound of the board
    fn get_line(&self, start: GridPosition, direction: GridPosition) -> Vec<GridPosition> {
        let last_position = self.get_end_line(start, direction);
        let mut current_position = start;
        let mut line = Vec::with_capacity(max(GRID_SIZE.0, GRID_SIZE.1) as usize);
        while current_position != last_position {
            current_position += direction;
            line.push(current_position);
        }
        line
    }

    /// Get the last position in a direction from a start
    ///
    /// Assume that the position is in the board and direction have coord in \[-1, 0, 1\]
    fn get_end_line(&self, position: GridPosition, direction: GridPosition) -> GridPosition {
        if !(-1..=1).contains(&(direction.x as i32)) || !(-1..=1).contains(&(direction.y as i32)) {
            error!("This value is not a direction: {}", direction);
        }
        let mut final_position = position;
        while self.in_board(final_position) {
            final_position += direction;
        }
        final_position - direction
    }

    /// Update the board without any verification
    pub fn update(&mut self, position: GridPosition, player_piece: Piece) {
        let enemy_piece = player_piece.next();
        let mut directions_to_eat = Vec::new();
        let mut new_piece;

        // For all direction
        for direction in Board::get_directions() {
            // If it is a possible direction
            if !self.in_board(GridPosition::new(
                position.x + direction.x,
                position.y + direction.y,
            )) {
                continue;
            }

            // We check if an enemy pawn is next to the position of the potential move.
            new_piece = self.get(GridPosition::new(
                position.x + direction.x,
                position.y + direction.y,
            ));
            if new_piece != enemy_piece {
                continue;
            }

            // For all position in a line
            for position in self.get_line(position, direction).iter() {
                new_piece = self.get(*position);
                if new_piece == Piece::EMPTY {
                    break;
                } else if new_piece == player_piece {
                    directions_to_eat.push(direction);
                    break;
                }
            }
        }
        self.set(position, player_piece);
        self.eat_by_directions(position, directions_to_eat, player_piece);
    }

    /// eat lines using the eat_by_direction() method
    fn eat_by_directions(
        &mut self,
        position: GridPosition,
        directions: Vec<GridPosition>,
        player_piece: Piece,
    ) {
        for direction in directions.iter() {
            self.eat_by_direction(position, *direction, player_piece);
        }
    }

    /// Eat a line (see the [rule](https://documentation.help/Reversi-Rules/rules.htm))
    fn eat_by_direction(
        &mut self,
        position: GridPosition,
        direction: GridPosition,
        player_piece: Piece,
    ) {
        let mut new_piece;
        for current_position in self.get_line(position, direction).iter() {
            new_piece = self.get(*current_position);
            if new_piece == player_piece {
                break;
            }
            self.set(*current_position, new_piece.next())
        }
    }

    /// Verify if this move is valid for a specific player
    pub fn is_valid_move(&self, position: GridPosition, player_piece: Piece) -> bool {
        let mut is_valid_move = false;
        let mut new_piece;
        let enemy_piece = player_piece.next();

        // If the initial position is empty
        if self.get(position) == Piece::EMPTY {
            // For all direction
            'outer: for direction in Board::get_directions() {
                // If it is a possible direction
                if !self.in_board(GridPosition::new(
                    position.x + direction.x,
                    position.y + direction.y,
                )) {
                    continue;
                }

                // We check if an enemy pawn is next to the position of the potential move.
                new_piece = self.get(GridPosition::new(
                    position.x + direction.x,
                    position.y + direction.y,
                ));
                if new_piece != enemy_piece {
                    continue;
                }

                // For all position in a line
                for position in self.get_line(position, direction).iter() {
                    new_piece = self.get(*position);
                    if new_piece == Piece::EMPTY {
                        break;
                    } else if new_piece == player_piece {
                        is_valid_move = true;
                        break 'outer;
                    }
                }
            }
        } else {
            debug!("Position not empty: {}", position);
        }
        is_valid_move
    }

    /// Compute and return all valid moves for a player
    pub fn get_valid_moves(&self, player_piece: Piece) -> Vec<GridPosition> {
        let mut valid_moves = Vec::new();
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                if self.get(GridPosition::new(x, y)) == Piece::EMPTY {
                    let position = GridPosition::new(x, y);
                    if self.is_valid_move(position, player_piece) {
                        valid_moves.push(position);
                    }
                }
            }
        }
        valid_moves
    }

    /// Return a vector of unit directions
    /// ```txt
    /// 1 2 3
    /// 4 . 5
    /// 6 7 8
    /// ```
    fn get_directions() -> Vec<GridPosition> {
        let mut directions = Vec::with_capacity(8);
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                directions.push(GridPosition::new(x, y));
            }
        }
        directions
    }

    /// Verify if the board can't be upgrade (i.e the game is finish)
    pub fn is_finish(&self) -> bool {
        !self.can_play(Piece::BLACK) && !self.can_play(Piece::WHITE)
    }

    /// Verify if a player can play with the actual board
    pub fn can_play(&self, player_piece: Piece) -> bool {
        self.get_valid_moves(player_piece).len() > 0
    }

    /// Verify if a position is in the board
    fn in_board(&self, position: GridPosition) -> bool {
        0 <= position.x
            && position.x < self.board.cols() as i16
            && 0 <= position.y
            && position.y < self.board.rows() as i16
    }

    /// Verify if the case is empty
    pub fn is_empty(&self, position: GridPosition) -> bool {
        self.get(position) == Piece::EMPTY
    }

    /// Verify if the board is full (no Piece::EMPTY)
    pub fn is_full(&self) -> bool {
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                if self.is_empty(GridPosition::new(x, y)) {
                    return false;
                }
            }
        }
        return true;
    }

    /// Compute the score of a player
    pub fn score(&self, player: Player) -> u8 {
        let mut score = 0;
        let color: Piece = player.piece;
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                if self.get(GridPosition::new(x, y)) == color {
                    score += 1;
                }
            }
        }
        score
    }

    /// Draw all the board (grid + pieces + valid moves)
    pub fn draw(&self, ctx: &mut Context, player_piece: Piece, theme: Theme) -> GameResult {
        Board::draw_empty_board(ctx, theme)?;
        self.draw_content_board(ctx, theme)?;
        if theme.valid_moves_color.is_some() {
            self.draw_valid_move(ctx, theme, player_piece)?;
        }
        Ok(())
    }

    /// Draw an empty board (only the grid)
    fn draw_empty_board(ctx: &mut Context, theme: Theme) -> GameResult {
        // horizontal line
        for y in 1..GRID_SIZE.1 {
            let mesh = graphics::MeshBuilder::new()
                .line(
                    &[
                        Vec2::new(0.0, (y * GRID_CELL_SIZE.1) as f32),
                        Vec2::new(
                            (GRID_SIZE.0 * GRID_CELL_SIZE.0) as f32,
                            (y * GRID_CELL_SIZE.1) as f32,
                        ),
                    ],
                    2.0,
                    theme.grid_color,
                )?
                .build(ctx)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        // vertical line
        for x in 1..GRID_SIZE.0 {
            let mesh = graphics::MeshBuilder::new()
                .line(
                    &[
                        Vec2::new((x * GRID_CELL_SIZE.0) as f32, 0.0),
                        Vec2::new(
                            (x * GRID_CELL_SIZE.0) as f32,
                            (GRID_SIZE.1 * GRID_CELL_SIZE.1) as f32,
                        ),
                    ],
                    2.0,
                    theme.grid_color,
                )?
                .build(ctx)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        Ok(())
    }

    /// Draw the content of the board (only pieces)
    fn draw_content_board(&self, ctx: &mut Context, theme: Theme) -> GameResult {
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                if self.board[y as usize][x as usize] == Piece::EMPTY {
                    continue;
                }
                let center = Vec2::new(
                    (x * GRID_CELL_SIZE.0) as f32 + GRID_CELL_SIZE.0 as f32 / 2.0,
                    (y * GRID_CELL_SIZE.1) as f32 + GRID_CELL_SIZE.1 as f32 / 2.0,
                );
                self.board[y as usize][x as usize].draw(ctx, theme, center)?;
            }
        }
        Ok(())
    }

    /// Draw all the valid move for the current player
    fn draw_valid_move(&self, ctx: &mut Context, theme: Theme, player_piece: Piece) -> GameResult {
        let mut center;
        for position in self.get_valid_moves(player_piece).iter() {
            center = Vec2::new(
                (position.x * GRID_CELL_SIZE.0) as f32 + GRID_CELL_SIZE.0 as f32 / 2.0,
                (position.y * GRID_CELL_SIZE.1) as f32 + GRID_CELL_SIZE.1 as f32 / 2.0,
            );
            let mesh = graphics::MeshBuilder::new()
                .circle(
                    graphics::DrawMode::fill(),
                    center,
                    3.0 * min(GRID_CELL_SIZE.0, GRID_CELL_SIZE.1) as f32 / 10.0,
                    1.0,
                    theme.valid_moves_color.unwrap(),
                )?
                .build(ctx)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();
        board.init();
        board
    }
}

#[cfg(test)]
mod tests_piece {
    use crate::Piece;

    #[test]
    fn fmt() {
        let piece_black = Piece::BLACK;
        let piece_white = Piece::WHITE;
        let piece_empty = Piece::EMPTY;

        assert_eq!("BLACK", format!("{}", piece_black));
        assert_eq!("WHITE", format!("{}", piece_white));
        assert_eq!("EMPTY", format!("{}", piece_empty));
    }

    #[test]
    fn default() {
        assert_eq!(Piece::default(), Piece::EMPTY);
    }
}

#[cfg(test)]
mod tests_board {
    use crate::{Board, GridPosition, Piece};
    use grid::grid;

    #[test]
    fn in_board() {
        const E: Piece = Piece::EMPTY;
        let grid = grid![[E,E]
                                    [E,E]];
        let board = Board::set_board(grid);

        // assert true
        assert!(board.in_board(GridPosition::new(0, 0)));
        assert!(board.in_board(GridPosition::new(0, 1)));
        assert!(board.in_board(GridPosition::new(1, 0)));
        assert!(board.in_board(GridPosition::new(1, 1)));

        // assert false (see the '!')
        assert!(!board.in_board(GridPosition::new(-1, 0)));
        assert!(!board.in_board(GridPosition::new(0, -1)));
        assert!(!board.in_board(GridPosition::new(-1, -1)));
        assert!(!board.in_board(GridPosition::new(0, 2)));
        assert!(!board.in_board(GridPosition::new(2, 0)));
        assert!(!board.in_board(GridPosition::new(2, 2)));
    }
}
