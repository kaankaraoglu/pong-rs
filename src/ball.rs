use ggez::glam::Vec2;
use ggez::graphics;

pub struct Ball {
    pub position: Vec2,
    pub mesh: graphics::Mesh,
    pub speed: f32,
    pub direction: Vec2,
}

impl Ball {
    pub const RADIUS: f32 = 10.0;
}
