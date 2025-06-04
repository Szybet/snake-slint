use crate::Size;
use slint::Color;

pub const GRID_SIZE_SIMPLE: i32 = 10;
pub const GRID_SIZE: Size = Size {
    width: GRID_SIZE_SIMPLE,
    height: GRID_SIZE_SIMPLE,
};
pub const BLOCK_BORDER_WIDTH: f32 = 1.0;
pub const BLOCK_BORDER_COLOR: Color = Color::from_rgb_u8(0, 0, 0);
pub const BLOCK_COLOR_USED: Color = Color::from_rgb_u8(0, 0, 0);

cfg_if::cfg_if! {
    if #[cfg(any(
        target_os = "none"
    ))] {
        pub const BLOCK_COLOR_UNUSED: Color = Color::from_rgb_u8(255, 255, 255);
        pub const LOOP_DELAY_MS: u64 = 10;
    } else {
        pub const BLOCK_COLOR_UNUSED: Color = Color::from_rgb_u8(0, 153, 51);
        pub const LOOP_DELAY_MS: u64 = 400;
    }
}