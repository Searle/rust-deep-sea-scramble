use rand::Rng;
use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

mod bubbles;
mod bullet;
mod consts;
mod entity;
mod fish;
mod fish_swarm;
mod mine;
mod ship;
mod surface_verts;
mod water;

use bubbles::*;
use bullet::*;
use consts::*;
use fish_swarm::*;
use mine::*;
use ship::*;
use water::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Deep Sea Scramble!")
        .build();

    let mut arena_x = 0.0;

    let mut water = Water::new();
    let mut bubbles_manager = BubblesManager::new();
    let mut bullet_manager = BulletManager::new();
    let mut fish_swarm_manager = FishSwarmManager::new();
    let mut mine_manager = MineManager::new();
    let mut ship = Ship::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        arena_x -= dt * 100.0;

        mine_manager.update(|entity, _| {
            entity.update(
                dt,
                arena_x,
                &mut bubbles_manager,
                &ship,
                &water.surface_verts,
            );
        });
        if let Some((step, surface_pos)) = water.update(arena_x) {
            if step == 0 {
                mine_manager.insert(Mine::new(surface_pos, &ship));
            }
        }

        let mut has_lead_fish_in_last_sector = false;
        fish_swarm_manager.update(|fish_swarm, _| {
            if fish_swarm.update(dt, &water.surface_verts) {
                // no fish in swarm
            }
            if fish_swarm.in_last_sector() {
                has_lead_fish_in_last_sector = true;
            }
        });

        if !has_lead_fish_in_last_sector {
            let mut rng = rand::thread_rng();
            fish_swarm_manager.insert(FishSwarm::new(rng.gen_range(10..30), rng.gen_range(0..9)));
        }

        bubbles_manager.update(|bubbles, _| bubbles.update(dt, &water.surface_verts));
        bullet_manager.update(|bullet, _| bullet.update(dt));
        ship.update(&mut bubbles_manager, &water.surface_verts);

        // Keyboard
        if rl.is_key_down(KEY_UP) {
            ship.pos.y -= 1.0;
        }
        if rl.is_key_down(KEY_DOWN) {
            ship.pos.y += 1.0;
        }
        if rl.is_key_down(KEY_SPACE) {
            ship.start_bullet(&mut bubbles_manager, &mut bullet_manager);
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTSKYBLUE);
        let d = water.draw(d);
        let d = fish_swarm_manager.draw(d);
        let d = bullet_manager.draw(d);
        let d = bubbles_manager.draw(d);
        let d = mine_manager.draw(d);
        ship.draw(d);
    }
}
