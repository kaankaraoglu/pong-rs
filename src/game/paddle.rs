use ggez::glam::{vec2, Vec2};
use ggez::graphics;
use ggez::graphics::{Color, Rect};

pub struct Paddle {
    pub position: Vec2,
    pub mesh: graphics::Mesh,
}

impl Paddle {
    pub const WIDTH: f32 = 15.0;
    pub const HEIGHT: f32 = 175.0;
    pub const SPEED: f32 = 25.0;
    pub const COLOR: Color = Color::WHITE;
    pub const STARTING_POSITION_X_OFFSET: f32 = 50.0;
    pub const BOUNDS: Rect = Rect {
        x: 0.0,
        y: 0.0,
        w: Paddle::WIDTH,
        h: Paddle::HEIGHT,
    };

    pub fn get_center_position(&self) -> Vec2 {
        vec2(
            self.position.x + Paddle::WIDTH / 2.0,
            self.position.y + Paddle::HEIGHT / 2.0,
        )
    }
}
