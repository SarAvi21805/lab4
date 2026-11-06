use raylib::prelude::*;

fn point(d: &mut RaylibDrawHandle<'_>, x: i32, y: i32, color: Color) {
    d.draw_pixel(x, y, color);
}

pub fn line(
    d: &mut RaylibDrawHandle<'_>,
    start: Vector2,
    end: Vector2,
    color: Color,
) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        point(d, x0, y0, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let error2 = 2 * error;

        if error2 >= dy {
            error += dy;
            x0 += sx;
        }

        if error2 <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

pub fn triangle(
    d: &mut RaylibDrawHandle<'_>,
    a: Vector2,
    b: Vector2,
    c: Vector2,
    color: Color,
) {
    line(d, a, b, color);
    line(d, b, c, color);
    line(d, c, a, color);
}