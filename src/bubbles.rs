use std::collections::HashMap;
use std::f32;

use rand::Rng;
use raylib::prelude::*;

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

pub struct BubblesManager {
    bubbles_map: HashMap<usize, Bubbles>,
    next_id: usize,
}

impl BubblesManager {
    pub fn new() -> Self {
        Self {
            bubbles_map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_bubbles(&mut self, num: usize) -> usize {
        let bubbles = Bubbles::new(num);
        self.next_id += 1;
        self.bubbles_map.insert(self.next_id, bubbles);
        self.next_id
    }

    pub fn set_pos(&mut self, id: usize, pos: Vector2) {
        if let Some(bullet) = self.bubbles_map.get_mut(&id) {
            bullet.pos = pos;
        }
    }

    pub fn is_finished(&self, id: usize) -> bool {
        self.bubbles_map
            .get(&id)
            .map_or(true, |bubbles| bubbles.finished)
    }

    pub fn update(&mut self, surface_verts: &SurfaceVerts, dt: f32) {
        for bubbles in self.bubbles_map.values_mut() {
            bubbles.update(&surface_verts, dt);
        }

        self.bubbles_map.retain(|_, bubbles| !bubbles.finished);
    }

    pub fn draw<'a>(&self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        for bubbles in self.bubbles_map.values() {
            d = bubbles.draw(d);
        }
        d
    }
}
