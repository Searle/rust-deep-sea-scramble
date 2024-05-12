use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use rand::Rng;
use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

static WINDOW_WIDTH: i32 = 640;
static WINDOW_HEIGHT: i32 = 480;
static SURFACE_WIDTH: i32 = 100;

fn ease_in_out_quad(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0
    }
}

struct Surface {
    pos: Vector2,
    freq: f32,
    amplitude: f32,
}

struct SurfaceVerts {
    layer_a: Vec<Vector2>,
    layer_b: Vec<Vector2>,
    layer_c: Vec<Vector2>,
}

fn get_surface_verts(surfaces: &Vec<Surface>, x: f32) -> SurfaceVerts {
    let mut layer_a: Vec<Vector2> = vec![];
    let mut layer_b: Vec<Vector2> = vec![];
    let mut layer_c: Vec<Vector2> = vec![];
    let mut last_x0 = 0.0;
    let mut last_y_a0 = 0.0;
    let mut last_y_b0 = 0.0;
    let mut last_y_c0 = 0.0;
    for surface in surfaces {
        let x0 = x + surface.pos.x;

        let tt2 = |ofs: f32, scale: f32| {
            (((surface.pos.x + x) * (0.01 + surface.freq * 0.02) + ofs).sin() + 1.0)
                * 0.5
                * (-0.75 - 0.25 * surface.amplitude)
                * scale
        };

        let y0 = (WINDOW_HEIGHT as f32) - surface.pos.y;
        let y_a0 = y0 + tt2(0.0, 40.0);
        let y_b0 = y_a0 + tt2(f32::consts::PI * 0.4, 30.0);
        let y_c0 = y_b0 + tt2(f32::consts::PI * 0.8, 20.0);
        let bottom = WINDOW_HEIGHT as f32;

        for j in 0..8 {
            let xd = (j as f32) / 7.0;
            let ei = ease_in_out_quad(xd);
            // let ei = xd;
            let y_a1 = last_y_a0 + (y_a0 - last_y_a0) * ei;
            let y_b1 = last_y_b0 + (y_b0 - last_y_b0) * ei;
            let y_c1 = last_y_c0 + (y_c0 - last_y_c0) * ei;
            let x1 = last_x0 + (x0 - last_x0) * xd;

            layer_a.push(Vector2 { x: x1, y: y_a1 });
            layer_a.push(Vector2 { x: x1, y: bottom });

            layer_b.push(Vector2 { x: x1, y: y_b1 });
            layer_b.push(Vector2 { x: x1, y: y_a1 });

            layer_c.push(Vector2 { x: x1, y: y_c1 });
            layer_c.push(Vector2 { x: x1, y: y_b1 });
        }

        last_x0 = x0;
        last_y_a0 = y_a0;
        last_y_b0 = y_b0;
        last_y_c0 = y_c0;
    }
    SurfaceVerts {
        layer_a,
        layer_b,
        layer_c,
    }
}

fn draw_surface_verts<'a>(
    mut d: RaylibDrawHandle<'a>,
    verticeses: &SurfaceVerts,
) -> RaylibDrawHandle<'a> {
    d.draw_triangle_strip(&verticeses.layer_a, Color::DARKBLUE);
    d.draw_triangle_strip(&verticeses.layer_b, Color::MEDIUMBLUE);
    d.draw_triangle_strip(&verticeses.layer_c, Color::LIGHTBLUE);

    d
}

fn get_ship_vertices(x: f32, y: f32) -> Vec<Vector2> {
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

fn draw_ship(mut d: RaylibDrawHandle, ship_x: f32, ship_y: f32) -> RaylibDrawHandle {
    let vertices = get_ship_vertices(ship_x, ship_y);
    d.draw_triangle_strip(&vertices, Color::WHITESMOKE);
    d
}

fn draw_bullet(mut d: RaylibDrawHandle, bullet_x: f32, bullet_y: f32) -> RaylibDrawHandle {
    let x = bullet_x;
    let y = bullet_y;

    if x > 0.0 {
        let vertices = vec![
            Vector2 { x: x, y: y - 5.0 }, // Top vertex
            Vector2 { x: x - 5.0, y: y }, // Left vertex
            Vector2 { x: x + 5.0, y: y }, // Right vertex
            Vector2 { x: x, y: y + 5.0 }, // Bottom vertex
        ];

        d.draw_triangle_strip(&vertices, Color::LIGHTGREEN);
    }
    d
}

fn get_mine_vertices(x: f32, y: f32) -> Vec<Vector2> {
    vec![
        Vector2 { x: x, y: y + 2.0 }, // Bottom center of the mine base
        Vector2 {
            x: x + 6.0,
            y: y + 6.0,
        }, // Right edge of the mine base
        Vector2 {
            x: x - 6.0,
            y: y + 6.0,
        }, // Left edge of the mine base
        Vector2 {
            x: x + 6.0,
            y: y - 5.0,
        }, // Upper right body
        Vector2 {
            x: x - 6.0,
            y: y - 5.0,
        }, // Upper left body
        Vector2 { x: x, y: y - 12.0 }, // Tip of the mine
    ]
}

struct Mine {
    pos: Vector2,
    launch_x: f32,
    dy: f32,
    bubbles: Rc<RefCell<Bubbles>>,
}

fn draw_mine<'a>(mut d: RaylibDrawHandle<'a>, x: f32, y: f32) -> RaylibDrawHandle<'a> {
    let vertices = get_mine_vertices(x, y);
    d.draw_triangle_strip(&vertices, Color::DARKORANGE);
    d
}

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

struct Bubbles {
    pos: Vector2,
    num: usize,
    els: Vec<Bubble>,
    dt: f32,
    next_dt: f32,
    started: bool,
}

impl Bubbles {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos: Vector2 { x: 0.0, y: 0.0 },
            num: 0,
            els: vec![],
            dt: 0.0,
            next_dt: 0.0,
            started: false,
        }))
    }

    fn start(&mut self, pos: Vector2, num: usize) {
        self.pos = pos;
        self.num = num;
        self.els.clear();
        self.dt = 0.0;
        self.next_dt = 0.0;
        self.started = true;
    }

    fn set_pos(&mut self, pos: Vector2) {
        self.pos = pos;
    }

    fn update(&mut self, surface_verts: &SurfaceVerts, dt: f32) {
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

    fn draw<'a>(&mut self, mut d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a> {
        for el in &self.els {
            if el.size > 0.0 {
                d.draw_circle(el.pos.x as i32, el.pos.y as i32, el.size, el.color);
            }
        }
        d
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Scramble!")
        .build();

    let mut surfaces: Vec<Surface> = vec![Surface {
        pos: Vector2 { x: 0.0, y: 0.0 },
        freq: 0.0,
        amplitude: 0.0,
    }];
    let mut rng = rand::thread_rng();

    let mut arena_x = 0.0;

    let mut ship_x = 100.0;
    let mut ship_y = WINDOW_HEIGHT as f32 - 100.0;

    let mut bullet_x = 0.0;
    let mut bullet_y = 0.0;
    let mut bullet_dx = 0.0;
    let mut bullet_dy = 0.0;
    let mut bullet_vy = 0.0;

    let mut mines: Vec<Mine> = vec![];

    let mut bubbles_list: Vec<Rc<RefCell<Bubbles>>> = vec![];

    let ship_bubbles = Bubbles::new();
    bubbles_list.push(Rc::clone(&ship_bubbles));

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        arena_x = arena_x - dt * 100.0;

        if bullet_x > 0.0 {
            bullet_x = bullet_x + dt * bullet_dx;
            bullet_y = bullet_y + dt * bullet_dy;
            bullet_vy *= 0.999;
            bullet_dy *= bullet_vy;
            if bullet_x > WINDOW_WIDTH as f32 || bullet_y >= WINDOW_HEIGHT as f32 {
                bullet_x = 0.0;
            }
        }

        loop {
            let surface = &surfaces[surfaces.len() - 1];
            let surface_x = surface.pos.x;
            if arena_x + surface_x >= WINDOW_WIDTH as f32 {
                break;
            }
            let step = rng.gen_range(-1..=1);
            let mut y = surface.pos.y + step as f32 * 50.0;
            y = y.max(150.0).min(400.0);
            surfaces.push(Surface {
                pos: Vector2 {
                    x: surface_x + SURFACE_WIDTH as f32,
                    y,
                },
                freq: rng.gen_range(0.0..1.0),
                amplitude: rng.gen_range(0.0..1.0),
            });
            if step == 0 {
                let bubbles = Bubbles::new();
                mines.push(Mine {
                    pos: Vector2 {
                        x: surface_x + SURFACE_WIDTH as f32 * 0.5,
                        y,
                    },
                    launch_x: ship_x + ship_y - (WINDOW_HEIGHT as f32 - y),
                    dy: 3.0,
                    bubbles: Rc::clone(&bubbles),
                });
                bubbles_list.push(Rc::clone(&bubbles));
            }
        }

        surfaces.retain(|surface| arena_x + surface.pos.x > -SURFACE_WIDTH as f32);

        let surface_verts = get_surface_verts(&surfaces, arena_x);

        for mine in &mut mines {
            if arena_x + mine.pos.x < mine.launch_x {
                if mine.pos.y > ship_y {
                    mine.pos.y -= dt * 80.0;
                }
                mine.dy = (mine.dy * 0.995).max(0.5);
                mine.pos.y += dt * 100.0 * mine.dy;
                let mut bubbles_ref = mine.bubbles.borrow_mut();
                if bubbles_ref.started {
                    bubbles_ref.set_pos(Vector2 {
                        x: arena_x + mine.pos.x,
                        y: mine.pos.y,
                    });
                } else {
                    bubbles_ref.start(
                        Vector2 {
                            x: arena_x + mine.pos.x,
                            y: mine.pos.y,
                        },
                        5,
                    );
                }
            } else {
                let x = arena_x + mine.pos.x;
                let index = get_surface_verts_index(&surface_verts, x);
                let y = surface_verts.layer_a[index].y;
                mine.pos.y = y;
            }
        }

        mines.retain(|mine| {
            arena_x + mine.pos.x >= -20.0 && mine.pos.y < WINDOW_HEIGHT as f32 + 20.0
        });

        let ship_index = get_surface_verts_index(&surface_verts, ship_x);
        let ship_y_ofs = surface_verts.layer_c[ship_index].y - surface_verts.layer_b[ship_index].y;

        let ship_y_min = surface_verts.layer_a[ship_index].y + 30.0;
        let ship_y_max = WINDOW_HEIGHT as f32 - 30.0;
        if ship_y < ship_y_min {
            let diff = ship_y_min - ship_y;
            ship_y += diff.min(2.0);
        }
        if ship_y > ship_y_max {
            let diff = ship_y - ship_y_max;
            ship_y -= diff.min(2.0);
        }

        {
            let mut ship_bubbles_ref = ship_bubbles.borrow_mut();
            ship_bubbles_ref.set_pos(Vector2 {
                x: ship_x + 10.0,
                y: ship_y,
            });
        }

        for bubbles in &bubbles_list {
            bubbles.borrow_mut().update(&surface_verts, dt);
        }

        // Keyboard

        if rl.is_key_down(KEY_UP) {
            ship_y -= 1.0;
        }
        if rl.is_key_down(KEY_DOWN) {
            ship_y += 1.0;
        }
        if rl.is_key_down(KEY_SPACE) && bullet_x == 0.0 {
            bullet_x = ship_x + 15.0;
            bullet_y = ship_y + 10.0;
            bullet_dx = 300.0;
            bullet_dy = 10.0;
            bullet_vy = 1.03;

            let mut ship_bubbles_ref = ship_bubbles.borrow_mut();
            ship_bubbles_ref.start(
                Vector2 {
                    x: ship_x + 10.0,
                    y: ship_y,
                },
                60,
            );
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        let mut color = Color::BLACK;

        /*
        for vertices in &verticeses {
            let collision = check_collision_point_poly(
                Vector2 {
                    x: bullet_x,
                    y: bullet_y,
                },
                &vertices,
            );
            if collision {
                color = Color::RED;
            }
        }
        */

        d.clear_background(Color::LIGHTSKYBLUE);
        d.draw_text("Scramble!", 12, 12, 20, color);

        d = draw_surface_verts(d, &surface_verts);

        for bubbles in &bubbles_list {
            d = bubbles.borrow_mut().draw(d);
        }

        for mine in &mines {
            d = draw_mine(d, arena_x + mine.pos.x, mine.pos.y);
        }

        let d = draw_ship(d, ship_x, ship_y + ship_y_ofs);
        let d = draw_bullet(d, bullet_x, bullet_y);
    }
}

fn get_surface_verts_index(surface_verts: &SurfaceVerts, x: f32) -> usize {
    let mut index = match surface_verts
        .layer_a
        .binary_search_by(|v| v.x.total_cmp(&x))
    {
        Ok(index) => index,
        Err(index) => index,
    };
    if index >= surface_verts.layer_a.len() {
        return surface_verts.layer_a.len() - 1;
    }
    while index > 0 {
        if surface_verts.layer_a[index].x != surface_verts.layer_a[index - 1].x {
            break;
        }
        index -= 1;
    }
    index
}
