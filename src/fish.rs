use std::f32;

use raylib::prelude::*;

use crate::{
    consts::WINDOW_HEIGHT,
    entity::{Entity, EntityManager},
    surface_verts::{get_surface_verts_index, SurfaceVerts},
};

use std::f32::consts::PI;

const DEBUG: bool = false;

fn rotate_point(x: f32, y: f32, angle: f32, origin_x: f32, origin_y: f32) -> Vector2 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    let translated_x = x - origin_x;
    let translated_y = y - origin_y;

    let rotated_x = translated_x * cos_theta - translated_y * sin_theta;
    let rotated_y = translated_x * sin_theta + translated_y * cos_theta;

    Vector2 {
        x: rotated_x + origin_x,
        y: rotated_y + origin_y,
    }
}

const COLORS: [Color; 4] = [
    Color::LIGHTBLUE,
    Color::LIGHTSALMON,
    Color::LIGHTSLATEGRAY,
    Color::LIGHTYELLOW,
];

fn draw_fish(
    mut d: RaylibDrawHandle,
    wobble: f32,
    fish_x: f32,
    fish_y: f32,
    rotation: f32,
    scale: f32,
    fish_type: i32,
) -> RaylibDrawHandle {
    let pos_wobble = ((wobble * 1.0).sin() + 1.0) * 0.5;
    let tail_wobble = ((wobble * 30.0 + fish_x + fish_y).sin() + 1.0) * 0.5;

    let x = fish_x + pos_wobble * scale * 0.25;
    let y = fish_y + pos_wobble * scale;

    let vertices = vec![
        Vector2 {
            x: x - (7.0 + tail_wobble) * scale,
            y: y - 2.0 * scale,
        },
        Vector2 {
            x: x - (7.0 + tail_wobble) * scale,
            y: y + 2.0 * scale,
        },
        Vector2 {
            x: x - 5.0 * scale,
            y: y - 0.5 * scale,
        },
        Vector2 {
            x: x - 5.0 * scale,
            y: y + 0.5 * scale,
        },
        Vector2 {
            x: x - 3.0 * scale,
            y: y - 2.0 * scale,
        },
        Vector2 {
            x: x - 3.0 * scale,
            y: y + 2.0 * scale,
        },
        Vector2 {
            x: x - 2.0 * scale,
            y: y - 2.0 * scale,
        },
        Vector2 {
            x: x - 2.0 * scale,
            y: y + 2.0 * scale,
        },
        Vector2 {
            x: x + 1.0 * scale,
            y: y - 0.5 * scale,
        },
        Vector2 {
            x: x + 1.0 * scale,
            y: y + 0.5 * scale,
        },
    ];

    let rotated_vertices: Vec<Vector2> = vertices
        .iter()
        .map(|v| rotate_point(v.x, v.y, rotation, x, y))
        .collect();

    d.draw_triangle_strip(
        &rotated_vertices,
        COLORS[(fish_type % (COLORS.len() as i32)) as usize],
    );
    d
}

pub struct Fish {
    pub pos: Vector2,
    finished: bool,
    wobble: f32,
    target_pos: Vector2,
    direction: f32,
    pub target_reached: bool,
    draw_pos_y: f32,
    scale: f32,
    fish_type: usize,
    fish_index: usize,
}

const MAX_SPEED: f32 = 40.0;

impl Fish {
    pub fn new(pos: Vector2, scale: f32, fish_type: usize) -> Self {
        Self {
            pos,
            finished: false,
            wobble: 0.0,
            target_pos: Vector2::zero(),
            direction: 0.0,
            target_reached: true,
            draw_pos_y: 0.0,
            scale,
            fish_type,
            fish_index: 0,
        }
    }

    pub fn update(&mut self, dt: f32, surface_verts: &SurfaceVerts, index: usize) {
        if self.finished {
            return;
        }

        self.fish_index = index;

        let fish_index = get_surface_verts_index(&surface_verts, self.pos.x);
        let surface_y = surface_verts.layer_a[fish_index].y + 30.0;

        let mut dir_change_fact = 0.005;
        if surface_y > self.target_pos.y {
            self.target_pos.y = surface_y;
            self.target_pos.x = self.pos.x;
            dir_change_fact = 0.6;
        }

        let dx = self.target_pos.x - self.pos.x;
        let dy = self.target_pos.y - self.pos.y;

        let direction = f32::atan2(dy, dx);

        let mut diff = direction - self.direction;
        if diff.abs() > PI {
            diff = (diff.abs() - 2.0 * PI) * diff.signum();
        }

        self.direction += diff * dir_change_fact;

        let max_dy = MAX_SPEED * self.direction.sin();
        let max_dx = MAX_SPEED * self.direction.cos();

        let dx = if dx < 0.0 {
            dx.max(-max_dx.abs() * dt)
        } else {
            dx.min(max_dx.abs() * dt)
        };
        let dy = if dy < 0.0 {
            dy.max(-max_dy.abs() * dt)
        } else {
            dy.min(max_dy.abs() * dt)
        };

        let effect = ((self.pos.y - surface_y) / (WINDOW_HEIGHT as f32 - surface_y))
            .max(0.0)
            .min(1.0);
        let y_surface_effect = surface_y + effect * (self.pos.y - surface_y);
        self.draw_pos_y = self.draw_pos_y * 0.95 + y_surface_effect * 0.05;

        self.pos.x += dx;
        if self.pos.x <= -90.0 {
            self.finished = true;
        }

        self.pos.y += dy;
        if self.pos.y + self.draw_pos_y < surface_y {
            self.pos.y = surface_y - self.draw_pos_y;
            self.target_reached = true;
        }

        if (self.target_pos.x - self.pos.x).abs() < 10.0
            && (self.target_pos.y - self.pos.y).abs() < 10.0
        {
            self.target_reached = true;
        }

        self.wobble += dt;
    }

    pub fn draw<'d>(&self, mut d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        if DEBUG {
            d.draw_circle(
                self.target_pos.x as i32,
                (self.target_pos.y + self.draw_pos_y - self.pos.y) as i32,
                3.0,
                Color::DARKGREEN,
            );

            let x = self.target_pos.x as i32;
            let y = (self.target_pos.y + self.draw_pos_y - self.pos.y) as i32;
            let n: String = self.fish_index.to_string();
            d.draw_text(&n, x, y, 10, Color::WHITE);

            d.draw_line(
                self.pos.x as i32,
                self.draw_pos_y as i32,
                self.target_pos.x as i32,
                (self.target_pos.y + self.draw_pos_y - self.pos.y) as i32,
                Color::LIGHTGREEN,
            );
        }
        draw_fish(
            d,
            self.wobble,
            self.pos.x,
            self.draw_pos_y,
            self.direction,
            self.scale,
            self.fish_type as i32,
        )
    }

    pub fn set_target_pos(&mut self, pos: Vector2) {
        self.target_pos = pos;
        self.target_reached = false;
    }

    pub fn has_reached_target(&self) -> bool {
        self.target_reached
    }
}

impl Entity for Fish {
    fn draw<'d>(&self, d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        self.draw(d)
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn set_pos(&mut self, pos: Vector2) {
        self.pos = pos;
    }
}

pub type FishManager = EntityManager<Fish>;
