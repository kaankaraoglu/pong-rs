mod game;
mod input;

extern crate ggez;

use crate::game::pong::Pong;
use crate::game::utils;
use ggez::event::run;
use ggez::{ContextBuilder, GameResult};

pub fn main() -> GameResult {
    let context_builder = ContextBuilder::new("pong-rs", "Kaan Karaoglu")
        .add_resource_path(utils::get_resource_directory());

    // Create the context and the event loop
    let (mut ctx, event_loop) = context_builder.build()?;

    // Create the state of the game
    let game = Pong::new(&mut ctx)?;

    // Run the main loop
    run(ctx, event_loop, game)
}
