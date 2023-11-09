use std::path::PathBuf;
use std::{env, path};

use ggez::glam::vec2;
use ggez::graphics::FontData;
use ggez::{Context, GameError};

use crate::game::ball::Ball;
use crate::game::paddle::Paddle;
use crate::game::player::Player;
use crate::input::input_state::InputState;

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
    player_one_paddle: &mut Paddle,
    player_two_paddle: &mut Paddle,
    input: &InputState,
) {
    let (_drawable_width, drawable_height) = ctx.gfx.drawable_size();

    // PLAYER ONE PADDLE MOVEMENTS.
    if input.key_w {
        player_one_paddle.position.y = f32::max(player_one_paddle.position.y - Paddle::SPEED, 0.0);
    }

    if input.key_s {
        player_one_paddle.position.y = f32::min(
            player_one_paddle.position.y + Paddle::SPEED,
            drawable_height - Paddle::HEIGHT,
        );
    }

    // PLAYER TWO PADDLE MOVEMENTS.
    if input.up {
        player_two_paddle.position.y = f32::max(player_two_paddle.position.y - Paddle::SPEED, 0.0);
    }

    if input.down {
        player_two_paddle.position.y = f32::min(
            player_two_paddle.position.y + Paddle::SPEED,
            drawable_height - Paddle::HEIGHT,
        );
    }
}

pub fn handle_collisions(
    ctx: &mut Context,
    ball: &mut Ball,
    player_one: &mut Player,
    player_two: &mut Player,
    turn_active: &mut bool,
    game_over: &mut bool,
) {
    // HANDLE PLAYER ONE PADDLE COLLISIONS.
    if check_paddle_to_ball_collision(&mut player_one.paddle, ball) {
        handle_paddle_collision(&mut player_one.paddle, ball, -1.0);
    }

    // HANDLE PLAYER TWO PADDLE COLLISIONS.
    if check_paddle_to_ball_collision(&mut player_two.paddle, ball) {
        handle_paddle_collision(&mut player_two.paddle, ball, 1.0);
    }

    // HANDLE THE CASE WHERE THE BALL HITS FLOOR OR THE CEILING.
    handle_floor_and_ceiling_collisions(ctx, ball);

    // HANDLE THE CASE WHERE THE BALL HITS THE SIDE WALLS.
    handle_side_wall_collisions(ctx, ball, player_one, player_two, turn_active, game_over);
}

fn handle_side_wall_collisions(
    ctx: &mut Context,
    ball: &mut Ball,
    player_one: &mut Player,
    player_two: &mut Player,
    turn_active: &mut bool,
    game_over: &mut bool,
) {
    let (width, _height) = ctx.gfx.drawable_size();

    if ball.position.x + Ball::RADIUS >= width {
        player_one.scored = true;
        process_win(player_one, player_two, ctx, ball, turn_active, game_over);
    } else if ball.position.x - Ball::RADIUS <= 0.0 {
        player_two.scored = true;
        process_win(player_one, player_two, ctx, ball, turn_active, game_over);
    }
}

fn check_paddle_to_ball_collision(paddle: &mut Paddle, ball: &mut Ball) -> bool {
    let mut is_colliding = false;
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
        is_colliding = true;
    }

    is_colliding
}

fn handle_paddle_collision(paddle: &mut Paddle, ball: &mut Ball, top_hit_angle_inversion: f32) {
    let relative_intersection_y =
        paddle.position.y + Paddle::HEIGHT / 2.0 - ball.position.y - Ball::RADIUS;

    let normalized_relative_intersection_y = relative_intersection_y / (Paddle::HEIGHT / 2.0);
    let bounce_angle = f32::to_radians(normalized_relative_intersection_y * -60.0);

    ball.direction.x = f32::cos(bounce_angle) * -1.0 * top_hit_angle_inversion;
    ball.direction.y = f32::sin(bounce_angle);
}

fn handle_floor_and_ceiling_collisions(ctx: &mut Context, ball: &mut Ball) {
    let (_width, height) = ctx.gfx.drawable_size();
    if ball.position.y >= height || ball.position.y <= 0.0 {
        ball.direction.y *= -1.0;
    }
}

fn restart_round(
    ctx: &mut Context,
    ball: &mut Ball,
    player_one: &mut Player,
    player_two: &mut Player,
    turn_active: &mut bool,
) {
    let (width, height) = ctx.gfx.drawable_size();

    // DEACTIVATE THE TURN
    *turn_active = false;

    // RESET THE PADDLE POSITIONS.
    player_one.paddle.position = vec2(
        Paddle::STARTING_POSITION_X_OFFSET,
        height / 2.0 - Paddle::HEIGHT / 2.0,
    );

    player_two.paddle.position = vec2(
        width - Paddle::STARTING_POSITION_X_OFFSET - Paddle::WIDTH,
        height / 2.0 - Paddle::HEIGHT / 2.0,
    );

    // RESET THE BALL POSITION ACCORDING TO THIS TURN'S WINNER.
    if player_one.scored {
        ball.position = vec2(
            Paddle::STARTING_POSITION_X_OFFSET + Paddle::WIDTH + Ball::RADIUS,
            height / 2.0,
        );
        ball.direction = vec2(1.0, 0.0);
    }

    if player_two.scored {
        ball.position = vec2(
            width - Paddle::STARTING_POSITION_X_OFFSET - Paddle::WIDTH - Ball::RADIUS,
            height / 2.0,
        );
        ball.direction = vec2(-1.0, 0.0);
    }

    player_one.scored = false;
    player_two.scored = false;
}

fn process_win(
    player_one: &mut Player,
    player_two: &mut Player,
    ctx: &mut Context,
    ball: &mut Ball,
    turn_active: &mut bool,
    game_over: &mut bool,
) {
    if player_one.scored {
        if player_two.life > 0 {
            player_two.decrease_life();
            restart_round(ctx, ball, player_one, player_two, turn_active);
        } else {
            *turn_active = false;
            *game_over = true;
        }
    } else if player_two.scored {
        if player_one.life > 0 {
            player_one.decrease_life();
            restart_round(ctx, ball, player_one, player_two, turn_active);
        } else {
            *turn_active = false;
            *game_over = true;
        }
    }
}
