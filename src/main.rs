use std::f32;

use rand::Rng;
use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

mod bubbles;
mod consts;
mod mines;
mod ship;
mod surface;
mod surface_verts;

use bubbles::*;
use consts::*;
use mines::*;
use ship::*;
use surface::*;
use surface_verts::*;

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

    let mut surfaces: Vec<Surface> = vec![Surface::new()];
    let mut rng = rand::thread_rng();

    let mut arena_x = 0.0;

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

        loop {
            let surface = &surfaces[surfaces.len() - 1];
            let surface_x = surface.pos.x;
            if arena_x + surface_x >= WINDOW_WIDTH as f32 {
                break;
            }
            let step = rng.gen_range(-1..=1);
            let mut y = surface.pos.y + step as f32 * 50.0;
            y = y.max(150.0).min(400.0);
            let new_surface = Surface {
                pos: Vector2 {
                    x: surface_x + SURFACE_WIDTH as f32,
                    y,
                },
                freq: rng.gen_range(0.0..1.0),
                amplitude: rng.gen_range(0.0..1.0),
            };
            if step == 0 {
                mines.add_mine(&mut bubbles_manager, &new_surface, &ship);
            }
            surfaces.push(new_surface);
        }

        surfaces.retain(|surface| arena_x + surface.pos.x > -SURFACE_WIDTH as f32);

        let surface_verts = get_surface_verts(&surfaces, arena_x);

        mines.update(arena_x, &ship, &surface_verts, dt);
        ship.update(&surface_verts);
        bubbles_manager.update(&surface_verts, dt);

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
        d = draw_surface_verts(d, &surface_verts);
        d = bubbles_manager.draw(d);
        d = mines.draw(d, arena_x);
        d = ship.draw(d);
        draw_bullet(d, bullet_x, bullet_y);
    }
}
