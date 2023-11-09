use crate::game::paddle::Paddle;

pub struct Player {
    pub paddle: Paddle,
    pub life: i8,
    pub scored: bool,
}

impl Player {
    pub const STARTING_LIFE_COUNT: i8 = 5;

    pub const fn new(paddle: Paddle) -> Self {
        Self {
            paddle,
            life: Player::STARTING_LIFE_COUNT,
            scored: false,
        }
    }

    pub fn decrease_life(&mut self) {
        self.life -= 1;
    }
}
