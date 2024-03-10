
use macroquad::prelude::*;

const PLAYER_HEIGHT: f32 = 145.0;
const PLAYER_WIDTH: f32 = 75.0;
const CYPRESS_WIDTH: f32 = 1480.0;
const CYPRESS_HEIGHT: f32 = 1220.0;
const CYPRESS_DRAW_OPTIONS: DrawTextureParams = DrawTextureParams { dest_size: Some(vec2(CYPRESS_WIDTH, CYPRESS_HEIGHT)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None };
const BACKGROUND_WIDTH: f32 = 2500.0;
const BACKGROUND_HEIGHT: f32 = 1000.0;
const BACKGROUND_DRAW_OPTIONS: DrawTextureParams = DrawTextureParams { dest_size: Some(vec2(BACKGROUND_WIDTH, BACKGROUND_HEIGHT)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None };
const CYPRESS_FOREST_WIDTH: f32 = 6000.0;
const CYPRESS_FOREST_HEIGHT: f32 = 600.0;
const CYPRESS_FOREST_DRAW_OPTIONS: DrawTextureParams = DrawTextureParams { dest_size: Some(vec2(CYPRESS_FOREST_WIDTH, CYPRESS_FOREST_HEIGHT)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None };

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        window_width: 1480,
        window_height: 1220,
        ..Default::default()
    }
}

fn out_x(x: f32, player_x: f32, layer: i32) -> f32 {
    if screen_width()/2.0 - PLAYER_WIDTH/2.0 - player_x/4.0 < 0.0 {
        if screen_width()/2.0 - PLAYER_WIDTH/2.0 - (player_x - 10000.0)/4.0 > screen_width() {
            screen_width()/2.0 - PLAYER_WIDTH/2.0 - (player_x - x)/(layer as f32)
        } else {
            screen_width()/2.0 - PLAYER_WIDTH/2.0 - (-2.0*screen_width() - 2.0*PLAYER_WIDTH + 10000.0 - x)/(layer as f32)
        }
    } else {
        screen_width()/2.0 - PLAYER_WIDTH/2.0 - (2.0*screen_width() - 2.0*PLAYER_WIDTH - x)/(layer as f32)
    }
}

// SW/2 - PW/2 - (PX - 5000)/4 = SW
// -SW/2 - PW/2 - (PX - 5000)/4 = 0
// -SW/2 - PW/2 = (PX - 5000)/4
// -2SW - 2PW = PX - 5000
// -2SW - 2PW + 5000 = PX

#[macroquad::main(window_conf)]
async fn main() {
    let player_img: Texture2D = load_texture("assets/player.png").await.unwrap();
    player_img.set_filter(FilterMode::Nearest);
    let mut player_draw_options: DrawTextureParams = DrawTextureParams { dest_size: Some(vec2(PLAYER_WIDTH, PLAYER_HEIGHT)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None };
    let cypress_img: Texture2D = load_texture("assets/cypress.png").await.unwrap();
    cypress_img.set_filter(FilterMode::Nearest);
    let background_img: Texture2D = load_texture("assets/background.png").await.unwrap();
    background_img.set_filter(FilterMode::Nearest);
    let cypress_forest_img: Texture2D = load_texture("assets/cypress_forest.png").await.unwrap();
    cypress_forest_img.set_filter(FilterMode::Nearest);

    let trees = [(200, true), (1000, true), (2234, false), (3000, true), (4363, false), (5000, true), (6435, false), (7000, false), (8654, true)];

    let y = 0.0;
    let mut x = 8000.0;
    
    loop {
        if is_key_down(KeyCode::A) {
            x -= 15.0;
            player_draw_options.flip_x = false;
        }
        if is_key_down(KeyCode::D) {
            x += 15.0;
            player_draw_options.flip_x = true;
        }
        if x < 1000.0 {
            x = 1000.0;
        }
        if x > 10000.0 - PLAYER_WIDTH - 1225.0 {
            x = 10000.0 - PLAYER_WIDTH - 1225.0;
        }
        draw_texture_ex(&background_img, out_x(0.0, x, 4), screen_height() - BACKGROUND_HEIGHT, WHITE, BACKGROUND_DRAW_OPTIONS);
        draw_texture_ex(&cypress_forest_img, out_x(0.0, x, 2), screen_height() - CYPRESS_FOREST_HEIGHT - 40.0, WHITE, CYPRESS_FOREST_DRAW_OPTIONS);
        for tree in trees {
            let (tree_x, flip) = tree;
            let mut tree_options = CYPRESS_DRAW_OPTIONS.clone();
            tree_options.flip_x = flip;
            draw_texture_ex(&cypress_img, out_x(tree_x as f32, x, 1), screen_height() - CYPRESS_HEIGHT - 20.0, WHITE, tree_options);
        }
        draw_texture_ex(&player_img, out_x(x, x, 1), screen_height() - PLAYER_HEIGHT - y - 20.0, WHITE, player_draw_options.clone());

        next_frame().await
    }
}