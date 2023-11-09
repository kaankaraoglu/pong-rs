use ggez::glam::Vec2;
use ggez::graphics::{Color, Mesh};

pub struct Ball {
    pub position: Vec2,
    pub mesh: Mesh,
    pub direction: Vec2,
}

impl Ball {
    pub const SPEED: f32 = 25.0;
    pub const RADIUS: f32 = 12.0;
    pub const MESH_TOLERANCE: f32 = 0.2;
    pub const COLOR: Color = Color::RED;

    pub fn get_center_position(&self) -> Vec2 {
        self.position
    }

    pub fn move_one_step(&mut self) {
        self.position.x += self.direction.x * Ball::SPEED;
        self.position.y += self.direction.y * Ball::SPEED;
    }
}
