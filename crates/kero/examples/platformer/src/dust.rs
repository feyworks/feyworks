use std::f32::consts::PI;

use kero::prelude::*;

use crate::NATIVE_RES;

const DUST_COLOR: Rgba8 = hex(0x6553ad);

// Serviceable, cheap 1d noise function, 0.0 to 1.0. pulled out of cobalt core but i'm sure i found it online somewhere...
fn rand(n: f32) -> f32 {
    (n * 12.9898).sin() * 43758.5453 % 1.0
}

// function that draws dust particles without having any state. they're deterministic and use a noise function
pub fn draw_dust(ctx: &Context, draw: &mut Draw) {
    let voronoi_grid = 12.0;
    let t = ctx.time.frame() as f32 * 0.005;

    let cam_pos = vec2(NATIVE_RES.x as f32, NATIVE_RES.y as f32) / 2.0;
    let view_size = vec2(NATIVE_RES.x as f32, NATIVE_RES.y as f32);

    let min_x = (cam_pos.x - view_size.x / 2.0).floor();
    let max_x = (cam_pos.x + view_size.x / 2.0).ceil();
    let min_y = (cam_pos.y - view_size.y / 2.0).floor();
    let max_y = (cam_pos.y + view_size.y / 2.0).ceil();

    let start_x = (min_x / voronoi_grid).floor() * voronoi_grid;
    let start_y = (min_y / voronoi_grid).floor() * voronoi_grid;

    let c1 = DUST_COLOR
        .to_f32()
        .mul_color(Rgba::new(1.0, 1.0, 1.0, 0.3))
        .to_u8()
        .premultiply();
    let c2 = DUST_COLOR
        .to_f32()
        .mul_color(Rgba::new(1.0, 1.0, 1.0, 0.7))
        .to_u8()
        .premultiply();

    let mut x = start_x;
    while x <= max_x {
        let mut y = start_y;
        while y <= max_y {
            let pseed = rand(x * 0.0117 + y * 0.0427);
            let always_on = rand(x * 0.027 + y * 0.017) < 0.5;

            let mut xx = x + voronoi_grid * rand(x + 0.31 + y * 0.21);
            let mut yy = y + voronoi_grid * rand(x + 0.32 + y * 0.54);

            let particle_speed = -0.2 + 0.4 * rand(x * 0.117 + y * 0.047);
            xx += 5.0 * (particle_speed * (t + pseed) * PI * 2.0).cos();
            yy += 5.0 * (particle_speed * (t + pseed) * PI * 2.0).sin();

            let c = if always_on { c1 } else { c2 };

            xx = xx.round();
            yy = yy.round();

            draw.rect(RectF::new(xx, yy, 1.0, 1.0), c);
            y += voronoi_grid;
        }
        x += voronoi_grid;
    }
}
