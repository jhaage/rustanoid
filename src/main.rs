use macroquad::prelude::*;

mod game_objects;
use game_objects::{
    ball::{Ball, BALL_SIZE},
    block::{Block, BlockType, BLOCK_SIZE},
    player::{Player, PLAYER_SIZE},
    powerup::Powerup,
    texture_manager::TextureManager,
    audio_manager::AudioManager,
};

#[derive(PartialEq)]
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
    
    // Base configuration for board dimensions
    let padding = 2.0;
    let available_width = screen_width() * 0.9;
    
    // Get level layout based on current level
    let layout = get_level_layout(level);
    let (width, height) = (layout.width, layout.height);
    
    // Calculate block size based on width
    let block_width = (available_width - (padding * (width as f32 - 1.0))) / width as f32;
    let block_height = block_width * (BLOCK_SIZE.y / BLOCK_SIZE.x);
    let actual_block_size = vec2(block_width, block_height);
    
    // Calculate board dimensions
    let board_width = (width as f32 * block_width) + ((width - 1) as f32 * padding);
    let board_start_x = (screen_width() - board_width) * 0.5;
    let board_start_y = 50f32;
    
    // Create block layout based on the pattern
    let mut temp_blocks = Vec::new();
    
    // Generate blocks based on the layout pattern
    for y in 0..height {
        for x in 0..width {
            let _index = y * width + x;
            
            // Check if we should create a block at this position
            if let Some(block_type) = layout.get_block_at(x, y) {
                let block_x = board_start_x + x as f32 * (block_width + padding);
                let block_y = board_start_y + y as f32 * (block_height + padding);
                
                // Add the block with the specified type
                temp_blocks.push(Block::new(
                    vec2(block_x, block_y),
                    block_type,
                    actual_block_size,
                ));
            }
        }
    }
    
    // Ensure at least one powerup block per level if we don't already have one
    if !temp_blocks.iter().any(|b| b.block_type == BlockType::SpawnPowerup) && !temp_blocks.is_empty() {
        let random_index = rand::gen_range(0, temp_blocks.len());
        if let Some(block) = temp_blocks.get_mut(random_index) {
            block.block_type = BlockType::SpawnPowerup;
        }
    }
    
    *blocks = temp_blocks;
}

// Structure to define a level layout
struct LevelLayout {
    width: usize,
    height: usize,
    pattern: Vec<Option<BlockType>>,
}

impl LevelLayout {
    fn get_block_at(&self, x: usize, y: usize) -> Option<BlockType> {
        if x < self.width && y < self.height {
            self.pattern[y * self.width + x]
        } else {
            None
        }
    }
}

fn get_level_layout(level: usize) -> LevelLayout {
    // Handle regular levels (1-10)
    let adjusted_level = if level <= 10 { level } else { ((level - 1) % 10) + 1 };
    
    match adjusted_level {
        1 => {
            // Level 1: Simple pattern with clear path in the middle (beginner friendly)
            let width = 8;
            let height = 4;
            let mut pattern = vec![None; width * height];
            
            for y in 0..height {
                for x in 0..width {
                    // Leave the middle column empty for an easy path
                    if x != width / 2 {
                        pattern[y * width + x] = Some(BlockType::Regular);
                    }
                }
            }
            
            // Add a power-up block
            pattern[1 * width + 1] = Some(BlockType::SpawnPowerup);
            
            LevelLayout { width, height, pattern }
        },
        
        2 => {
            // Level 2: Zigzag pattern
            let width = 9;
            let height = 5;
            let mut pattern = vec![None; width * height];
            
            for y in 0..height {
                for x in 0..width {
                    // Create a zigzag pattern by clearing alternating columns
                    if (y % 2 == 0 && x % 2 == 0) || (y % 2 == 1 && x % 2 == 1) {
                        pattern[y * width + x] = Some(BlockType::Regular);
                    }
                }
            }
            
            // Add some medium blocks
            pattern[2 * width + 2] = Some(BlockType::Medium);
            pattern[0 * width + 4] = Some(BlockType::Medium);
            
            // Add a power-up block
            pattern[3 * width + 5] = Some(BlockType::SpawnPowerup);
            
            LevelLayout { width, height, pattern }
        },
        
        3 => {
            // Level 3: Castle-like pattern
            let width = 9;
            let height = 6;
            let mut pattern = vec![None; width * height];
            
            // Build outer walls
            for y in 0..height {
                for x in 0..width {
                    if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                        pattern[y * width + x] = Some(BlockType::Medium);
                    }
                    // Add interior structures
                    else if (x == 2 || x == width - 3) && y > 1 {
                        pattern[y * width + x] = Some(BlockType::Regular);
                    }
                }
            }
            
            // Add gates (openings)
            pattern[height - 1] = None; // Bottom left gate
            pattern[(height - 1) * width + (width - 1)] = None; // Bottom right gate
            pattern[0 * width + width / 2] = None; // Top middle gate
            
            // Add power-up and strong blocks
            pattern[2 * width + 4] = Some(BlockType::SpawnPowerup);
            pattern[3 * width + 4] = Some(BlockType::Strong);
            
            LevelLayout { width, height, pattern }
        },
        
        4 => {
            // Level 4: Snake pattern
            let width = 10;
            let height = 6;
            let mut pattern = vec![None; width * height];
            
            for y in 0..height {
                for x in 0..width {
                    // Create a snake-like pattern
                    if y % 2 == 0 {
                        // Even rows: fill all except rightmost cell
                        if x < width - 1 {
                            pattern[y * width + x] = Some(BlockType::Regular);
                        }
                    } else {
                        // Odd rows: fill all except leftmost cell
                        if x > 0 {
                            pattern[y * width + x] = Some(BlockType::Regular);
                        }
                    }
                }
            }
            
            // Add some strong blocks at the turns
            for y in 1..height-1 {
                if y % 2 == 0 {
                    pattern[y * width + (width - 2)] = Some(BlockType::Strong);
                } else {
                    pattern[y * width + 1] = Some(BlockType::Strong);
                }
            }
            
            // Add power-ups
            pattern[1 * width + 5] = Some(BlockType::SpawnPowerup);
            pattern[3 * width + 5] = Some(BlockType::SpawnBallOnDeath);
            
            LevelLayout { width, height, pattern }
        },
        
        5 => {
            // Level 5: Concentric squares
            let width = 11;
            let height = 7;
            let mut pattern = vec![None; width * height];
            
            for y in 0..height {
                for x in 0..width {
                    // Create concentric squares
                    let distance = y.min(height - 1 - y).min(x).min(width - 1 - x);
                    
                    match distance {
                        0 => pattern[y * width + x] = Some(BlockType::Strong),
                        1 => pattern[y * width + x] = Some(BlockType::Medium),
                        2 => pattern[y * width + x] = Some(BlockType::Regular),
                        _ => {} // Leave empty
                    }
                }
            }
            
            // Create entry points into the squares
            pattern[0 * width + width / 2] = None; // Top middle
            pattern[(height - 1) * width + width / 2] = None; // Bottom middle
            pattern[height / 2 * width + 0] = None; // Middle left
            pattern[height / 2 * width + (width - 1)] = None; // Middle right
            
            // Add power-ups
            pattern[2 * width + 2] = Some(BlockType::SpawnPowerup);
            pattern[4 * width + 8] = Some(BlockType::SpawnPowerup);
            
            LevelLayout { width, height, pattern }
        },
        
        6 => {
            // Level 6: Branching paths
            let width = 12;
            let height = 7;
            let mut pattern = vec![Some(BlockType::Regular); width * height];
            
            // Create main path
            for x in 0..width {
                pattern[3 * width + x] = None;
            }
            
            // Create vertical paths
            for y in 0..height {
                pattern[y * width + 2] = None;
                pattern[y * width + 5] = None;
                pattern[y * width + 9] = None;
            }
            
            // Add stronger blocks in key positions
            for y in 0..height {
                for x in 0..width {
                    if (y == 1 || y == 5) && (x == 1 || x == 6 || x == 10) {
                        pattern[y * width + x] = Some(BlockType::Medium);
                    }
                    if (y == 0 || y == 6) && (x % 4 == 0) {
                        pattern[y * width + x] = Some(BlockType::Strong);
                    }
                }
            }
            
            // Add power-ups
            pattern[1 * width + 3] = Some(BlockType::SpawnPowerup);
            pattern[5 * width + 8] = Some(BlockType::SpawnBallOnDeath);
            
            LevelLayout { width, height, pattern }
        },
        
        7 => {
            // Level 7: Diamond pattern
            let width = 13;
            let height = 8;
            let mut pattern = vec![None; width * height];
            
            let center_x = width / 2;
            let center_y = height / 2;
            
            for y in 0..height {
                for x in 0..width {
                    // Calculate Manhattan distance from center
                    let dist = (x as isize - center_x as isize).abs() + 
                               (y as isize - center_y as isize).abs();
                    
                    if dist <= 4 && dist % 2 == 0 {
                        if dist == 0 {
                            pattern[y * width + x] = Some(BlockType::Strong);
                        } else {
                            pattern[y * width + x] = Some(BlockType::Regular);
                        }
                    } else if dist <= 6 && x % 2 == y % 2 {
                        pattern[y * width + x] = Some(BlockType::Medium);
                    }
                }
            }
            
            // Add diagonal paths
            for i in 0..width.min(height) {
                if i < width && i < height {
                    pattern[i * width + i] = None;
                }
                if i < height && (width - 1 - i) < width {
                    pattern[i * width + (width - 1 - i)] = None;
                }
            }
            
            // Add power-ups
            pattern[1 * width + center_x] = Some(BlockType::SpawnPowerup);
            pattern[(height - 2) * width + center_x] = Some(BlockType::SpawnBallOnDeath);
            
            LevelLayout { width, height, pattern }
        },
        
        8 => {
            // Level 8: Tetris pieces layout
            let width = 10;
            let height = 8;
            let mut pattern = vec![None; width * height];
            
            // Create background with some gaps
            for y in 0..height {
                for x in 0..width {
                    if (x + y) % 4 != 0 {
                        pattern[y * width + x] = Some(BlockType::Regular);
                    }
                }
            }
            
            // Create Tetris I-piece (vertical)
            for y in 1..5 {
                pattern[y * width + 2] = Some(BlockType::Medium);
            }
            
            // Create Tetris T-piece
            pattern[2 * width + 6] = Some(BlockType::Medium);
            pattern[3 * width + 5] = Some(BlockType::Medium);
            pattern[3 * width + 6] = Some(BlockType::Medium);
            pattern[3 * width + 7] = Some(BlockType::Medium);
            
            // Create Tetris L-piece
            pattern[5 * width + 3] = Some(BlockType::Medium);
            pattern[6 * width + 3] = Some(BlockType::Medium);
            pattern[7 * width + 3] = Some(BlockType::Medium);
            pattern[7 * width + 4] = Some(BlockType::Medium);
            
            // Create Tetris square piece
            pattern[5 * width + 7] = Some(BlockType::Strong);
            pattern[5 * width + 8] = Some(BlockType::Strong);
            pattern[6 * width + 7] = Some(BlockType::Strong);
            pattern[6 * width + 8] = Some(BlockType::Strong);
            
            // Add power-ups
            pattern[1 * width + 7] = Some(BlockType::SpawnPowerup);
            pattern[6 * width + 1] = Some(BlockType::SpawnBallOnDeath);
            
            LevelLayout { width, height, pattern }
        },
        
        9 => {
            // Level 9: Grid with strong center
            let width = 11;
            let height = 9;
            let mut pattern = vec![None; width * height];
            
            // Create grid pattern
            for y in 0..height {
                for x in 0..width {
                    if x % 3 == 0 || y % 3 == 0 {
                        pattern[y * width + x] = Some(BlockType::Regular);
                        
                        // Make the central crossing stronger
                        if (x == 3 || x == 6) && (y == 3 || y == 6) {
                            pattern[y * width + x] = Some(BlockType::Medium);
                        }
                    }
                }
            }
            
            // Create a strong center
            for y in 4..=5 {
                for x in 4..=6 {
                    pattern[y * width + x] = Some(BlockType::Strong);
                }
            }
            
            // Create tunnels by removing some blocks
            for i in 0..width {
                // Horizontal tunnels
                pattern[1 * width + i] = None;
                pattern[7 * width + i] = None;
            }
            
            for i in 0..height {
                // Vertical tunnels
                pattern[i * width + 1] = None;
                pattern[i * width + 9] = None;
            }
            
            // Add power-ups
            pattern[2 * width + 5] = Some(BlockType::SpawnPowerup);
            pattern[6 * width + 5] = Some(BlockType::SpawnBallOnDeath);
            
            LevelLayout { width, height, pattern }
        },
        
        10 => {
            // Level 10: Maze-like pattern
            let width = 14;
            let height = 10;
            let mut pattern = vec![None; width * height];
            
            // First, fill all with regular blocks
            for y in 0..height {
                for x in 0..width {
                    pattern[y * width + x] = Some(BlockType::Regular);
                }
            }
            
            // Create maze paths (using horizontal and vertical lines)
            
            // Horizontal paths
            for x in 1..width-1 {
                pattern[1 * width + x] = None;
                pattern[3 * width + x] = None;
                pattern[5 * width + x] = None;
                pattern[7 * width + x] = None;
                pattern[9 * width + x] = None;
            }
            
            // Vertical paths - connect the horizontal paths
            for y in 0..height {
                pattern[y * width + 2] = None;
                pattern[y * width + 5] = None;
                pattern[y * width + 8] = None;
                pattern[y * width + 11] = None;
            }
            
            // Block some of the connections to create a more maze-like structure
            pattern[1 * width + 5] = Some(BlockType::Medium);
            pattern[3 * width + 2] = Some(BlockType::Medium);
            pattern[5 * width + 11] = Some(BlockType::Medium);
            pattern[7 * width + 8] = Some(BlockType::Medium);
            pattern[9 * width + 5] = Some(BlockType::Medium);
            
            // Add some strong blocks at strategic locations
            pattern[2 * width + 3] = Some(BlockType::Strong);
            pattern[4 * width + 6] = Some(BlockType::Strong);
            pattern[6 * width + 9] = Some(BlockType::Strong);
            pattern[8 * width + 12] = Some(BlockType::Strong);
            
            // Add a few more medium blocks to increase difficulty
            pattern[0 * width + 0] = Some(BlockType::Medium);
            pattern[0 * width + width-1] = Some(BlockType::Medium);
            pattern[(height-1) * width + 0] = Some(BlockType::Medium);
            pattern[(height-1) * width + width-1] = Some(BlockType::Medium);
            
            // Add power-ups
            pattern[2 * width + 7] = Some(BlockType::SpawnPowerup);
            pattern[6 * width + 3] = Some(BlockType::SpawnPowerup);
            pattern[8 * width + 10] = Some(BlockType::SpawnBallOnDeath);
            
            LevelLayout { width, height, pattern }
        },
        
        // Default case for levels beyond 10 (loops back to level 1-10 patterns but with increased difficulty)
        _ => {
            // This should never happen due to the adjusted_level calculation above
            // But returning a simple layout just in case
            let layout = get_level_layout(1);
            
            // Increase the difficulty by converting regular blocks to medium/strong ones
            let mut pattern = layout.pattern;
            let difficulty_factor = (level / 10) + 1;
            
            for i in 0..pattern.len() {
                if let Some(_block_type) = pattern[i] {
                    // As levels progress, increase the chance of stronger blocks
                    let random_val = rand::gen_range(0, 10);
                    if random_val < difficulty_factor {
                        pattern[i] = Some(BlockType::Strong);
                    } else if random_val < difficulty_factor * 2 {
                        pattern[i] = Some(BlockType::Medium);
                    }
                }
            }
            
            LevelLayout {
                width: layout.width,
                height: layout.height,
                pattern,
            }
        }
    }
}

fn handle_powerup_collision(player: &mut Player, powerups: &mut Vec<Powerup>, audio_manager: &AudioManager) {
    let max_paddle_width = screen_width() / 3.0;
    
    powerups.retain(|powerup| {
        if powerup.rect.overlaps(&player.rect) {
            // Apply powerup effect: Increase paddle size, but limit to max width
            player.rect.w = (player.rect.w + 50f32).min(max_paddle_width);
            audio_manager.play_sound_effect("powerup_collected");
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
    
    // Initialize and load audio
    let mut audio_manager = AudioManager::new();
    audio_manager.load_sounds(base_path).await;
    audio_manager.play_background_music();

    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;
    let mut current_level = 1;
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    let mut powerups = Vec::new();
    let mut level_completed: bool = false;
    
    // For development/testing - enables level jumping with keyboard shortcuts
    let mut dev_mode = true;
    let mut show_dev_message = false;
    let mut dev_message_timer = 0.0;

    init_blocks(&mut blocks, current_level);
    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));

    loop {
        // Level jumping shortcuts for development/testing
        if dev_mode {
            // Number keys 1-9 to jump to levels 1-9
            for i in 1..=9 {
                if is_key_pressed(match i {
                    1 => KeyCode::Key1,
                    2 => KeyCode::Key2,
                    3 => KeyCode::Key3,
                    4 => KeyCode::Key4,
                    5 => KeyCode::Key5,
                    6 => KeyCode::Key6,
                    7 => KeyCode::Key7,
                    8 => KeyCode::Key8,
                    9 => KeyCode::Key9,
                    _ => KeyCode::Key0,
                }) {
                    current_level = i;
                    level_completed = true;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, current_level, level_completed);
                    game_state = GameState::Game;
                    show_dev_message = true;
                    dev_message_timer = 2.0; // Show message for 2 seconds
                }
            }
            
            // 0 key for level 10
            if is_key_pressed(KeyCode::Key0) {
                current_level = 10;
                level_completed = true;
                reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, current_level, level_completed);
                game_state = GameState::Game;
                show_dev_message = true;
                dev_message_timer = 2.0;
            }
            
            // Page Up/Down to cycle through levels
            if is_key_pressed(KeyCode::PageUp) && game_state == GameState::Game {
                current_level = (current_level + 1).min(20); // Limit to 20 levels for safety
                level_completed = true;
                reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, current_level, level_completed);
                show_dev_message = true;
                dev_message_timer = 2.0;
            }
            
            if is_key_pressed(KeyCode::PageDown) && game_state == GameState::Game {
                current_level = (current_level - 1).max(1); // Don't go below level 1
                level_completed = true;
                reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, current_level, level_completed);
                show_dev_message = true;
                dev_message_timer = 2.0;
            }
            
            // Toggle dev mode with F12
            if is_key_pressed(KeyCode::F12) {
                dev_mode = !dev_mode;
                show_dev_message = true;
                dev_message_timer = 2.0;
            }
            
            // Update dev message timer
            if show_dev_message {
                dev_message_timer -= get_frame_time();
                if dev_message_timer <= 0.0 {
                    show_dev_message = false;
                }
            }
        }

        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                player.update(get_frame_time());
                
                for ball in balls.iter_mut() {
                    // Store position before update to detect wall collisions
                    let prev_x = ball.rect.x;
                    let prev_y = ball.rect.y;
                    let prev_right = ball.rect.x + ball.rect.w;
                    
                    // Update ball position
                    ball.update(get_frame_time());
                    
                    // Detect wall collisions more reliably
                    let hit_left_wall = prev_x > 0.0 && ball.rect.x <= 0.01;
                    let hit_right_wall = prev_right < screen_width() && ball.rect.x + ball.rect.w >= screen_width() - 0.01;
                    let hit_ceiling = prev_y > 0.0 && ball.rect.y <= 0.01;
                    
                    if hit_left_wall || hit_right_wall || hit_ceiling {
                        audio_manager.play_sound_effect("wall_hit");
                    }
                }

                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    if resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect) {
                        audio_manager.play_sound_effect("bounce");
                    }
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            audio_manager.play_sound_effect("block_hit");
                            if block.lives <= 0 {
                                score += 10;
                                audio_manager.play_sound_effect("block_destroyed");
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
                handle_powerup_collision(&mut player, &mut powerups, &audio_manager);

                let balls_len = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player_lives -= 1;
                    audio_manager.play_sound_effect("life_lost");
                    balls.push(Ball::new(player.rect.point() + vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32, -50f32)));
                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                }

                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                    audio_manager.play_sound_effect("level_completed");
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

                // Show dev mode message if enabled
                if dev_mode && show_dev_message {
                    let dev_message = format!("Dev Mode: Level {}", current_level);
                    let dev_message_dim = measure_text(&dev_message, Some(font), 20u16, 1.0);
                    draw_text_ex(
                        &dev_message,
                        screen_width() * 0.5f32 - dev_message_dim.width * 0.5f32,
                        screen_height() - 40.0,
                        TextParams { font, font_size: 20u16, color: RED, ..Default::default() },
                    );
                }
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
