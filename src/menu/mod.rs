use raylib::prelude::*;

use crate::Scene;

pub struct Menu {}

impl Menu {
    pub fn draw(d: &mut RaylibDrawHandle<'_>, scene: &mut Scene) {
        d.draw_text(
            "Snake Game",
            (get_monitor_width(get_current_monitor()) / 2) - 170,
            200,
            70,
            Color::RED,
        );
        d.draw_rectangle(
            (get_monitor_width(get_current_monitor()) / 2) - 160,
            (get_monitor_height(get_current_monitor()) / 2) - 170,
            400,
            50,
            Color::GREEN,
        );

        d.draw_rectangle(
            (get_monitor_width(get_current_monitor()) / 2) - 160,
            (get_monitor_height(get_current_monitor()) / 2) - 100,
            400,
            50,
            Color::GREEN,
        );

        d.draw_text(
            "Start Game",
            (get_monitor_width(get_current_monitor()) / 2) - 150,
            (get_monitor_height(get_current_monitor()) / 2) - 170,
            50,
            Color::RED,
        );

        d.draw_text(
            "Quit",
            (get_monitor_width(get_current_monitor()) / 2) - 150,
            (get_monitor_height(get_current_monitor()) / 2) - 100,
            50,
            Color::RED,
        );

        d.draw_text("Start Game: Enter", 5, 5, 20, Color::RED);
        d.draw_text("Quit Game: Escape", 5, 25, 20, Color::RED);

        d.draw_text(
            "Made with love by Giorgi",
            (get_monitor_width(get_current_monitor()) / 2) - 90,
            (get_monitor_height(get_current_monitor())) - 50,
            20,
            Color::GREEN,
        );

        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            *scene = Scene::ACTGAME;
        }
    }
}
