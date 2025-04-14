use macroquad::prelude::*;

mod game_objects;
use game_objects::{
    ball::{Ball, BALL_SIZE},
    block::{Block, BlockType, BLOCK_SIZE},
    player::{Player, PLAYER_SIZE},
    powerup::Powerup,
    texture_manager::TextureManager,
};

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams{font, font_size: 50u16, color: BLACK, ..Default::default()}
    );
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
    init_blocks(blocks, current_level);

    if !level_completed {
        // Reset everything for game over
        balls.clear();
        *score = 0;
        *player_lives = 3;
        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
    } else {
        // Just reset ball position for next level
        balls.clear();
        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
    }
}

fn init_blocks(blocks: &mut Vec<Block>, level: usize) {
    blocks.clear();
    let (width, height) = match level {
        1 => (4, 3),      // Level 1: Wider but fewer rows
        2 => (6, 4),      // Level 2: More blocks
        3 => (8, 5),      // Level 3: Even more blocks
        _ => (10, 6),     // Level 4+: Maximum difficulty
    };

    // Calculate the size of blocks to fit the screen width with padding
    let padding = 2.0; // Reduced padding
    let available_width = screen_width() * 0.9;
    let block_width = (available_width - (padding * (width as f32 - 1.0))) / width as f32;
    let block_height = block_width * (BLOCK_SIZE.y / BLOCK_SIZE.x);
    let actual_block_size = vec2(block_width, block_height);

    let board_width = (width as f32 * block_width) + ((width - 1) as f32 * padding);
    let board_start_x = (screen_width() - board_width) * 0.5;
    let board_start_y = 50f32;

    // Create initial block layout
    let mut temp_blocks = Vec::new();
    for i in 0..width * height {
        let block_x = board_start_x + (i % width) as f32 * (block_width + padding);
        let block_y = board_start_y + (i / width) as f32 * (block_height + padding);
        
        // Create varied patterns based on level
        let should_create_block = match level {
            1 => true,  // Level 1: Simple full pattern
            2 => {
                // Level 2: Checkerboard pattern in top row
                let row = i / width;
                if row == 0 {
                    (i % width) % 2 == 0
                } else {
                    true
                }
            },
            3 => {
                // Level 3: Alternating gaps in top two rows
                let row = i / width;
                if row < 2 {
                    (i % width + row) % 2 == 0
                } else {
                    true
                }
            },
            _ => {
                // Level 4+: Random gaps in top rows with increasing complexity
                let row = i / width;
                if row < 2 {
                    rand::gen_range(0, 100) < 70  // 70% chance of block in top rows
                } else {
                    rand::gen_range(0, 100) < 90  // 90% chance of block in other rows
                }
            }
        };

        if should_create_block {
            let block_type = if level >= 4 && rand::gen_range(0, 10) < 1 {
                BlockType::Strong
            } else if rand::gen_range(0, 10) < 2 {
                BlockType::Medium
            } else if rand::gen_range(0, 10) < 1 {
                BlockType::SpawnPowerup
            } else {
                BlockType::Regular
            };

            temp_blocks.push(Block::new(
                vec2(block_x, block_y),
                block_type,
                actual_block_size,
            ));
        }
    }

    // Ensure at least one powerup block per level
    if !temp_blocks.iter().any(|b| b.block_type == BlockType::SpawnPowerup) {
        let random_index = rand::gen_range(0, temp_blocks.len());
        if let Some(block) = temp_blocks.get_mut(random_index) {
            block.block_type = BlockType::SpawnPowerup;
        }
    }

    *blocks = temp_blocks;
}

fn handle_powerup_collision(player: &mut Player, powerups: &mut Vec<Powerup>) {
    let max_paddle_width = screen_width() / 3.0;
    
    powerups.retain(|powerup| {
        if powerup.rect.overlaps(&player.rect) {
            // Apply powerup effect: Increase paddle size, but limit to max width
            player.rect.w = (player.rect.w + 50f32).min(max_paddle_width);
            false // Remove the powerup after collision
        } else {
            true
        }
    });
}

#[macroquad::main("rustanoid")]
async fn main() {
    // Define base path for assets - will be different for web assembly (./serve.sh and next.js website) vs cargo run
    let base_path = if cfg!(target_arch = "wasm32") {
        "rustanoid/res/"
    } else {
        "res/"
    };

    let font = load_ttf_font(&format!("{}{}", base_path, "Heebo-VariableFont_wght.ttf")).await.unwrap();
    
    // Initialize managers
    let mut texture_manager = TextureManager::new();
    texture_manager.load_textures(base_path).await;

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
                    if resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect) {
                        // Audio removed
                    }
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            // Audio removed
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

        if let Some(bg_texture) = texture_manager.background_texture {
            draw_texture_ex(
                bg_texture,
                0.0,
                0.0,
                Color::new(0.7, 0.7, 0.7, 1.0),
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        } else {
            clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
        }

        player.draw(&texture_manager);
        for block in blocks.iter() {
            block.draw(&texture_manager);
        }
        for ball in balls.iter() {
            ball.draw(&texture_manager);
        }
        for powerup in powerups.iter() {
            powerup.draw(&texture_manager);
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

                let level_text = format!("Level: {}", current_level);
                let level_text_dim = measure_text(&level_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &level_text,
                    screen_width() - level_text_dim.width - 30.0,
                    40.0,
                    TextParams { font, font_size: 30u16, color: BLACK, ..Default::default()},
                );
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("Level {} Completed!", current_level), font);
            }
            GameState::Dead => {
                draw_title_text(&format!("Game over. Your score: {}", score), font);
            }
        }

        next_frame().await
    }
}
