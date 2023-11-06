extern crate ggez;

use ggez::event::run;
use ggez::{ContextBuilder, GameResult};

use pong::Pong;

mod ball;
mod input;
mod paddle;
mod pong;
mod utils;

pub fn main() -> GameResult {
    let context_builder = ContextBuilder::new("pong-rust", "Kaan Karaoglu")
        .add_resource_path(utils::get_resource_directory());

    // Create the context and the event loop
    let (mut ctx, event_loop) = context_builder.build()?;

    // Create the state of the game
    let game = Pong::new(&mut ctx)?;

    // Run the main loop
    run(ctx, event_loop, game)
}
