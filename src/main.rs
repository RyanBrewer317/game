use header::Updateable;
use macroquad::prelude::*;

mod cypress_forest;
mod header;
mod weapons;
mod camera;
mod player;
mod tree;

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
    let cypress_forest_obj = cypress_forest::CypressForest::new().await;
    let mut cypress_axe_obj = weapons::CypressAxe::new().await;
    let mut camera = camera::Camera { x: 0.0, y: 0.0 };
    let mut player = player::Player::new().await;

    loop {
        player.update();

        camera.x = player.pos.x - screen_width()/2.0;
        cypress_axe_obj.pos.x = player.pos.x;
        cypress_axe_obj.facing_right = player.facing_right;

        camera.draw(&cypress_forest_obj);
        camera.draw(&player);
        camera.draw(&cypress_axe_obj);

        next_frame().await
    }
}
