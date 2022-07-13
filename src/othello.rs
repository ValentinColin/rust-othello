//! The main module that manage the game logic

use ggez::{Context, event, GameResult};
use ggez::event::{KeyCode, MouseButton};
use ggez::graphics::{self, Color, Rect};
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
    gameover: bool,
    theme: Theme,
}

impl Othello {
    /// Create a new game
    pub fn new(player_black: Player, player_white: Player) -> Self {
        Othello {
            board: Board::default(),
            player_black,
            player_white,
            current_player: player_black,
            current_move: None,
            current_player_has_played: false,
            gameover: false,
            theme: DEFAULT_THEME,
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

    /// Set the board of the game
    pub fn set_board(mut self, board: Board) -> Self {
        self.board = board;
        self
    }

    /// Set the theme of the game
    pub fn set_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }


    /// Ask if the game is over
    fn is_over(&self) -> bool {
        self.board.is_finish()
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
    fn score(&self) -> (u8, u8) {
        debug!("the score is:\n\t{}: {}\n\t{}: {}",
            self.player_black, self.board.score(self.player_black),
            self.player_white, self.board.score(self.player_white)
        );
        (self.board.score(self.player_black), self.board.score(self.player_white))
    }

    /// Draw the score on the screen
    fn draw_score(&self, ctx: &mut Context) -> GameResult {
        let (score_black, score_white) = self.score();

        // Draw the background of the popup
        let popup = graphics::MeshBuilder::new()
            .rounded_rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    SCREEN_SIZE.0 as f32 / 4.0,
                    SCREEN_SIZE.1 as f32 / 4.0,
                    SCREEN_SIZE.0 as f32 / 2.0,
                    SCREEN_SIZE.1 as f32 / 2.0,
                ),
                50.0,
                Color::new(0.5, 0.5, 0.5, 1.0)
            )?
            .build(ctx)?;
        graphics::draw(ctx, &popup, graphics::DrawParam::default())?;

        // Set and draw the text in the popup
        let text_format = format!("{}: {}\n\t{}: {}",
            self.player_black, score_black,
            self.player_white, score_white
        );

        let font = graphics::Font::new(ctx, self.theme.font_path)?;
        let text = graphics::Text::new((text_format.as_str(), font, self.theme.font_scale));
        let dest_point = glam::Vec2::new(300.0, 300.0);
        graphics::draw(ctx, &text, (dest_point,))?;

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for Othello {
    /// Update will happen on every frame before it is drawn.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if ! self.gameover {
            if self.is_over() {
                self.gameover = true;
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
        graphics::clear(ctx, self.theme.background_color);

        // Draw the board and his content
        self.board.draw(ctx, self.current_player.piece, self.theme)?;

        // If the game is over draw a popup to show the score
        if self.gameover {
            self.draw_score(ctx)?;
        }

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
            KeyCode::S => {debug!("SCORE from key S"); self.score();},
            _ => {}
        };
    }
}
