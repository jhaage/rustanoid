use macroquad::audio::{load_sound, Sound, play_sound, PlaySoundParams};

pub struct AudioManager {
    pub paddle_hit: Option<Sound>,
    pub brick_hit: Option<Sound>,
    pub background_music: Option<Sound>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            paddle_hit: None,
            brick_hit: None,
            background_music: None,
        }
    }

    pub async fn load_sounds(&mut self, base_path: &str) {
        self.paddle_hit = Some(load_sound(&format!("{}sounds/paddle_hit.wav", base_path)).await.unwrap());
        self.brick_hit = Some(load_sound(&format!("{}sounds/brick_hit.wav", base_path)).await.unwrap());
        self.background_music = Some(load_sound(&format!("{}sounds/background_music.ogg", base_path)).await.unwrap());
    }

    pub fn play_sound_effect(&self, effect_type: &str) {
        match effect_type {
            "bounce" | "paddle_hit" => self.play_paddle_hit(),
            "block_hit" | "brick_hit" => self.play_brick_hit(),
            "block_destroyed" => self.play_brick_hit(), // Same sound for now
            "life_lost" => self.play_brick_hit(),       // Reusing brick hit sound for life lost
            _ => {}
        }
    }

    pub fn play_paddle_hit(&self) {
        if let Some(sound) = self.paddle_hit {
            play_sound(sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_brick_hit(&self) {
        if let Some(sound) = self.brick_hit {
            play_sound(sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_background_music(&self) {
        if let Some(music) = self.background_music {
            play_sound(music, PlaySoundParams {
                looped: true,
                volume: 0.5,
            });
        }
    }
}