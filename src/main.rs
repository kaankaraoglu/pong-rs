extern crate ggez;

use ggez::{event, ContextBuilder, GameResult};

use pong::Pong;
use utils::get_resource_directory;

mod ball;
mod paddle;
mod pong;
mod utils;

pub fn main() -> GameResult {
    let context_builder = ContextBuilder::new("pong-rust", "Kaan Karaoglu")
        .add_resource_path(get_resource_directory());

    // Create the context and event loop
    let (mut ctx, event_loop) = context_builder.build()?;

    // Create the state of the game
    let game = Pong::new(&mut ctx)?;

    // Run the main loop
    event::run(ctx, event_loop, game)
}
