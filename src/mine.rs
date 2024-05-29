use std::f32;

use raylib::prelude::*;

use crate::bubbles::*;
use crate::consts::*;
use crate::entity::Entity;
use crate::entity::EntityManager;
use crate::ship::*;
use crate::surface_verts::*;

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

pub struct Mine {
    pos: Vector2,
    launch_x: f32,
    dy: f32,
    bubble_id: usize,
    arena_x: f32,
    finished: bool,
}

impl Mine {
    pub fn new(surface_pos: Vector2, ship: &Ship) -> Self {
        Self {
            pos: Vector2 {
                x: surface_pos.x - SURFACE_WIDTH as f32 * 0.5,
                y: surface_pos.y,
            },
            launch_x: ship.pos.x + ship.pos.y - (WINDOW_HEIGHT as f32 - surface_pos.y),
            dy: 3.0,
            bubble_id: 0,
            arena_x: 0.0,
            finished: false,
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        arena_x: f32,
        bubbles_manager: &mut BubblesManager,
        ship: &Ship,
        surface_verts: &SurfaceVerts,
    ) {
        if self.finished {
            return;
        }
        self.arena_x = arena_x;

        if arena_x + self.pos.x < 50.0 {
            self.finished = true;
        }

        if arena_x + self.pos.x < self.launch_x {
            if self.pos.y > ship.pos.y {
                self.pos.y -= dt * 80.0;
            }
            self.dy = (self.dy * 0.995).max(0.5);
            self.pos.y += dt * 100.0 * self.dy;

            if bubbles_manager.is_finished(self.bubble_id) {
                self.bubble_id = bubbles_manager.insert(Bubbles::new(5));
            }
            bubbles_manager.set_pos(
                self.bubble_id,
                Vector2 {
                    x: arena_x + self.pos.x,
                    y: self.pos.y,
                },
            );
            return;
        }

        let x = arena_x + self.pos.x;
        let index = get_surface_verts_index(&surface_verts, x);
        let y = surface_verts.layer_a[index].y;
        self.pos.y = y;
    }

    fn draw<'d>(&self, mut d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        let vertices = get_mine_vertices(self.arena_x + self.pos.x, self.pos.y);
        d.draw_triangle_strip(&vertices, Color::DARKORANGE);
        d
    }
}

impl Entity for Mine {
    fn draw<'d>(&self, d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        self.draw(d)
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn set_pos(&mut self, _pos: Vector2) {
        // unused
    }
}

pub type MineManager = EntityManager<Mine>;
