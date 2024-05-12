use std::f32;

use raylib::prelude::*;

pub struct Surface {
    pub pos: Vector2,
    pub freq: f32,
    pub amplitude: f32,
}

impl Surface {
    pub fn new() -> Self {
        Self {
            pos: Vector2 { x: 0.0, y: 0.0 },
            freq: 0.0,
            amplitude: 0.0,
        }
    }
}
