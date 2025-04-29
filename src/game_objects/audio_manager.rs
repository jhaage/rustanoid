use macroquad::audio::{load_sound, Sound, play_sound, PlaySoundParams};

pub struct AudioManager {
    pub paddle_hit: Option<Sound>,
    pub brick_hit: Option<Sound>,
    pub brick_destroyed: Option<Sound>,
    pub life_lost: Option<Sound>,
    pub level_completed: Option<Sound>,
    pub powerup_collected: Option<Sound>,
    pub wall_hit: Option<Sound>,
    pub background_music: Option<Sound>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            paddle_hit: None,
            brick_hit: None,
            brick_destroyed: None,
            life_lost: None,
            level_completed: None,
            powerup_collected: None,
            wall_hit: None,
            background_music: None,
        }
    }

    pub async fn load_sounds(&mut self, base_path: &str) {
        self.paddle_hit = Some(load_sound(&format!("{}sounds/paddle_hit.wav", base_path)).await.unwrap());
        self.brick_hit = Some(load_sound(&format!("{}sounds/brick_hit.wav", base_path)).await.unwrap());
        self.brick_destroyed = Some(load_sound(&format!("{}sounds/brick_destroyed.wav", base_path)).await.unwrap());
        self.life_lost = Some(load_sound(&format!("{}sounds/life_lost.wav", base_path)).await.unwrap());
        self.level_completed = Some(load_sound(&format!("{}sounds/level_complete.wav", base_path)).await.unwrap());
        self.powerup_collected = Some(load_sound(&format!("{}sounds/paddle_grow.wav", base_path)).await.unwrap());
        self.wall_hit = Some(load_sound(&format!("{}sounds/wall_hit.wav", base_path)).await.unwrap());
        self.background_music = Some(load_sound(&format!("{}sounds/background_music.ogg", base_path)).await.unwrap());
    }

    pub fn play_sound_effect(&self, effect_type: &str) {
        match effect_type {
            "bounce" | "paddle_hit" => self.play_paddle_hit(),
            "block_hit" | "brick_hit" => self.play_brick_hit(),
            "block_destroyed" | "brick_destroyed" => self.play_brick_destroyed(),
            "life_lost" => self.play_life_lost(),
            "level_completed" => self.play_level_completed(),
            "powerup_collected" => self.play_powerup_collected(),
            "wall_hit" => self.play_wall_hit(),
            _ => {}
        }
    }

    pub fn play_paddle_hit(&self) {
        if let Some(sound) = &self.paddle_hit {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_brick_hit(&self) {
        if let Some(sound) = &self.brick_hit {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_brick_destroyed(&self) {
        if let Some(sound) = &self.brick_destroyed {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_life_lost(&self) {
        if let Some(sound) = &self.life_lost {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_level_completed(&self) {
        if let Some(sound) = &self.level_completed {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_powerup_collected(&self) {
        if let Some(sound) = &self.powerup_collected {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_wall_hit(&self) {
        if let Some(sound) = &self.wall_hit {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
    }

    pub fn play_background_music(&self) {
        if let Some(music) = &self.background_music {
            play_sound(&music, PlaySoundParams {
                looped: true,
                volume: 0.5,
            });
        }
    }
}