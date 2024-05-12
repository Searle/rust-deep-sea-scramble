use std::f32;

use rand::Rng;
use raylib::prelude::*;

use crate::consts::*;
use crate::surface_verts::*;

pub struct Surface {
    pub pos: Vector2,
    pub step: i32,
    pub freq: f32,
    pub amplitude: f32,
}

impl Surface {
    pub fn new() -> Self {
        Self {
            pos: Vector2 { x: 0.0, y: 0.0 },
            step: 0,
            freq: 0.0,
            amplitude: 0.0,
        }
    }
}

pub struct Water {
    surfaces: Vec<Surface>,
    pub surface_verts: SurfaceVerts,
}

impl Water {
    pub fn new() -> Self {
        Self {
            surfaces: vec![Surface::new()],
            surface_verts: SurfaceVerts::new(),
        }
    }

    pub fn update(&mut self, arena_x: f32) -> Option<(i32, Vector2)> {
        let mut rng = rand::thread_rng();
        let mut result: Option<(i32, Vector2)> = None;
        loop {
            let surface = &self.surfaces[self.surfaces.len() - 1];
            let surface_x = surface.pos.x;
            if arena_x + surface_x >= WINDOW_WIDTH as f32 {
                self.surfaces
                    .retain(|surface| arena_x + surface.pos.x > -SURFACE_WIDTH as f32);
                self.surface_verts = get_surface_verts(&self.surfaces, arena_x);
                return result;
            }
            let step = rng.gen_range(-1..=1);
            let mut y = surface.pos.y + step as f32 * 50.0;
            y = y.max(150.0).min(400.0);
            let new_surface = Surface {
                pos: Vector2 {
                    x: surface_x + SURFACE_WIDTH as f32,
                    y,
                },
                step,
                freq: rng.gen_range(0.0..1.0),
                amplitude: rng.gen_range(0.0..1.0),
            };
            result = Some((
                new_surface.step,
                Vector2 {
                    x: new_surface.pos.x,
                    y: new_surface.pos.y,
                },
            ));
            self.surfaces.push(new_surface);
        }
    }

    pub fn draw<'a>(&mut self, d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        draw_surface_verts(d, &self.surface_verts)
    }
}
