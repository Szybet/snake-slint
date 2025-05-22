use log::info;
use slint::{Color, ComponentHandle, Model, ModelRc, VecModel};

use crate::{AppWindow, GameAdapter, consts::GRID_SIZE};

impl AppWindow {
    pub fn set_block_color(&self, x: usize, y: usize, color: Color) {
        let grid = self.global::<GameAdapter>().get_block_colors();
        if let Some(row_model_rc) = grid.row_data(y as usize) {
            if let Some(row_model) = row_model_rc
                .as_any()
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
        let grid_model = VecModel::default();
        for _ in 0..GRID_SIZE.height as usize {
            let row_model = VecModel::from(vec![color; GRID_SIZE.width as usize]);
            grid_model.push(ModelRc::new(row_model));
        }
        self.global::<GameAdapter>()
            .set_block_colors(ModelRc::new(grid_model));
    }
}
