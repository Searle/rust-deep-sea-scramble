use std::f32;

use rand::Rng;
use raylib::prelude::*;

use crate::entity::{Entity, EntityManager};
use crate::surface_verts::SurfaceVerts;
use crate::{consts::*, Fish};

pub struct FishSwarm {
    fishes: Vec<Fish>,
    target_x: f32,
    finished: bool,
}

fn make_new_target_pos(pos: Vector2, ofs_x: f32, range: f32) -> Vector2 {
    let mut rng = rand::thread_rng();
    Vector2 {
        x: (pos.x + rng.gen_range(-range..range) - ofs_x)
            .max(-20.0)
            .min(WINDOW_WIDTH as f32 + 20.0),
        y: (pos.y + rng.gen_range(-range..range))
            .max(300.0)
            .min(WINDOW_HEIGHT as f32 + 20.0),
    }
}

impl FishSwarm {
    pub fn new(count: i32) -> Self {
        let mut fishes = vec![];
        for _ in 0..count {
            fishes.push(Fish::new(Vector2::zero()));
        }
        Self {
            fishes,
            target_x: 0.0,
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32, surface_verts: &SurfaceVerts) {
        if self.finished {
            return;
        }

        self.target_x += dt;
        let tx = self.target_x * 4.0;

        for i in 0..self.fishes.len() {
            if self.fishes[i].pos.x == 0.0 && self.fishes[i].pos.y == 0.0 {
                let mut rng = rand::thread_rng();
                self.fishes[i].pos = Vector2 {
                    x: WINDOW_WIDTH as f32 + 120.0,
                    y: rng.gen_range(100.0..WINDOW_HEIGHT as f32),
                }
            }
            self.fishes[i].update(dt, surface_verts);
            if self.fishes[i].has_reached_target() {
                if i == 0 {
                    let pos = make_new_target_pos(self.fishes[0].pos, tx, 200.0);
                    self.fishes[0].set_target_pos(pos);
                } else {
                    let p = (i - 1) / 2;
                    let pos = make_new_target_pos(self.fishes[p].pos, tx, 40.0);
                    self.fishes[i].set_target_pos(pos);
                }
            }
        }
    }

    pub fn draw<'d>(&self, mut d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        for fish in &self.fishes {
            d = fish.draw(d)
        }
        d
    }
}

impl Entity for FishSwarm {
    fn draw<'d>(&self, d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        self.draw(d)
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn set_pos(&mut self, pos: Vector2) {
        // self.pos = pos;
    }
}

pub type FishSwarmManager = EntityManager<FishSwarm>;
