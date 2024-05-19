use std::f32;

use rand::Rng;
use raylib::prelude::*;

use crate::entity::Entity;
use crate::surface_verts::*;

struct Bubble {
    pos: Vector2,
    size: f32,
    color: Color,
    ax: f32,
    ay: f32,
    vx: f32,
    vy: f32,
    dt0: f32,
    dt: f32,
}

pub struct Bubbles {
    pos: Vector2,
    num: usize,
    els: Vec<Bubble>,
    dt: f32,
    next_dt: f32,
    finished: bool,
}

impl Bubbles {
    pub fn new(num: usize) -> Self {
        Self {
            pos: Vector2::zero(),
            num,
            els: vec![],
            dt: 0.0,
            next_dt: 0.0,
            finished: false,
        }
    }

    pub fn update(&mut self, surface_verts: &SurfaceVerts, dt: f32) {
        if self.finished {
            return;
        }

        let mut rng = rand::thread_rng();

        let mut found_one = false;
        for el in &mut self.els {
            if el.dt0 == 0.0 {
                el.size -= 0.1;
                if el.size >= 1.0 {
                    found_one = true;
                }
                continue;
            }
            found_one = true;
            el.dt += dt;
            let age = el.dt - el.dt0;
            let vx_next = el.vx + el.ax * dt;
            el.pos.x += ((el.vx + vx_next) * 0.5
                + (age * 1.0 * 2.0).sin() * (age * 1.0 * 3.0).cos() * 40.0)
                * dt;
            el.vx = vx_next;
            let vy_next = el.vy + el.ay * dt;
            el.pos.y += ((el.vy + vy_next) * 0.5) * dt;
            el.vy = vy_next;

            let index = get_surface_verts_index(&surface_verts, el.pos.x);
            let y = surface_verts.layer_a[index].y;

            if el.pos.y < y {
                el.dt0 = 0.0;
            }
        }
        self.dt += dt;
        if self.els.len() < self.num {
            found_one = true;
            if self.next_dt <= self.dt {
                self.els.push(Bubble {
                    pos: self.pos,
                    size: rng.gen_range(2.0..10.0),
                    color: Color::WHITE.alpha(0.5),
                    ax: 0.0,
                    ay: -40.0,
                    vx: 0.0,
                    vy: -100.0,
                    dt0: self.dt + rng.gen_range(0.0..2.0),
                    dt: 0.0,
                });
                self.next_dt = self.dt + 0.1 + rng.gen_range(0.0..0.1);
            }
        }
        if !found_one {
            self.finished = true;
        }
    }

    pub fn draw<'a>(&self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        if !self.finished {
            for el in &self.els {
                if el.size > 0.0 {
                    d.draw_circle_v(el.pos, el.size, el.color);
                }
            }
        }
        d
    }
}

impl Entity for Bubbles {
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
