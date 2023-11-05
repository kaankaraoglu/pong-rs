use std::path::PathBuf;
use std::{env, path};

use ggez::glam::vec2;
use ggez::graphics::FontData;
use ggez::{Context, GameError};

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

pub fn handle_inputs(
    ctx: &Context,
    player_paddle: &mut Paddle,
    npc_paddle: &mut Paddle,
    input: &InputState,
) {
    let (_drawable_width, drawable_height) = ctx.gfx.drawable_size();

    // PLAYER PADDLE MOVEMENTS.
    if input.up {
        npc_paddle.position.y = f32::max(npc_paddle.position.y - Paddle::SPEED, 0.0);
    }

    if input.down {
        npc_paddle.position.y = f32::min(
            npc_paddle.position.y + Paddle::SPEED,
            drawable_height - Paddle::HEIGHT,
        );
    }

    // NPC PADDLE MOVEMENTS.
    if input.key_w {
        player_paddle.position.y = f32::max(player_paddle.position.y - Paddle::SPEED, 0.0);
    }

    if input.key_s {
        player_paddle.position.y = f32::min(
            player_paddle.position.y + Paddle::SPEED,
            drawable_height - Paddle::HEIGHT,
        );
    }
}

pub fn handle_collisions(
    ctx: &mut Context,
    ball: &mut Ball,
    player_paddle: &mut Paddle,
    npc_paddle: &mut Paddle,
) {
    // HANDLE PLAYER'S PADDLE COLLISIONS.
    if check_for_collision(ball, player_paddle) {
        handle_paddle_collision(ball, player_paddle, -1.0);
    }

    // HANDLE NPC'S PADDLE COLLISIONS.
    if check_for_collision(ball, npc_paddle) {
        handle_paddle_collision(ball, npc_paddle, 1.0);
    }

    // HANDLE THE CASE WHERE THE BALL HITS ANY EDGE OF THE SCREEN.
    handle_wall_collisions(ctx, ball);
}

fn check_for_collision(ball: &mut Ball, paddle: &mut Paddle) -> bool {
    let mut is_collision = false;
    let ball_center = ball.get_center_position();
    let paddle_center = paddle.get_center_position();
    let mut distance_between = ball_center - paddle_center;
    let clamped = vec2(
        f32::clamp(
            distance_between.x,
            Paddle::WIDTH / 2.0 * -1.0,
            Paddle::WIDTH / 2.0,
        ),
        f32::clamp(
            distance_between.y,
            Paddle::HEIGHT / 2.0 * -1.0,
            Paddle::HEIGHT / 2.0,
        ),
    );

    let closest_point = paddle_center + clamped;
    distance_between = closest_point - ball_center;
    if f32::abs(distance_between.x) < Ball::RADIUS && f32::abs(distance_between.y) < Ball::RADIUS {
        is_collision = true;
    }

    is_collision
}

fn handle_paddle_collision(ball: &mut Ball, paddle: &mut Paddle, top_hit_angle_inversion: f32) {
    let relative_intersection_y =
        paddle.position.y + Paddle::HEIGHT / 2.0 - ball.position.y - Ball::RADIUS;

    let normalized_relative_intersection_y = relative_intersection_y / (Paddle::HEIGHT / 2.0);
    let bounce_angle = f32::to_radians(normalized_relative_intersection_y * -60.0);

    ball.direction.x = f32::cos(bounce_angle) * -1.0 * top_hit_angle_inversion;
    ball.direction.y = f32::sin(bounce_angle);
}

pub fn handle_wall_collisions(ctx: &mut Context, ball: &mut Ball) {
    let (width, height) = ctx.gfx.drawable_size();

    // BALL COLLISION WITH FLOOR AND CEILING.
    if ball.position.y >= height || ball.position.y <= 0.0 {
        ball.direction.y *= -1.0;
        return;
    }

    // BALL COLLISION WITH THE SIDE WALLS
    // TODO KAAN: Reset the ball to the default position. Extract Life OR end game.
    if ball.position.x + Ball::RADIUS >= width || ball.position.x - Ball::RADIUS <= 0.0 {
        ball.position.x = width / 2.0;
        ball.position.y = height / 2.0;
        ctx.request_quit()
    }
}
