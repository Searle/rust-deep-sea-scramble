use std::f32;

use raylib::prelude::*;

use crate::consts::*;
use crate::surface::*;

pub struct SurfaceVerts {
    pub layer_a: Vec<Vector2>,
    pub layer_b: Vec<Vector2>,
    pub layer_c: Vec<Vector2>,
}

fn ease_in_out_quad(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0
    }
}

pub fn get_surface_verts(surfaces: &Vec<Surface>, x: f32) -> SurfaceVerts {
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

pub fn draw_surface_verts<'a>(
    mut d: RaylibDrawHandle<'a>,
    verticeses: &SurfaceVerts,
) -> RaylibDrawHandle<'a> {
    d.draw_triangle_strip(&verticeses.layer_a, Color::DARKBLUE);
    d.draw_triangle_strip(&verticeses.layer_b, Color::MEDIUMBLUE);
    d.draw_triangle_strip(&verticeses.layer_c, Color::LIGHTBLUE);

    d
}

pub fn get_surface_verts_index(surface_verts: &SurfaceVerts, x: f32) -> usize {
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
