extern crate ggez;

use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{event, graphics, Context, GameError, GameResult};

use ball::Ball;
use paddle::Paddle;

pub struct Pong {
    frames: usize,
    ball: Ball,
    player_paddle: Paddle,
    opponent_paddle: Paddle,
}

impl Pong {
    pub fn new(ctx: &mut Context) -> GameResult<Pong> {
        const PLAYER_PADDLE_STARTING_POSITION_X_OFFSET: f32 = 20.0;
        let (width, height) = ctx.gfx.drawable_size();

        // Create the ball
        let ball = Ball {
            position: vec2(50.0, 50.0),
            mesh: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::ZERO,
                Ball::RADIUS,
                0.2,
                Color::RED,
            )?,
            speed: 4.5,
            direction: vec2(1.0, 0.75),
        };

        // Create player's paddle

        let player_paddle = Paddle {
            position: vec2(PLAYER_PADDLE_STARTING_POSITION_X_OFFSET, height / 2.0),
            mesh: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Paddle::BOUNDS,
                Paddle::COLOR,
            )?,
            speed: Paddle::SPEED,
        };

        // Create opponent's paddle
        let opponent_paddle = Paddle {
            position: vec2(
                width - Paddle::WIDTH - PLAYER_PADDLE_STARTING_POSITION_X_OFFSET,
                height / 20.0,
            ),
            mesh: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Paddle::BOUNDS,
                Paddle::COLOR,
            )?,
            speed: Paddle::SPEED,
        };

        Ok(Pong {
            frames: 0,
            ball,
            player_paddle,
            opponent_paddle,
        })
    }
}

impl event::EventHandler<GameError> for Pong {
    // TODO: MAKE THE BALL REACT TO THE PADDLE.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let (width, height) = _ctx.gfx.drawable_size();

        self.ball.position.x += self.ball.direction.x * self.ball.speed;
        self.ball.position.y += self.ball.direction.y * self.ball.speed;

        if self.ball.position.x + Ball::RADIUS >= width
            || self.ball.position.x - Ball::RADIUS <= 0.0
        {
            self.ball.direction.x *= -1.0;
        }

        if self.ball.position.y >= height || self.ball.position.y <= 0.0 {
            self.ball.direction.y *= -1.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw the ball
        canvas.draw(
            &self.ball.mesh,
            Vec2::new(self.ball.position.x, self.ball.position.y),
        );

        // Draw player's paddle
        canvas.draw(&self.player_paddle.mesh, self.player_paddle.position);

        // Draw opponent's paddle
        canvas.draw(&self.opponent_paddle.mesh, self.opponent_paddle.position);

        canvas.finish(ctx)?;

        // Count FPS and log.
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps().ceil());
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        let (_screen_width, screen_height) = ctx.gfx.drawable_size();

        match input.keycode {
            Some(KeyCode::Up) => {
                let mut next_pos = vec2(
                    0.0,
                    self.player_paddle.position.y - self.player_paddle.speed,
                );

                if next_pos.y <= 0.0 {
                    next_pos.y = 0.0 + Paddle::DEFAULT_X_OFFSET
                }

                self.player_paddle.position.y = next_pos.y;
            }
            Some(KeyCode::Down) => {
                let mut next_pos = vec2(
                    0.0,
                    self.player_paddle.position.y + self.player_paddle.speed,
                );

                if next_pos.y + Paddle::HEIGHT >= screen_height {
                    next_pos.y = screen_height - Paddle::HEIGHT - Paddle::DEFAULT_X_OFFSET
                }

                self.player_paddle.position.y = next_pos.y;
            }
            Some(KeyCode::Escape) => {
                ctx.request_quit();
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
}
