use std::path::PathBuf;
use std::{env, path};

use ggez::Context;

use crate::ball::Ball;
use crate::input::InputState;
use crate::paddle::Paddle;

pub fn get_resource_directory() -> PathBuf {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    }
}

pub fn handle_player_input(ctx: &Context, paddle: &mut Paddle, input: &InputState) {
    let (_drawable_width, drawable_height) = ctx.gfx.drawable_size();
    let mut next_pos = paddle.position;

    if input.up {
        next_pos.y = paddle.position.y - paddle.speed;
        if next_pos.y <= 0.0 {
            next_pos.y = 0.0;
        }
    }

    if input.down {
        next_pos.y = paddle.position.y + paddle.speed;

        if next_pos.y + Paddle::HEIGHT > drawable_height {
            next_pos.y = drawable_height - Paddle::HEIGHT
        }
    }

    paddle.position.y = next_pos.y;
}

pub fn handle_ball_movement(ctx: &mut Context, ball: &mut Ball) {
    let (width, height) = ctx.gfx.drawable_size();

    ball.position.x += ball.direction.x * ball.speed;
    ball.position.y += ball.direction.y * ball.speed;

    if ball.position.x + Ball::RADIUS >= width || ball.position.x - Ball::RADIUS <= 0.0 {
        ball.direction.x *= -1.0;
    }

    if ball.position.y >= height || ball.position.y <= 0.0 {
        ball.direction.y *= -1.0;
    }
}
