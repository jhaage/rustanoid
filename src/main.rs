use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams{font, font_size: 50u16, color: BLACK, ..Default::default()}
    );
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
        }
        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }

    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath,
    Strong,
    SpawnPowerup,
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        let lives = match block_type {
            BlockType::Strong => 4,
            _ => 2,
        };
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives,
            block_type,
        }
    }
    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                2 => RED,
                _ => ORANGE,
            },
            BlockType::SpawnBallOnDeath => GREEN,
            BlockType::Strong => DARKBLUE,
            BlockType::SpawnPowerup => YELLOW,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

struct Powerup {
    rect: Rect,
    vel: Vec2,
}

impl Powerup {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, 30f32, 30f32), // Powerup size
            vel: vec2(0f32, 1f32), // Falling down
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.y += self.vel.y * dt * 200f32; // Falling speed
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, PURPLE);
    }
}

// collision with positional correction
fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // Early exit if no collision
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };

    let to = b.center() - a.center();
    let to_signum = to.signum();

    if intersection.w > intersection.h {
        // Bounce on y-axis
        a.y -= to_signum.y * intersection.h;

        if to_signum.y > 0f32 {
            vel.y = -vel.y.abs();
        } else {
            vel.y = vel.y.abs();
        }

        // Adjust trajectory if colliding with the paddle
        if b.h == PLAYER_SIZE.y {
            let paddle_center = b.x + b.w * 0.5;
            let ball_center = a.x + a.w * 0.5;
            let relative_hit_pos = (ball_center - paddle_center) / (b.w * 0.5);

            // Adjust the x velocity based on the relative hit position
            vel.x += relative_hit_pos * 1.5; // Increase multiplier for sharper angles

            // Normalize to maintain consistent speed
            *vel = vel.normalize();

            // Clamp the angle to prevent excessive sharpness
            let min_y_velocity = 0.5; // Minimum y component to avoid shallow angles
            if vel.y.abs() < min_y_velocity {
                vel.y = vel.y.signum() * min_y_velocity;
                *vel = vel.normalize(); // Re-normalize after clamping
            }
        }
    } else {
        // Bounce on x-axis
        a.x -= to_signum.x * intersection.w;

        if to_signum.x < 0f32 {
            vel.x = vel.x.abs();
        } else {
            vel.x = -vel.x.abs();
        }
    }

    true
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
    current_level: usize,
    level_completed: bool,
) {
    *player = Player::new();
    balls.clear();
    init_blocks(blocks, current_level);

    if !level_completed {
        *score = 0;
        *player_lives = 3;
    } else {
        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
    }

}

fn init_blocks(blocks: &mut Vec<Block>, level: usize) {
    blocks.clear();
    let (width, height, block_size, block_lives) = match level {
        1 => (3, 3, BLOCK_SIZE, 2), // Level 1: Default size and lives
        2 => (6, 6, BLOCK_SIZE * 0.75, 3), // Level 2: Smaller blocks, more lives
        _ => (10, 10, BLOCK_SIZE * 0.5, 4), // Level 3+: Even smaller blocks, more lives
    };

    let padding = 5f32;
    let total_block_size = block_size + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;

        let block_type = if rand::gen_range(0, 10) < 2 {
            BlockType::SpawnPowerup // 20% chance to spawn a powerup block
        } else if rand::gen_range(0, 10) < 4 {
            BlockType::Strong // 20% chance to spawn a strong block
        } else {
            BlockType::Regular
        };

        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), block_type));
    }
}

fn handle_powerup_collision(player: &mut Player, powerups: &mut Vec<Powerup>) {
    powerups.retain(|powerup| {
        if powerup.rect.overlaps(&player.rect) {
            // Apply powerup effect: Increase paddle size
            player.rect.w += 50f32;
            false // Remove the powerup after collision
        } else {
            true
        }
    });
}

#[macroquad::main("rustanoid")]
async fn main() {
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf").await.unwrap();
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;
    let mut current_level = 1;
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    let mut powerups = Vec::new();
    let mut level_completed: bool = false;

    init_blocks(&mut blocks, current_level);

    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                player.update(get_frame_time());
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            if block.lives <= 0 {
                                score += 10;
                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    spawn_later.push(Ball::new(ball.rect.point()));
                                } else if block.block_type == BlockType::SpawnPowerup {
                                    powerups.push(Powerup::new(block.rect.point()));
                                }
                            }
                        }
                    }
                }
                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }

                for powerup in powerups.iter_mut() {
                    powerup.update(get_frame_time());
                }
                handle_powerup_collision(&mut player, &mut powerups);

                let balls_len = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player_lives -= 1;
                    balls.push(Ball::new(player.rect.point() + vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32, -50f32)));
                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                }

                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("Level {} Completed!", current_level), font);
                if is_key_pressed(KeyCode::Space) {
                    current_level += 1;
                    level_completed = true;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, current_level, level_completed);
                    game_state = GameState::Menu;
                }
            }
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    current_level = 1;
                    level_completed = false;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, current_level, level_completed);
                    game_state = GameState::Menu;
                }
            }
        }

        clear_background(WHITE);
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }
        for powerup in powerups.iter() {
            powerup.draw();
        }

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start", font);
            }
            GameState::Game => {
                let score_text = format!("score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() },
                );

                draw_text_ex(
                    &format!("lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() },
                );
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("Level {} Completed!", current_level), font);
            }
            GameState::Dead => {
                draw_title_text(&format!("You died. {} score", score), font);
            }
        }

        next_frame().await
    }
}
