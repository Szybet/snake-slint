use log::info;
use rand::RngCore;
use rand::SeedableRng;
use slint::{ComponentHandle, SharedString, platform::Key};

use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use crate::{
    AppWindow, GameAdapter,
    consts::{BLOCK_COLOR_UNUSED, BLOCK_COLOR_USED, GRID_SIZE},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}

#[derive(Default, Debug, Clone)]
pub struct Snake {
    body: Vec<SnakePart>,
    ball: Ball,
    end_game: bool,
    platform_random: Option<fn() -> u64>,
}

impl Snake {
    pub fn new(platform_random: fn() -> u64) -> Self {
        let mut snake = Snake::default();
        // Head
        let mut snake_head = SnakePart::default();
        snake_head.x = (GRID_SIZE.width / 2) as usize;
        snake_head.y = (GRID_SIZE.height / 2) as usize;
        snake_head.dir = Direction::Right;
        snake.body.push(snake_head);
        // Another part to see where it's going
        let mut snake_part_first = SnakePart::default();
        snake_part_first.x = (GRID_SIZE.width / 2) as usize;
        snake_part_first.y = (GRID_SIZE.height / 2) as usize - 1;
        snake_part_first.dir = Direction::Right;
        snake.body.push(snake_part_first);
        // Platform random - before ball
        snake.platform_random = Some(platform_random);
        // Ball
        snake.random_ball_location();
        // Endgame
        snake.end_game = false;

        snake
    }
    pub fn game_loop(&mut self, dir_arg: &mut Direction) {
        info!("Gameloop received dir: {:?}", dir_arg);

        if self.end_game {
            if *dir_arg != Direction::None {
                *self = Snake::new(self.platform_random.unwrap());
            }
            return;
        }

        // Capture and reset direction
        let dir = *dir_arg;
        *dir_arg = Direction::None;

        // Calculate new head direction without holding mutable reference
        let current_dir = self.body[0].dir.clone();
        let new_dir = match dir {
            Direction::Up if !matches!(current_dir, Direction::Down) => Direction::Up,
            Direction::Down if !matches!(current_dir, Direction::Up) => Direction::Down,
            Direction::Left if !matches!(current_dir, Direction::Right) => Direction::Left,
            Direction::Right if !matches!(current_dir, Direction::Left) => Direction::Right,
            _ => current_dir,
        };

        // Calculate new head position
        let (width, height) = (GRID_SIZE.width as usize, GRID_SIZE.height as usize);
        let (mut new_x, mut new_y) = (self.body[0].x, self.body[0].y);
        match new_dir {
            Direction::Up => {
                new_x = (new_x as isize - 1).rem_euclid(width as isize) as usize;
            }
            Direction::Down => {
                new_x = (new_x + 1) % width;
            }
            Direction::Left => {
                new_y = (new_y as isize - 1).rem_euclid(height as isize) as usize;
            }
            Direction::Right => {
                new_y = (new_y + 1) % height;
            }
            Direction::None => {}
        }

        // Update head position and direction directly
        self.body[0].dir = new_dir;
        let prev_head_pos = (self.body[0].x, self.body[0].y);
        self.body[0].x = new_x;
        self.body[0].y = new_y;

        // Move body parts using previous positions
        let mut last_pos = prev_head_pos;
        for part in self.body.iter_mut().skip(1) {
            let current_pos = (part.x, part.y);
            part.x = last_pos.0;
            part.y = last_pos.1;
            last_pos = current_pos;
        }

        // Check self-collision with immutable borrow
        if self
            .body
            .iter()
            .skip(1)
            .any(|p| p.x == self.body[0].x && p.y == self.body[0].y)
        {
            self.end_game = true;
            return;
        }

        // Check ball collision
        if self.body[0].x == self.ball.x && self.body[0].y == self.ball.y {
            if let Some(last) = self.body.last() {
                self.body.push(SnakePart {
                    x: last.x,
                    y: last.y,
                    dir: last.dir.clone(),
                });
            }
            self.random_ball_location();
        }
    }

    pub fn draw(&self, ui: &AppWindow) {
        ui.set_block_color_all(BLOCK_COLOR_UNUSED);
        for part in &self.body {
            ui.set_block_color(part.x, part.y, BLOCK_COLOR_USED);
        }
        ui.set_block_color(self.ball.x, self.ball.y, BLOCK_COLOR_USED);
        if self.end_game {
            ui.global::<GameAdapter>()
                .set_overlay_text(format!("Score: {}", self.body.len() - 2).into());
            ui.global::<GameAdapter>().set_overlay_visible(true);
        } else {
            ui.global::<GameAdapter>().set_overlay_visible(false);
        }
    }

    pub fn random_ball_location(&mut self) {
        // info!("&self.body is: {:#?}", &self.body);
        // let mut c = 0;
        loop {
            self.ball.x =
                rand_range(0, GRID_SIZE.width as u32, self.platform_random.unwrap()) as usize;
            self.ball.y =
                rand_range(0, GRID_SIZE.height as u32, self.platform_random.unwrap()) as usize;

            // info!("Iterating random: {}x{}", self.ball.x, self.ball.y);

            let mut fine = true;
            for part in &self.body {
                if self.ball.x == part.x && self.ball.y == part.y {
                    fine = false;
                    info!("Matched! for part: {}x{}", part.x, part.y);
                    break;
                }
            }
            if fine {
                break;
            }

            // c = c + 1;
            // if c > 300 {
                // panic!("Did not found ball location!");
            //}
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SnakePart {
    x: usize,
    y: usize,
    dir: Direction,
}

#[derive(Default, Debug, Clone)]
pub struct Ball {
    x: usize,
    y: usize,
}

pub fn key_to_direction(key: SharedString) -> Direction {
    if key == "a" {
        return Direction::Left;
    }
    if key == "s" {
        return Direction::Down;
    }
    if key == "d" {
        return Direction::Right;
    }
    if key == "w" {
        return Direction::Up;
    }
    Direction::None
}

pub fn rand_range(min: u32, max: u32, platform_random: fn() -> u64) -> u32 {
    if min >= max {
        info!("min must be less than max");
        return 0;
    }

    min + (get_random(platform_random) % (max - min))
}

fn get_random(platform_random: fn() -> u64) -> u32 {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(platform_random());
    rng.next_u32()
}
