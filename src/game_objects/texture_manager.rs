use macroquad::prelude::*;

pub struct TextureManager {
    pub block_texture: Option<Texture2D>,
    pub ball_texture: Option<Texture2D>,
    pub paddle_texture: Option<Texture2D>,
    pub power_up_texture: Option<Texture2D>,
    pub background_texture: Option<Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            block_texture: None,
            ball_texture: None,
            paddle_texture: None,
            power_up_texture: None,
            background_texture: None,
        }
    }

    pub async fn load_textures(&mut self, base_path: &str) {
        self.block_texture = Some(load_texture(&format!("{}{}", base_path, "block.png")).await.unwrap());
        self.ball_texture = Some(load_texture(&format!("{}{}", base_path, "ball.png")).await.unwrap());
        self.paddle_texture = Some(load_texture(&format!("{}{}", base_path, "paddle.png")).await.unwrap());
        self.power_up_texture = Some(load_texture(&format!("{}{}", base_path, "powerup.png")).await.unwrap());
        self.background_texture = Some(load_texture(&format!("{}{}", base_path, "background.png")).await.unwrap());
    }
}