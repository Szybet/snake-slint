#![no_std]

extern crate alloc;
use alloc::vec;
use alloc::format;

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

