extern crate ggez;

use ggez::event::EventHandler;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{timer, Context, GameError, GameResult};

use crate::game::ball::Ball;
use crate::game::paddle::Paddle;
use crate::game::player::Player;
use crate::game::utils::{handle_collisions, handle_inputs, load_resources};
use crate::input::input_state::InputState;

pub struct Pong {
    input: InputState,
    ball: Ball,
    turn_active: bool,
    player_one: Player, // PLAYER ON THE LEFT.
    player_two: Player, // PLAYER ON THE RIGHT.
    game_over: bool,
    first_run: bool,
}

impl Pong {
    pub fn new(ctx: &mut Context) -> GameResult<Pong> {
        let (width, height) = ctx.gfx.drawable_size();
        let screen_center_y = height / 2.0;

        load_resources(ctx)?;

        // Create the paddles' mesh
        let paddle_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), Paddle::BOUNDS, Paddle::COLOR)?;

        // Create player's paddle
        let player_one_paddle = Paddle {
            mesh: paddle_mesh.clone(),
            position: vec2(
                Paddle::STARTING_POSITION_X_OFFSET,
                screen_center_y - Paddle::HEIGHT / 2.0,
            ),
        };

        // Create second player's paddle
        let player_two_paddle = Paddle {
            mesh: paddle_mesh,
            position: vec2(
                width - Paddle::STARTING_POSITION_X_OFFSET - Paddle::WIDTH,
                screen_center_y - Paddle::HEIGHT / 2.0,
            ),
        };

        // Create the ball's mesh
        let ball_mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::ZERO,
            Ball::RADIUS,
            Ball::MESH_TOLERANCE,
            Ball::COLOR,
        )?;

        // Create the ball
        let ball = Ball {
            mesh: ball_mesh,
            direction: vec2(1.0, 0.0),
            position: vec2(
                player_one_paddle.position.x + Paddle::WIDTH + Ball::RADIUS,
                player_one_paddle.position.y + Paddle::HEIGHT / 2.0,
            ),
        };

        Ok(Pong {
            ball,
            input: Default::default(),
            player_one: Player::new(player_one_paddle),
            player_two: Player::new(player_two_paddle),
            turn_active: false,
            game_over: false,
            first_run: true,
        })
    }

    fn draw_ball(&mut self, canvas: &mut Canvas) {
        canvas.draw(
            &self.ball.mesh,
            Vec2::new(self.ball.position.x, self.ball.position.y),
        );
    }

    fn draw_paddles(&mut self, canvas: &mut Canvas) {
        canvas.draw(
            &self.player_one.paddle.mesh,
            self.player_one.paddle.position,
        );
        canvas.draw(
            &self.player_two.paddle.mesh,
            self.player_two.paddle.position,
        );
    }

    fn draw_scoreboard(&mut self, canvas: &mut Canvas) {
        let player_one_score_string = format!("Player 1 score: {}", self.player_one.life);
        let player_one_score_position = vec2(25.0, 10.0);

        canvas.draw(
            Text::new(player_one_score_string)
                .set_font("LiberationMono")
                .set_scale(48.),
            DrawParam::from(player_one_score_position).color(Color::WHITE),
        );

        let player_two_score_string = format!("Player 2 score: {}", self.player_two.life);
        let player_two_score_position = vec2(25.0, 60.0);

        canvas.draw(
            Text::new(player_two_score_string)
                .set_font("LiberationMono")
                .set_scale(48.),
            DrawParam::from(player_two_score_position).color(Color::WHITE),
        );
    }

    fn draw_game_over(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut game_over_string = "Game Over! ".to_string();

        if self.player_two.life <= 0 {
            game_over_string += "Player #1 wins!"
        } else if self.player_one.life <= 0 {
            game_over_string += "Player #2 wins!";
        }

        game_over_string += "\nPress Space key to start again!";

        let mut game_over_text = Text::new(game_over_string);
        let game_over_text = game_over_text.set_font("LiberationMono").set_scale(64.);
        let game_over_text_dimensions = game_over_text.dimensions(ctx);

        let (width, height) = ctx.gfx.drawable_size();
        let game_over_text_position = vec2(
            width / 2.0 - game_over_text_dimensions.unwrap().w / 2.0,
            height / 2.0 - game_over_text_dimensions.unwrap().h / 2.0,
        );

        canvas.draw(
            game_over_text,
            DrawParam::from(game_over_text_position).color(Color::WHITE),
        );
    }

    fn draw_menu(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let menu_string =
            "Welcome to PONG-RS!\nPlayer #1 (Left) moves with W and S\nPlayer #2 (Right) \
            moves with Up and Down Arrows"
                .to_string();

        let mut menu_text = Text::new(menu_string);
        let menu_text = menu_text.set_font("LiberationMono").set_scale(64.);
        let menu_text_dimensions = menu_text.dimensions(ctx);

        let (width, height) = ctx.gfx.drawable_size();
        let menu_text_position = vec2(
            width / 2.0 - menu_text_dimensions.unwrap().w / 2.0,
            height / 2.0 - menu_text_dimensions.unwrap().h / 2.0,
        );

        canvas.draw(
            menu_text,
            DrawParam::from(menu_text_position).color(Color::WHITE),
        );
    }
}

impl EventHandler<GameError> for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 60;

        // https://gameprogrammingpatterns.com/game-loop.html#do-you-own-the-game-loop,-or-does-the-platform
        while ctx.time.check_update_time(TARGET_FPS) && !self.game_over {
            if !self.game_over && !self.turn_active && self.input.key_space {
                self.turn_active = true;
            }

            if self.first_run && self.input.key_space {
                self.first_run = false;
            }

            handle_inputs(
                ctx,
                &mut self.player_one.paddle,
                &mut self.player_two.paddle,
                &self.input,
            );

            if self.turn_active {
                self.ball.move_one_step();
            }

            handle_collisions(
                ctx,
                &mut self.ball,
                &mut self.player_one,
                &mut self.player_two,
                &mut self.turn_active,
                &mut self.game_over,
            );
        }

        if self.game_over && self.input.key_space {
            self.game_over = false;
            self.turn_active = true;
            self.player_one.life = Player::STARTING_LIFE_COUNT;
            self.player_two.life = Player::STARTING_LIFE_COUNT;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Alternative frame color: graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        if self.first_run {
            self.draw_menu(ctx, &mut canvas);
        }

        if self.game_over {
            self.draw_game_over(ctx, &mut canvas);
        }

        self.draw_ball(&mut canvas);
        self.draw_paddles(&mut canvas);
        self.draw_scoreboard(&mut canvas);

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
            Some(KeyCode::Space) => {
                self.input.key_space = true;
            }
            Some(KeyCode::Escape) => {
                ctx.request_quit();
            }
            _ => (),
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        // When any key is lifted, we set that key to false,
        // because we want paddles to stop moving.
        match input.keycode {
            Some(KeyCode::Up) => {
                self.input.up = false;
            }
            Some(KeyCode::Down) => {
                self.input.down = false;
            }
            Some(KeyCode::W) => {
                self.input.key_w = false;
            }
            Some(KeyCode::S) => {
                self.input.key_s = false;
            }
            Some(KeyCode::Space) => {
                self.input.key_space = false;
            }
            _ => (),
        }
        Ok(())
    }
}
