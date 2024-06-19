use lazy_static::lazy_static;
use std::f32;
use std::sync::atomic::{AtomicUsize, Ordering};

use rand::Rng;
use raylib::prelude::*;

use crate::consts::*;
use crate::entity::{Entity, EntityManager};
use crate::fish::{Fish, FishManager};
use crate::surface_verts::SurfaceVerts;

pub struct FishSwarm {
    fish_manager: FishManager,
    target_x: f32,
    finished: bool,
    ofs_x: f32,
}

lazy_static! {
    static ref INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
}

fn make_new_target_pos(pos: Vector2, ofs_x: f32, range: f32) -> Vector2 {
    let mut rng = rand::thread_rng();
    Vector2 {
        x: (pos.x + rng.gen_range(-range..range) - ofs_x)
            .max(-30.0)
            .min(WINDOW_WIDTH as f32 + 20.0),
        y: (pos.y + rng.gen_range(-range..range))
            .max(300.0)
            .min(WINDOW_HEIGHT as f32 + 20.0),
    }
}

impl FishSwarm {
    pub fn new(count: i32, ofs_x: f32) -> Self {
        INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
        let mut rng = rand::thread_rng();
        let mut fish_manager = FishManager::new();
        let fish_scale = rng.gen_range(1.0..3.0);
        let fish_type = INSTANCE_COUNT.load(Ordering::SeqCst);
        for _ in 0..count {
            fish_manager.insert(Fish::new(Vector2::zero(), fish_scale, fish_type));
        }
        Self {
            fish_manager,
            target_x: 0.0,
            finished: false,
            ofs_x,
        }
    }

    pub fn update(&mut self, dt: f32, surface_verts: &SurfaceVerts) -> bool {
        if self.finished {
            return true;
        }

        self.target_x += dt;
        let tx = self.target_x * 4.0;

        let mut poss: Vec<Vector2> = vec![];

        self.finished = self.fish_manager.update(|fish, i| {
            if fish.pos.y == 0.0 {
                let mut rng = rand::thread_rng();
                fish.pos = Vector2 {
                    x: WINDOW_WIDTH as f32 + 120.0 + self.ofs_x,
                    y: rng.gen_range(100.0..WINDOW_HEIGHT as f32),
                }
            }
            fish.update(dt, surface_verts, i as usize);
            if fish.has_reached_target() {
                if i == 0 {
                    let pos = make_new_target_pos(fish.pos, tx, 200.0);
                    fish.set_target_pos(pos);
                } else {
                    let p = ((i - 1) / 2) as usize;
                    let pos = make_new_target_pos(poss[p], tx, 40.0);
                    fish.set_target_pos(pos);
                }
            }
            poss.push(fish.pos);
        });

        self.finished
    }

    pub fn draw<'d>(&self, mut d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        self.fish_manager.draw(d)
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
