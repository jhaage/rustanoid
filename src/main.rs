use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);

#[macroquad::main("rustanoid")]
async fn main() {
    let player_rect = Rect::new(
        screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
        screen_height() - 100f32,
        PLAYER_SIZE.x,
        PLAYER_SIZE.y
    );
    
    loop {
        clear_background(WHITE);
        draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, DARKGRAY);
        next_frame().await
    }
}
