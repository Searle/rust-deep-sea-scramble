use std::cell::RefCell;
use std::rc::Rc;

use raylib::prelude::*;

use crate::bubbles::*;
use crate::bullet::*;
use crate::consts::*;
use crate::entity::EntityManager;
use crate::surface_verts::*;

pub fn get_ship_vertices(x: f32, y: f32) -> Vec<Vector2> {
    vec![
        Vector2 { x: x, y: y - 5.0 }, // Rear top
        Vector2 {
            x: x + 0.0,
            y: y + 5.0,
        }, // Rear bottom
        Vector2 {
            x: x + 15.0,
            y: y - 10.0,
        }, // Upper mid body
        Vector2 {
            x: x + 15.0,
            y: y + 10.0,
        }, // Lower mid body
        Vector2 {
            x: x + 30.0,
            y: y - 5.0,
        }, // Top near cannon
        Vector2 {
            x: x + 30.0,
            y: y + 5.0,
        }, // Start of the front cannon
        Vector2 {
            x: x + 40.0,
            y: y - 5.0,
        }, // Cockpit top back
        Vector2 {
            x: x + 45.0,
            y: y + 5.0,
        }, // End of the front cannon
        Vector2 {
            x: x + 45.0,
            y: y - 5.0,
        }, // Cockpit top front
    ]
}

pub struct Ship {
    pub pos: Vector2,
    bubbles_id: usize,
    bullet_id: usize,
    y_ofs: f32,
}

impl Ship {
    pub fn new(bubbles_manager: &mut EntityManager<Bubbles>) -> Self {
        Self {
            pos: Vector2 {
                x: 100.0,
                y: WINDOW_HEIGHT as f32 - 100.0,
            },
            bubbles_id: 0,
            bullet_id: 0,
            y_ofs: 0.0,
        }
    }

    pub fn update(
        &mut self,
        bubbles_manager: &mut EntityManager<Bubbles>,
        surface_verts: &SurfaceVerts,
    ) {
        let ship_index = get_surface_verts_index(&surface_verts, self.pos.x);
        self.y_ofs = surface_verts.layer_c[ship_index].y - surface_verts.layer_b[ship_index].y;

        let ship_y_min = surface_verts.layer_a[ship_index].y + 30.0;
        let ship_y_max = WINDOW_HEIGHT as f32 - 30.0;
        if self.pos.y < ship_y_min {
            let diff = ship_y_min - self.pos.y;
            self.pos.y += diff.min(2.0);
        }
        if self.pos.y > ship_y_max {
            let diff = self.pos.y - ship_y_max;
            self.pos.y -= diff.min(2.0);
        }

        bubbles_manager.set_pos(
            self.bubbles_id,
            Vector2 {
                x: self.pos.x + 10.0,
                y: self.pos.y,
            },
        );
    }

    pub fn draw<'a>(&mut self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        let vertices = get_ship_vertices(self.pos.x, self.pos.y);
        d.draw_triangle_strip(&vertices, Color::WHITESMOKE);
        d
    }

    pub fn start_bullet(
        &mut self,
        bubbles_manager: &mut EntityManager<Bubbles>,
        bullet_manager: &mut EntityManager<Bullet>,
    ) {
        if bullet_manager.is_finished(self.bullet_id) {
            self.bullet_id = bullet_manager.insert(Bullet::new(Vector2 {
                x: self.pos.x + 15.0,
                y: self.pos.y + 10.0,
            }));
            self.bubbles_id = bubbles_manager.insert(Bubbles::new(20));
        }
    }
}
