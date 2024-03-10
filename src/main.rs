use macroquad::prelude::*;

mod cypress_forest;
mod header;
mod weapons;

use crate::header::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        window_width: 1480,
        window_height: 1220,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let player_img: Texture2D = load_texture("assets/player.png").await.unwrap();
    player_img.set_filter(FilterMode::Nearest);
    let mut player_draw_options: DrawTextureParams = DrawTextureParams {
        dest_size: Some(vec2(PLAYER_WIDTH, PLAYER_HEIGHT)),
        source: None,
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };
    let cypress_forest_obj = cypress_forest::CypressForest::init().await;
    let cypress_axe_obj = weapons::CypressAxe::init().await;

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

        cypress_forest_obj.draw(x);
        draw_texture_ex(
            &player_img,
            out_x(x, x, 1),
            screen_height() - PLAYER_HEIGHT - y - 20.0,
            WHITE,
            player_draw_options.clone(),
        );
        cypress_axe_obj.draw(player_draw_options.flip_x, x, y);

        next_frame().await
    }
}
