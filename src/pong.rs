extern crate ggez;

use ggez::event::EventHandler;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color, DrawParam, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{graphics, timer, Context, GameError, GameResult};

use crate::ball::Ball;
use crate::input::InputState;
use crate::paddle::Paddle;
use crate::utilities::{handle_collisions, handle_inputs, load_resources};

pub struct Pong {
    frames: usize,
    fps: f64,
    ball: Ball,
    player_paddle: Paddle,
    npc_paddle: Paddle,
    input: InputState,
}

impl Pong {
    pub const GAME_ID: &'static str = "pong-rust";
    pub const AUTHOR: &'static str = "Kaan Karaoglu";

    pub fn new(ctx: &mut Context) -> GameResult<Pong> {
        let (width, height) = ctx.gfx.drawable_size();
        let screen_center_vertical = height / 2.0;

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
            position: vec2(
                Paddle::STARTING_POSITION_X_OFFSET + Paddle::WIDTH / 2.0,
                0.0, //width / 2.0,
                     //height / 2.0 - 30.0,
            ),
            mesh: ball_mesh,
            direction: vec2(0.0, 1.0),
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
        };

        let npc_paddle = Paddle {
            position: vec2(
                width - Paddle::STARTING_POSITION_X_OFFSET - Paddle::WIDTH,
                screen_center_vertical - Paddle::HEIGHT / 2.0,
            ),
            mesh: paddle_mesh,
        };

        Ok(Pong {
            frames: 0,
            fps: 0.0,
            ball,
            player_paddle,
            npc_paddle,
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

    fn draw_npc_paddle(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.npc_paddle.mesh, self.npc_paddle.position);
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
            handle_inputs(
                ctx,
                &mut self.player_paddle,
                &mut self.npc_paddle,
                &self.input,
            );

            self.ball.move_one_step();

            handle_collisions(
                ctx,
                &mut self.ball,
                &mut self.player_paddle,
                &mut self.npc_paddle,
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        self.draw_ball(&mut canvas);
        self.draw_player_paddle(&mut canvas);
        self.draw_npc_paddle(&mut canvas);
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
            Some(KeyCode::W) => {
                self.input.key_w = true;
            }
            Some(KeyCode::S) => {
                self.input.key_s = true;
            }
            Some(KeyCode::Escape) => {
                ctx.request_quit();
            }
            _ => (), // Do nothing
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        // When any key is lifted, we set that key to false,
        // because we want paddles to stop moving.
        if let Some(KeyCode::Up | KeyCode::Down) = input.keycode {
            self.input.up = false;
            self.input.down = false;
        }
        if let Some(KeyCode::W | KeyCode::S) = input.keycode {
            self.input.key_w = false;
            self.input.key_s = false;
        }
        Ok(())
    }
}
