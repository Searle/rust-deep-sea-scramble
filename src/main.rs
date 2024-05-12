use std::f32;

use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

mod bubbles;
mod consts;
mod mines;
mod ship;
mod surface_verts;
mod water;

use bubbles::*;
use consts::*;
use mines::*;
use ship::*;
use water::*;

fn draw_bullet(mut d: RaylibDrawHandle, bullet_x: f32, bullet_y: f32) -> RaylibDrawHandle {
    let x = bullet_x;
    let y = bullet_y;

    if x > 0.0 {
        let vertices = vec![
            Vector2 { x: x, y: y - 5.0 }, // Top vertex
            Vector2 { x: x - 5.0, y: y }, // Left vertex
            Vector2 { x: x + 5.0, y: y }, // Right vertex
            Vector2 { x: x, y: y + 5.0 }, // Bottom vertex
        ];

        d.draw_triangle_strip(&vertices, Color::LIGHTGREEN);
    }
    d
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Deep Sea Scramble!")
        .build();

    let mut arena_x = 0.0;

    let mut water = Water::new();
    let mut bubbles_manager = BubblesManager::new();
    let mut ship = Ship::new(&mut bubbles_manager);

    let mut bullet_x = 0.0;
    let mut bullet_y = 0.0;
    let mut bullet_dx = 0.0;
    let mut bullet_dy = 0.0;
    let mut bullet_vy = 0.0;

    let mut mines = Mines::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        arena_x = arena_x - dt * 100.0;

        if bullet_x > 0.0 {
            bullet_x = bullet_x + dt * bullet_dx;
            bullet_y = bullet_y + dt * bullet_dy;
            bullet_vy *= 0.999;
            bullet_dy *= bullet_vy;
            if bullet_x > WINDOW_WIDTH as f32 || bullet_y >= WINDOW_HEIGHT as f32 {
                bullet_x = 0.0;
            }
        }

        if let Some((step, surface_pos)) = water.update(arena_x) {
            if step == 0 {
                mines.add_mine(&mut bubbles_manager, surface_pos, &ship);
            }
        }

        mines.update(arena_x, &ship, &water.surface_verts, dt);
        ship.update(&water.surface_verts);
        bubbles_manager.update(&water.surface_verts, dt);

        // Keyboard

        if rl.is_key_down(KEY_UP) {
            ship.pos.y -= 1.0;
        }
        if rl.is_key_down(KEY_DOWN) {
            ship.pos.y += 1.0;
        }
        if rl.is_key_down(KEY_SPACE) && bullet_x == 0.0 {
            bullet_x = ship.pos.x + 15.0;
            bullet_y = ship.pos.y + 10.0;
            bullet_dx = 300.0;
            bullet_dy = 10.0;
            bullet_vy = 1.03;

            ship.start_bubbles();
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTSKYBLUE);
        d = water.draw(d);
        d = bubbles_manager.draw(d);
        d = mines.draw(d, arena_x);
        d = ship.draw(d);
        draw_bullet(d, bullet_x, bullet_y);
    }
}
