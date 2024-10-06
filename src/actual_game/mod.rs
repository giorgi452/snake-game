use std::ops::RangeBounds;

use rand::Rng;
use raylib::prelude::*;

pub struct Snake {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
}

pub struct Apple {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
}

impl Apple {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }

    pub fn handle_collision(&mut self, snake: &mut Snake) {
        let mut rng = rand::thread_rng();
        if (snake.x - 50..=snake.x + 50).contains(&self.x)
            && (snake.y - 50..=snake.y + 50).contains(&self.y)
        {
            self.x = rng.gen_range(0..get_monitor_width(get_current_monitor()));
            self.y = rng.gen_range(0..get_monitor_height(get_current_monitor()));
            snake.width += 10;
        }
    }
}

impl Snake {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }

    pub fn moves(&mut self, d: &mut RaylibHandle) {
        if d.is_key_down(KeyboardKey::KEY_D) {
            // let c = self.height.clone();
            // let h = self.width.clone();
            // self.width = c.clone();
            // self.height = h.clone();
            self.x += 1;
        } else if d.is_key_down(KeyboardKey::KEY_A) {
            // let c = self.height.clone();
            // let h = self.width.clone();
            // self.width = c.clone();
            // self.height = h.clone();
            self.x -= 1;
        } else if d.is_key_down(KeyboardKey::KEY_W) {
            // let c = self.width.clone();
            // self.width = self.height.clone();
            // self.height = c.clone();
            self.y -= 1;
        } else if d.is_key_down(KeyboardKey::KEY_S) {
            // let c = self.width.clone();
            // self.width = self.height.clone();
            // self.height = c.clone();
            self.y += 1;
        }
    }
}
