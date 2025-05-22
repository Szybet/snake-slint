use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use consts::{
    BLOCK_BORDER_COLOR, BLOCK_BORDER_WIDTH, BLOCK_COLOR_UNUSED, BLOCK_COLOR_USED, GRID_SIZE,
    LOOP_DELAY_MS,
};
use game::{Direction, Snake, key_to_direction};
use log::info;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

pub mod consts;
pub mod game;
pub mod grid;

pub fn get_snake_app_window() -> AppWindow {
    let ui = AppWindow::new().unwrap();
    ui.global::<GameAdapter>().set_grid_size(GRID_SIZE);

    // Set grid structure
    ui.set_block_color_all(BLOCK_COLOR_UNUSED);

    ui.global::<GameAdapter>()
        .set_block_border_width(BLOCK_BORDER_WIDTH);
    ui.global::<GameAdapter>()
        .set_block_border_color(BLOCK_BORDER_COLOR);

    ui.global::<GameAdapter>().set_overlay_text("".into());
    ui.global::<GameAdapter>().set_overlay_visible(false);

    ui
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    info!("Starting...");

    let ui = get_snake_app_window();

    let ui_weak = ui.as_weak();

    std::thread::spawn(move || {
        let snake = Arc::new(Mutex::new(Snake::new()));
        let dir = Arc::new(Mutex::new(Direction::None));
        loop {
            // info!("Dir: {:?}", dir);
            std::thread::sleep(std::time::Duration::from_millis(LOOP_DELAY_MS));

            let ui_clone = ui_weak.clone();
            let snake_clone = Arc::clone(&snake);
            let dir_clone = Arc::clone(&dir);

            slint::invoke_from_event_loop(move || {
                let ui = ui_clone.upgrade().unwrap();
                ui.on_key_pressed({
                    let dir_clone = Arc::clone(&dir_clone);
                    move |key: SharedString| {
                        // info!("Received key: {:?}", key);
                        let mut dir = dir_clone.lock().unwrap();
                        if *dir == Direction::None {
                            *dir = key_to_direction(key);
                        }
                    }
                });
                let mut snake_lock = snake_clone.lock().unwrap();
                let mut dir = dir_clone.lock().unwrap();
                snake_lock.game_loop(&mut dir);
                *dir = Direction::None;
                snake_lock.draw(&ui);
            })
            .unwrap();
        }
    });

    ui.run().unwrap();

    Ok(())
}
