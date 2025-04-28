use macroquad::prelude::*;
use crate::game_objects::texture_manager::TextureManager;

pub const BLOCK_SIZE: Vec2 = vec2(100f32, 40f32);

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath,
    Medium,  // 2 lives
    Strong,  // 3 lives
    SpawnPowerup,
}

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType, size: Vec2) -> Self {
        let lives = match block_type {
            BlockType::Strong => 3,
            BlockType::Medium => 2,
            _ => 1,
        };
        Self {
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            lives,
            block_type,
        }
    }

    pub fn draw(&self, texture_manager: &TextureManager) {
        let color = match self.block_type {
            BlockType::Regular => WHITE,
            BlockType::Medium => match self.lives {
                2 => ORANGE,
                1 => YELLOW,
                _ => unreachable!(),
            },
            BlockType::Strong => match self.lives {
                3 => RED,
                2 => ORANGE,
                1 => YELLOW,
                _ => unreachable!(),
            },
            BlockType::SpawnBallOnDeath => GREEN,
            BlockType::SpawnPowerup => BLUE,
        };
        
        if let Some(texture) = texture_manager.block_texture {
            draw_texture_ex(
                texture,
                self.rect.x,
                self.rect.y,
                color,
                DrawTextureParams {
                    dest_size: Some(vec2(self.rect.w, self.rect.h)),
                    ..Default::default()
                },
            );
        }
    }
}