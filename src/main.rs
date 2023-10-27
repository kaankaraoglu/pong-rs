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

    let state = Pong::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
