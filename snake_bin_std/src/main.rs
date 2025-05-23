use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use log::info;
use rand::Rng;
use slint::SharedString;
use snake_lib::{consts::LOOP_DELAY_MS, game::{key_to_direction, Direction, Snake}, get_snake_app_window};
use slint::ComponentHandle;

fn get_random_u64() -> u64 {
    let mut r = rand::rng();
    r.random::<u64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    info!("Starting...");

    let ui = get_snake_app_window();

    let ui_weak = ui.as_weak();

    std::thread::spawn(move || {
        let snake = Arc::new(Mutex::new(Snake::new(get_random_u64)));
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