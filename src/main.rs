use std::{
    sync::{Arc, Mutex},
    thread,
};

use actual_game::{Apple, Snake};
use ffi::{HideCursor, ToggleBorderlessWindowed, ToggleFullscreen};
use menu::Menu;
use rand::Rng;
use raylib::prelude::*;

pub mod actual_game;
pub mod menu;

pub enum Scene {
    MENU,
    ACTGAME,
}

fn main() {
    let (mut rl, thread) = raylib::init().title("Snake Game").build();
    unsafe {
        ToggleFullscreen();
        ToggleBorderlessWindowed();
        HideCursor();
    }

    let mut rng = rand::thread_rng();

    let random_snake_x = rng.gen_range(0..get_monitor_width(get_current_monitor()));
    let random_snake_y = rng.gen_range(0..get_monitor_height(get_current_monitor()));

    let mut scene = Scene::MENU;

    let snake = Arc::new(Mutex::new(Snake::new(
        random_snake_x,
        random_snake_y,
        30,
        10,
        Color::GREEN,
    )));

    let apple = Arc::new(Mutex::new(Apple::new(
        rng.gen_range(0..get_monitor_width(get_current_monitor())),
        rng.gen_range(0..get_monitor_height(get_current_monitor())),
        10,
        10,
        Color::RED,
    )));

    while !rl.window_should_close() {
        {
            let mut state = snake.lock().unwrap();
            state.moves(&mut rl);
            let mut apple_state = apple.lock().unwrap();
            apple_state.handle_collision(&mut state);
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        match scene {
            Scene::MENU => Menu::draw(&mut d, &mut scene),
            Scene::ACTGAME => {
                let state = snake.lock().unwrap();
                state.draw(&mut d);
                let apple_state = apple.lock().unwrap();
                apple_state.draw(&mut d);
            }
        };
    }
}
