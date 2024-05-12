use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

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
    pub started: bool,
}

impl Bubbles {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos: Vector2 { x: 0.0, y: 0.0 },
            num: 0,
            els: vec![],
            dt: 0.0,
            next_dt: 0.0,
            started: false,
        }))
    }

    pub fn start(&mut self, pos: Vector2, num: usize) {
        self.pos = pos;
        self.num = num;
        self.els.clear();
        self.dt = 0.0;
        self.next_dt = 0.0;
        self.started = true;
    }

    pub fn set_pos(&mut self, pos: Vector2) {
        self.pos = pos;
    }

    pub fn update(&mut self, surface_verts: &SurfaceVerts, dt: f32) {
        if !self.started {
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
            self.started = false;
        }
    }

    pub fn draw<'a>(&mut self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        for el in &self.els {
            if el.size > 0.0 {
                d.draw_circle(el.pos.x as i32, el.pos.y as i32, el.size, el.color);
            }
        }
        d
    }
}

pub struct BubblesManager {
    bubbles_list: Vec<Rc<RefCell<Bubbles>>>,
}

impl BubblesManager {
    pub fn new() -> Self {
        Self {
            bubbles_list: Vec::new(),
        }
    }

    pub fn add_bubbles(&mut self) -> Rc<RefCell<Bubbles>> {
        let bubbles = Bubbles::new();
        self.bubbles_list.push(Rc::clone(&bubbles));
        bubbles
    }

    pub fn update(&mut self, surface_verts: &SurfaceVerts, dt: f32) {
        for bubbles in self.bubbles_list.iter() {
            bubbles.borrow_mut().update(&surface_verts, dt);
        }
    }

    pub fn draw<'a>(&self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        for bubbles in &self.bubbles_list {
            d = bubbles.borrow_mut().draw(d);
        }
        d
    }
}
