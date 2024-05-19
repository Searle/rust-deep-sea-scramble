use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use raylib::prelude::*;

use crate::bubbles::*;
use crate::consts::*;
use crate::ship::*;
use crate::surface_verts::*;

use crate::BubblesManager;

fn get_mine_vertices(x: f32, y: f32) -> Vec<Vector2> {
    vec![
        Vector2 { x: x, y: y + 2.0 }, // Bottom center of the mine base
        Vector2 {
            x: x + 6.0,
            y: y + 6.0,
        }, // Right edge of the mine base
        Vector2 {
            x: x - 6.0,
            y: y + 6.0,
        }, // Left edge of the mine base
        Vector2 {
            x: x + 6.0,
            y: y - 5.0,
        }, // Upper right body
        Vector2 {
            x: x - 6.0,
            y: y - 5.0,
        }, // Upper left body
        Vector2 { x: x, y: y - 12.0 }, // Tip of the mine
    ]
}

struct Mine {
    pos: Vector2,
    launch_x: f32,
    dy: f32,
    bubble_id: usize,
}

impl Mine {
    fn update(
        &mut self,
        arena_x: f32,
        bubbles_manager: &mut BubblesManager,
        ship: &Ship,
        surface_verts: &SurfaceVerts,
        dt: f32,
    ) {
        if arena_x + self.pos.x < self.launch_x {
            if self.pos.y > ship.pos.y {
                self.pos.y -= dt * 80.0;
            }
            self.dy = (self.dy * 0.995).max(0.5);
            self.pos.y += dt * 100.0 * self.dy;

            if bubbles_manager.is_finished(self.bubble_id) {
                self.bubble_id = bubbles_manager.add_bubbles(5);
            }
            bubbles_manager.set_pos(
                self.bubble_id,
                Vector2 {
                    x: arena_x + self.pos.x,
                    y: self.pos.y,
                },
            );
        } else {
            let x = arena_x + self.pos.x;
            let index = get_surface_verts_index(&surface_verts, x);
            let y = surface_verts.layer_a[index].y;
            self.pos.y = y;
        }
    }

    fn draw<'a>(&mut self, mut d: RaylibDrawHandle<'a>, arena_x: f32) -> RaylibDrawHandle<'a> {
        let vertices = get_mine_vertices(arena_x + self.pos.x, self.pos.y);
        d.draw_triangle_strip(&vertices, Color::DARKORANGE);
        d
    }
}

pub struct Mines {
    mine_list: Vec<Mine>,
}

impl Mines {
    pub fn new() -> Self {
        Self {
            mine_list: Vec::new(),
        }
    }

    pub fn add_mine(
        &mut self,
        bubbles_manager: &mut BubblesManager,
        surface_pos: Vector2,
        ship: &Ship,
    ) {
        self.mine_list.push(Mine {
            pos: Vector2 {
                x: surface_pos.x - SURFACE_WIDTH as f32 * 0.5,
                y: surface_pos.y,
            },
            launch_x: ship.pos.x + ship.pos.y - (WINDOW_HEIGHT as f32 - surface_pos.y),
            dy: 3.0,
            bubble_id: 0,
        });
    }

    pub fn update(
        &mut self,
        arena_x: f32,
        bubbles_manager: &mut BubblesManager,
        ship: &Ship,
        surface_verts: &SurfaceVerts,
        dt: f32,
    ) {
        for mine in self.mine_list.iter_mut() {
            mine.update(arena_x, bubbles_manager, &ship, &surface_verts, dt);
        }

        self.mine_list.retain(|mine| {
            arena_x + mine.pos.x >= -20.0 && mine.pos.y < WINDOW_HEIGHT as f32 + 20.0
        });
    }

    pub fn draw<'a>(&mut self, mut d: RaylibDrawHandle<'a>, arena_x: f32) -> RaylibDrawHandle<'a> {
        for mine in self.mine_list.iter_mut() {
            d = mine.draw(d, arena_x);
        }
        d
    }
}
