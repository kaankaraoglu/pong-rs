use ggez::glam::Vec2;
use ggez::graphics;

pub struct Paddle {
    pub position: Vec2,
    pub mesh: graphics::Mesh,
    pub speed: f32,
}

impl Paddle {
    pub const WIDTH: f32 = 15.0;
    pub const HEIGHT: f32 = 100.0;
}
