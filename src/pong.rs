extern crate ggez;

use ggez::event::EventHandler;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color, DrawParam, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{graphics, timer, Context, GameError, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::input::InputState;
use crate::paddle::Paddle;
use crate::utils::{handle_ball_movement, handle_collisions, handle_player_input, load_resources};

pub struct Pong {
    frames: usize,
    fps: f64,
    assets: Assets,
    ball: Ball,
    player_paddle: Paddle,
    opponent_paddle: Paddle,
    input: InputState,
}

impl Pong {
    pub fn new(ctx: &mut Context) -> GameResult<Pong> {
        let (width, height) = ctx.gfx.drawable_size();
        let screen_center_vertical = height / 2.0;
        let assets = Assets::new().expect("Failed to load assets!");

        load_resources(ctx)?;

        // Create the ball's mesh
        let ball_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            Ball::RADIUS,
            Ball::MESH_TOLERANCE,
            Ball::COLOR,
        )?;

        // Create the ball
        let ball = Ball {
            position: vec2(width / 2.0, height / 2.0),
            mesh: ball_mesh,
            speed: 4.5,
            direction: vec2(-1.0, 0.75),
        };

        // Create the paddles' mesh
        let paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Paddle::BOUNDS,
            Paddle::COLOR,
        )?;

        // Create player's paddle
        let player_paddle = Paddle {
            position: vec2(
                Paddle::STARTING_POSITION_X_OFFSET,
                screen_center_vertical - Paddle::HEIGHT / 2.0,
            ),
            mesh: paddle_mesh.clone(),
            speed: Paddle::SPEED,
        };

        // Create opponent's paddle
        let opponent_paddle = Paddle {
            position: vec2(
                width - Paddle::STARTING_POSITION_X_OFFSET - Paddle::WIDTH,
                screen_center_vertical - Paddle::HEIGHT / 2.0,
            ),
            mesh: paddle_mesh,
            speed: Paddle::SPEED,
        };

        Ok(Pong {
            frames: 0,
            fps: 0.0,
            assets,
            ball,
            player_paddle,
            opponent_paddle,
            input: Default::default(),
        })
    }

    fn draw_ball(&mut self, canvas: &mut Canvas) {
        canvas.draw(
            &self.ball.mesh,
            Vec2::new(self.ball.position.x, self.ball.position.y),
        );
    }

    fn draw_player_paddle(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.player_paddle.mesh, self.player_paddle.position);
    }

    fn draw_opponent_paddle(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.opponent_paddle.mesh, self.opponent_paddle.position);
    }

    fn draw_fps_counter(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        self.frames += 1;
        if (self.frames % 25) == 0 {
            self.fps = ctx.time.fps().floor();
        }

        let fps_string = format!("FPS:{}", self.fps);
        let fps_counter_position = vec2(100.0, 10.0);

        canvas.draw(
            Text::new(fps_string)
                .set_font("LiberationMono")
                .set_scale(48.),
            DrawParam::from(fps_counter_position).color(Color::WHITE),
        );
    }
}

impl EventHandler<GameError> for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 60;

        // https://gameprogrammingpatterns.com/game-loop.html#do-you-own-the-game-loop,-or-does-the-platform
        while ctx.time.check_update_time(TARGET_FPS) {
            // For dev, move the opponent the same way the player moves.
            self.opponent_paddle.position.y = self.player_paddle.position.y;

            handle_player_input(ctx, &mut self.player_paddle, &self.input);
            handle_ball_movement(&mut self.ball);
            handle_collisions(
                ctx,
                &mut self.ball,
                &mut self.player_paddle,
                &mut self.opponent_paddle,
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        self.draw_ball(&mut canvas);
        self.draw_player_paddle(&mut canvas);
        self.draw_opponent_paddle(&mut canvas);
        self.draw_fps_counter(ctx, &mut canvas);

        // Render!
        canvas.finish(ctx)?;

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
