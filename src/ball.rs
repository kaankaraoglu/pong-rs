use ggez::glam::Vec2;
use ggez::graphics::{Color, Mesh};

pub struct Ball {
    pub position: Vec2,
    pub mesh: Mesh,
    pub speed: f32,
    pub direction: Vec2,
}

impl Ball {
    pub const RADIUS: f32 = 12.0;
    pub const MESH_TOLERANCE: f32 = 0.2;
    pub const COLOR: Color = Color::RED;
}
