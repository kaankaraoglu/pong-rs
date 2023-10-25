extern crate ggez;

use std::path::PathBuf;
use std::{env, path};

use ggez::context::HasMut;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::Color;
use ggez::{event, graphics, Context, GameError, GameResult};
use graphics::Canvas;

struct MainState {
    frames: usize,
    ball: Ball,
}

struct Ball {
    position: Vec2,
    mesh: graphics::Mesh,
    speed: f32,
    direction: Vec2,
}

impl Ball {
    const RADIUS: f32 = 10.0;
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let ball = Ball {
            position: vec2(50.0, 50.0),
            mesh: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(0.0, 0.0),
                Ball::RADIUS,
                0.1,
                Color::RED,
            )?,
            speed: 4.5,
            direction: vec2(1.0, 0.75),
        };

        Ok(MainState { frames: 0, ball })
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let (width, height) = _ctx.gfx.drawable_size();

        self.ball.position.x += self.ball.direction.x * self.ball.speed;
        self.ball.position.y += self.ball.direction.y * self.ball.speed;

        if self.ball.position.x + Ball::RADIUS >= width
            || self.ball.position.x - Ball::RADIUS <= 0.0
        {
            self.ball.direction.x = self.ball.direction.x * -1.0;
        }

        if self.ball.position.y >= height || self.ball.position.y <= 0.0 {
            self.ball.direction.y = self.ball.direction.y * -1.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(
            &self.ball.mesh,
            Vec2::new(self.ball.position.x, self.ball.position.y),
        );
        canvas.finish(ctx)?;

        // Count FPS and log.
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps().ceil());
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong-rust", "Kaan Karaoglu")
        .add_resource_path(get_resource_directory());

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

fn get_resource_directory() -> PathBuf {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    resource_dir
}
