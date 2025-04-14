use macroquad::prelude::*;
use crate::game_objects::texture_manager::TextureManager;

pub const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
pub const PLAYER_SPEED: f32 = 700f32;

pub struct Player {
    pub rect: Rect,
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

    pub fn draw(&self, texture_manager: &TextureManager) {
        if let Some(texture) = texture_manager.paddle_texture {
            draw_texture_ex(
                texture,
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