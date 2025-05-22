use log::info;
use slint::{Color, ComponentHandle, Model};

use crate::{consts::GRID_SIZE, AppWindow, GameAdapter};

impl AppWindow {
    pub fn set_block_color(&self, x: usize, y: usize, color: Color) {
        let grid = self.global::<GameAdapter>().get_block_colors();
        if let Some(row_model_rc) = grid.row_data(y as usize) {
            if let Some(row_model) = row_model_rc.as_any()
                .downcast_ref::<slint::VecModel<slint::Color>>() 
            {
                row_model.set_row_data(x as usize, color);
            } else {
                info!("Failed to get row_model");
            }
        } else {
            info!("Failed to get row_model_rc");
        }
    }

    pub fn set_block_color_all(&self, color: Color) {
        for x in 0..GRID_SIZE.width as usize {
            for y in 0..GRID_SIZE.height as usize {
                self.set_block_color(x, y, color);
            }
        }
    }
}
