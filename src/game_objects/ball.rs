use macroquad::prelude::*;
use crate::game_objects::texture_manager::TextureManager;

pub const BALL_SIZE: f32 = 50f32;
pub const BALL_SPEED: f32 = 400f32;

pub struct Ball {
    pub rect: Rect,
    pub vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        let random_angle = rand::gen_range(-45f32, 45f32).to_radians();
        let direction = vec2(random_angle.sin(), -random_angle.cos());
        
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: direction.normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Cap the maximum delta time to prevent large jumps
        let capped_dt = dt.min(1.0 / 60.0);
        
        // Update position
        self.rect.x += self.vel.x * capped_dt * BALL_SPEED;
        self.rect.y += self.vel.y * capped_dt * BALL_SPEED;

        // Handle screen bounds with proper reflection
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
            self.vel.x = -self.vel.x;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
            self.vel.x = -self.vel.x;
        }
        if self.rect.y < 0f32 {
            self.rect.y = 0f32;
            self.vel.y = -self.vel.y;
        }
        
        // Ensure velocity stays normalized
        self.vel = self.vel.normalize();
    }

    pub fn draw(&self, texture_manager: &TextureManager) {
        if let Some(texture) = &texture_manager.ball_texture {
            draw_texture_ex(
                &texture,
                self.rect.x,
                self.rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.rect.w, self.rect.h)),
                    ..Default::default()
                },
            );
        }
    }
}