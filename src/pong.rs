extern crate ggez;

use ggez::event::EventHandler;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{graphics, timer, Context, GameError, GameResult};

use crate::ball::Ball;
use crate::input::InputState;
use crate::paddle::Paddle;
use crate::utils::{handle_ball_movement, handle_player_input};

pub struct Pong {
    frames: usize,
    ball: Ball,
    player_paddle: Paddle,
    opponent_paddle: Paddle,
    input: InputState,
}

impl Pong {
    pub fn new(ctx: &mut Context) -> GameResult<Pong> {
        const PLAYER_PADDLE_STARTING_POSITION_X_OFFSET: f32 = 20.0;
        const OPPONENT_PADDLE_STARTING_POSITION_X_OFFSET: f32 = -20.0;
        let (width, height) = ctx.gfx.drawable_size();

        // Create the ball
        let ball_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            Ball::RADIUS,
            0.2,
            Color::RED,
        )?;

        let ball = Ball {
            position: vec2(50.0, 50.0),
            mesh: ball_mesh,
            speed: 15.0,
            direction: vec2(1.0, 0.75),
        };

        // Create player's paddle
        let paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Paddle::BOUNDS,
            Paddle::COLOR,
        )?;

        let player_paddle = Paddle {
            position: vec2(PLAYER_PADDLE_STARTING_POSITION_X_OFFSET, height / 2.0),
            mesh: paddle_mesh.clone(),
            speed: Paddle::SPEED,
        };

        // Create opponent's paddle
        let opponent_paddle = Paddle {
            position: vec2(
                width - Paddle::WIDTH + OPPONENT_PADDLE_STARTING_POSITION_X_OFFSET,
                height / 2.0,
            ),
            mesh: paddle_mesh,
            speed: Paddle::SPEED,
        };

        Ok(Pong {
            frames: 0,
            ball,
            player_paddle,
            opponent_paddle,
            input: Default::default(),
        })
    }
}

impl EventHandler<GameError> for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 60;

        // https://gameprogrammingpatterns.com/game-loop.html#do-you-own-the-game-loop,-or-does-the-platform
        while ctx.time.check_update_time(TARGET_FPS) {
            handle_player_input(ctx, &mut self.player_paddle, &self.input);
            handle_ball_movement(ctx, &mut self.ball);
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

        // Render!
        canvas.finish(ctx)?;

        // Count FPS and log.
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps().floor());
        }

        // Yield the timeslice.
        // This tells the OS that we're done using the CPU but it should get back to this program
        // as soon as it can. This ideally prevents the game from using 100% CPU all the time even
        // if vsync is off. The actual behavior can be a little platform-specific.
        timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.input.up = true;
            }
            Some(KeyCode::Down) => {
                self.input.down = true;
            }
            Some(KeyCode::Escape) => {
                ctx.request_quit();
            }
            _ => (), // Do nothing
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        // When the key is lifted, we set both up and down input to false because,
        // we want paddle to stop moving.
        if let Some(KeyCode::Up | KeyCode::Down) = input.keycode {
            self.input.up = false;
            self.input.down = false;
        }

        Ok(())
    }
}
