use std::path::PathBuf;
use std::{env, path};

use ggez::graphics::FontData;
use ggez::{Context, GameError};
use rust_raylib::collision::check_collision_circle_rect;
use rust_raylib::math::Rectangle;
use rust_raylib::math::Vector2;

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
        PathBuf::from("./resources")
    }
}

pub fn load_resources(ctx: &mut Context) -> Result<(), GameError> {
    ctx.gfx.add_font(
        "LiberationMono",
        FontData::from_path(ctx, "/font/LiberationMono-Regular.ttf")?,
    );
    Ok(())
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

pub fn handle_ball_movement(ball: &mut Ball) {
    ball.position.x += ball.direction.x * ball.speed;
    ball.position.y += ball.direction.y * ball.speed;
}

pub fn handle_collisions(
    ctx: &mut Context,
    ball: &mut Ball,
    player_paddle: &mut Paddle,
    opponent_paddle: &mut Paddle,
) {
    let (width, height) = ctx.gfx.drawable_size();

    // BALL COLLISION WITH FLOOR AND CEILING.
    if ball.position.y >= height || ball.position.y <= 0.0 {
        println!("Floor or ceiling. Ball position: {}", ball.position);
        ball.direction.y *= -1.0;
        return;
    }

    // BALL COLLISION WITH THE SIDE WALLS
    // TODO: Reset the ball to the default position. Extract Life OR end game.
    if ball.position.x + Ball::RADIUS >= width || ball.position.x - Ball::RADIUS <= 0.0 {
        ball.position.x = width / 2.0;
        ball.position.y = height / 2.0;
        println!("Side walls -- Ball position: {}", ball.position);
        ctx.request_quit()
    }

    // BALL COLLISION WITH PLAYER PADDLE.
    let player_is_colliding = check_collision_circle_rect(
        Vector2::from(ball.position),
        Ball::RADIUS,
        get_paddle_as_rectangle(player_paddle),
    );

    if player_is_colliding {
        ball.direction.x *= -1.0
    }

    // BALL COLLISION WITH OPPONENT PADDLE.
    let opponent_is_colliding = check_collision_circle_rect(
        Vector2::from(ball.position),
        Ball::RADIUS,
        get_paddle_as_rectangle(opponent_paddle),
    );

    if opponent_is_colliding {
        ball.direction.x *= -1.0
    }
}

pub fn get_paddle_as_rectangle(paddle: &mut Paddle) -> Rectangle {
    Rectangle {
        x: paddle.position.x,
        y: paddle.position.y,
        width: Paddle::WIDTH,
        height: Paddle::HEIGHT,
    }
}
