use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::{Color, Rect};

pub struct Paddle {
    pub position: Vec2,
    pub mesh: graphics::Mesh,
    pub speed: f32,
}

impl Paddle {
    pub const WIDTH: f32 = 15.0;
    pub const HEIGHT: f32 = 100.0;
    pub const DEFAULT_X_OFFSET: f32 = 15.0;
    pub const SPEED: f32 = 400.0;
    pub const COLOR: Color = Color::WHITE;
    pub const BOUNDS: Rect = Rect {
        x: 0.0,
        y: 0.0,
        w: Paddle::WIDTH,
        h: Paddle::HEIGHT,
    };
}
