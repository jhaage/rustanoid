use macroquad::prelude::*;
use crate::game_objects::texture_manager::TextureManager;

pub struct Powerup {
    pub rect: Rect,
    pub vel: Vec2,
}

impl Powerup {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, 30f32, 30f32),
            vel: vec2(0f32, 1f32),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.y += self.vel.y * dt * 200f32;
    }

    pub fn draw(&self, texture_manager: &TextureManager) {
        if let Some(texture) = texture_manager.power_up_texture {
            draw_texture_ex(
                texture,
                self.rect.x,
                self.rect.y,
                PURPLE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.rect.w, self.rect.h)),
                    ..Default::default()
                },
            );
        }
    }
}