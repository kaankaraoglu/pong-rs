extern crate ggez;

use ggez::event::run;
use ggez::{ContextBuilder, GameResult};

use pong::Pong;

mod ball;
mod input;
mod paddle;
mod pong;
mod utilities;

pub fn main() -> GameResult {
    let context_builder = ContextBuilder::new(Pong::GAME_ID, Pong::AUTHOR)
        .add_resource_path(utilities::get_resource_directory());

    // Create the context and the event loop
    let (mut ctx, event_loop) = context_builder.build()?;

    // Create the state of the game
    let game = Pong::new(&mut ctx)?;

    // Run the main loop
    run(ctx, event_loop, game)
}
