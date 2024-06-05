use std::f32;

use rand::Rng;
use raylib::prelude::*;

use crate::entity::{Entity, EntityManager};
use crate::surface_verts::SurfaceVerts;
use crate::{consts::*, Fish};

pub struct FishSwarm {
    fishes: Vec<Fish>,
    target_pos: Vector2,
    finished: bool,
}

fn make_new_target_pos(pos: Vector2, range: f32) -> Vector2 {
    let mut rng = rand::thread_rng();
    Vector2 {
        x: (pos.x + rng.gen_range(-range..range))
            .max(-20.0)
            .min(WINDOW_WIDTH as f32 + 20.0),
        y: (pos.y + rng.gen_range(-range..range))
            .max(100.0)
            .min(WINDOW_HEIGHT as f32 + 50.0),
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
            target_pos: Vector2::zero(),
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32, surface_verts: &SurfaceVerts) {
        if self.finished {
            return;
        }

        if self.target_pos.y == 0.0 && self.target_pos.x == 0.0 {
            self.target_pos = Vector2 { x: 100.0, y: 200.0 };
            self.fishes[0].set_target_pos(self.target_pos);
            self.fishes[0].set_pos(Vector2 { x: 200.0, y: 100.0 });
        } else {
            for i in 0..self.fishes.len() {
                self.fishes[i].update(dt, surface_verts);
                if self.fishes[i].has_reached_target() {
                    if i == 0 {
                        let pos = make_new_target_pos(self.fishes[0].pos, 200.0);
                        self.fishes[0].set_target_pos(pos);
                    } else {
                        let p = (i - 1) / 2;
                        let pos = make_new_target_pos(self.fishes[p].pos, 40.0);
                        self.fishes[i].set_target_pos(pos);
                    }
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
