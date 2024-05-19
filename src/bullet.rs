use std::f32;

use raylib::prelude::*;

use crate::consts::*;
use crate::entity::Entity;
use crate::surface_verts::*;

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

pub struct Bullet {
    pos: Vector2,
    posd: Vector2,
    vy: f32,
    finished: bool,
}

impl Bullet {
    pub fn new(pos: Vector2) -> Self {
        Self {
            pos,
            posd: Vector2 { x: 300.0, y: 10.0 },
            vy: 1.03,
            finished: false,
        }
    }

    // TODO: use _serface_verts
    pub fn update(&mut self, _surface_verts: &SurfaceVerts, dt: f32) {
        if self.finished {
            return;
        }

        self.pos.x = self.pos.x + dt * self.posd.x;
        self.pos.y = self.pos.y + dt * self.posd.y;
        self.vy *= 0.999;
        self.posd.y *= self.vy;
        if self.pos.x < 0.0
            || self.pos.y < 0.0
            || self.pos.x > WINDOW_WIDTH as f32
            || self.pos.y >= WINDOW_HEIGHT as f32
        {
            self.finished = true;
        }
    }

    pub fn draw<'a>(&self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        if !self.finished {
            d = draw_bullet(d, self.pos.x, self.pos.y);
        }
        d
    }
}

impl Entity for Bullet {
    fn update(&mut self, surface_verts: &SurfaceVerts, dt: f32) {
        self.update(surface_verts, dt);
    }

    fn draw<'a>(&self, d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        self.draw(d)
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn set_pos(&mut self, pos: Vector2) {
        self.pos = pos;
    }
}
