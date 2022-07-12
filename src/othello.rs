//! The main module that manage the game logic

use ggez::{Context, event, GameResult, graphics};
use ggez::event::{KeyCode, MouseButton};
use ggez::graphics::Color;
use log::{debug, error, info};

use crate::*;

/// Main structure of the game that hold all state necessary to play
pub struct Othello {
    board: Board,
    player_black: Player,
    player_white: Player,
    current_player: Player,
    current_move: Option<GridPosition>,
    current_player_has_played: bool,
    gameover: bool
}

impl Othello {
    /// Create a new game
    pub fn new(player_black: Player, player_white: Player, board: Board) -> Self {
        Othello {
            board,
            player_black,
            player_white,
            current_player: player_white,
            current_move: None,
            current_player_has_played: false,
            gameover: false,
        }
    }

    /// Reset the game
    fn reset(&mut self) {
        self.board.reset();
        self.current_player = self.player_white;
        self.current_move = None;
        self.current_player_has_played = false;
        self.gameover = false;
    }

    /// Ask if the game is over
    fn is_over(&self) -> bool {
        self.board.is_full()
    }

    /// action to do when the user click
    fn click(&mut self, x: f32, y: f32) {
        let pos = GridPosition::from_screen(x, y);
        if pos.in_screen() {
            let grid_pos = pos.into_grid();
            if self.board.is_valid_move(grid_pos, self.current_player.piece) {
                info!("The player {} play at {}", self.current_player, grid_pos);
                // self.board.update(grid_pos, self.current_player);
                self.current_move = Some(grid_pos);

                // Notify that the player has played.
                // We do it here because we may have a menu on the side,
                // so players will could click without playing
                self.current_player_has_played = true;
            } else {
                info!("The position {} is not valid for {}", grid_pos, self.current_player)
            }
        } else {
            error!("The user click cannot be out of the screen");
        }
    }

    /// Return the next player who have to play
    fn next_player(&mut self) -> Player {
        match self.current_player.piece {
            Piece::BLACK => self.player_white,
            Piece::WHITE => self.player_black,
            Piece::EMPTY => panic!("A player cannot have the Piece::EMPTY"),
        }
    }

    /// Draw the score on the screen
    fn scores(&self) -> (u8, u8) {
        debug!("the score is:\n\t{}: {}\n\t{}: {}",
            self.player_black, self.board.score(self.player_black),
            self.player_white, self.board.score(self.player_white)
        );
        (self.board.score(self.player_black), self.board.score(self.player_white))
    }

    /// Draw the score on the screen
    fn draw_score(&self, _ctx: &mut Context) -> GameResult {
        // todo!("Draw a rect that print the scores")
        #[allow(unused)]
        let (score_black, score_white) = self.scores();
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for Othello {
    /// Update will happen on every frame before it is drawn.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ! self.gameover {
            if self.is_over() {
                self.gameover = true;
                self.draw_score(ctx)?;
            } else if self.current_player_has_played {
                self.board.update(
                    self.current_move.unwrap(),
                    self.current_player.piece,
                );
                self.current_player_has_played = false;
                self.current_player = self.next_player();
            } else if !self.board.can_play(self.current_player.piece) {
                self.current_player = self.next_player();
            }
        }
        Ok(())
    }

    /// Render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // First we clear the screen and set the background color
        graphics::clear(ctx, Color::GREEN);

        // Draw the board and his content
        self.board.draw(ctx, self.current_player.piece)?;

        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx)?;

        // And return success.
        Ok(())
    }

    /// Called every time a mouse button gets pressed
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        if button == MouseButton::Left {
            self.click(x, y);
        }
    }

    /// Called every time a key gets pressed
    /// Inputs are managed here
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => {info!("EXIT from key Escape"); event::quit(ctx);},
            KeyCode::R => {debug!("RESET from key R"); self.reset();},
            KeyCode::S => {debug!("SCORE from key S"); self.scores();},
            _ => {}
        };
    }
}
