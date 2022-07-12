//! Example of a specific case of othello

use ggez::{event, GameResult};
use env_logger;
use grid::*;

use rust_othello::*;


const B: Piece = Piece::BLACK;
const W: Piece = Piece::WHITE;
const E: Piece = Piece::EMPTY;


fn main() -> GameResult {
    // Init the logger
    env_logger::init();

    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let context_builder = ggez::ContextBuilder::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_AUTHORS"))
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default()
            .title("Othello Example 1")
        )
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default()
                         .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        );

    // And finally we attempt to build the context and create the window. If it fails, we panic with the message
    // "Failed to build ggez context"
    let (ctx, event_loop) = context_builder.build().expect("Failed to build ggez context");

    // Create our board for example
    let grid = grid![[E,B,B,B,B,B,B,E]
                                [B,B,B,B,B,B,B,B]
                                [B,B,B,B,B,B,B,B]
                                [B,B,B,B,B,B,B,B]
                                [B,B,B,W,B,B,B,B]
                                [B,B,B,B,B,B,B,B]
                                [B,B,B,B,B,B,B,B]
                                [E,B,B,B,B,B,B,E]];
    let board = Board::set_board(grid);

    // Next we create a new instance of our Game struct, which implements EventHandler
    let state = Othello::new(PLAYER_ONE, PLAYER_TWO, board);

    // And finally we actually run our game, passing in our context, event_loop and state.
    event::run(ctx, event_loop, state)
}
