use crate::models::{Bounds, FractalPoint};
use rand::Rng;

pub const CANVAS_WIDTH: usize = 800;
pub const CANVAS_HEIGHT: usize = 600;

fn encode_intensity(iter: i32, max_iterations: i32) -> i32 {
    if iter == max_iterations { 0 } else { iter * 255 / max_iterations }
}

pub fn generate_mandelbrot(bounds: Bounds, max_iterations: i32) -> Vec<FractalPoint> {
    let mut points = Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT);
    let x_range = bounds.x_max - bounds.x_min;
    let y_range = bounds.y_max - bounds.y_min;

    for screen_y in 0..CANVAS_HEIGHT {
        for screen_x in 0..CANVAS_WIDTH {
            let c_re = bounds.x_min + (screen_x as f64 * x_range / CANVAS_WIDTH as f64);
            let c_im = bounds.y_min + (screen_y as f64 * y_range / CANVAS_HEIGHT as f64);

            let (mut z_re, mut z_im, mut iter) = (0.0, 0.0, 0);
            while z_re * z_re + z_im * z_im <= 4.0 && iter < max_iterations {
                let next_re = z_re * z_re - z_im * z_im + c_re;
                let next_im = 2.0 * z_re * z_im + c_im;
                z_re = next_re;
                z_im = next_im;
                iter += 1;
            }

            points.push(FractalPoint {
                x: screen_x as f64,
                y: screen_y as f64,
                intensity: encode_intensity(iter, max_iterations),
            });
        }
    }
    points
}

pub fn generate_julia(bounds: Bounds, max_iterations: i32) -> Vec<FractalPoint> {
    let mut points = Vec::with_capacity(CANVAS_WIDTH * CANVAS_HEIGHT);
    let x_range = bounds.x_max - bounds.x_min;
    let y_range = bounds.y_max - bounds.y_min;
    let (c_re, c_im) = (-0.400, 0.600);

    for screen_y in 0..CANVAS_HEIGHT {
        for screen_x in 0..CANVAS_WIDTH {
            let mut z_re = bounds.x_min + (screen_x as f64 * x_range / CANVAS_WIDTH as f64);
            let mut z_im = bounds.y_min + (screen_y as f64 * y_range / CANVAS_HEIGHT as f64);
            let mut iter = 0;

            while z_re * z_re + z_im * z_im <= 4.0 && iter < max_iterations {
                let next_re = z_re * z_re - z_im * z_im + c_re;
                let next_im = 2.0 * z_re * z_im + c_im;
                z_re = next_re;
                z_im = next_im;
                iter += 1;
            }

            points.push(FractalPoint {
                x: screen_x as f64,
                y: screen_y as f64,
                intensity: encode_intensity(iter, max_iterations),
            });
        }
    }
    points
}

pub fn generate_leaf() -> Vec<FractalPoint> {
    let mut points = Vec::new();
    let mut pixel_grid = vec![vec![0i32; CANVAS_HEIGHT]; CANVAS_WIDTH];
    let (mut x, mut y) = (0.0, 0.0);
    let mut rng = rand::thread_rng();

    for _ in 0..150_000 {
        let r: u8 = rng.gen_range(0..100);
        let (next_x, next_y) = match r {
            0 => (0.0, 0.16 * y),
            1..=85 => (0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6),
            86..=92 => (0.20 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6),
            _ => (-0.15 * x + 0.28 * y, 0.26 * x + 0.24 * y + 0.44),
        };
        x = next_x;
        y = next_y;

        let screen_x = ((x + 2.182) * (CANVAS_WIDTH - 1) as f64 / (2.655 + 2.182)).round() as usize;
        let screen_y = ((9.96 - y) * (CANVAS_HEIGHT - 1) as f64 / 9.96).round() as usize;

        if screen_x < CANVAS_WIDTH && screen_y < CANVAS_HEIGHT {
            pixel_grid[screen_x][screen_y] = 200;
        }
    }

    for px in 0..CANVAS_WIDTH {
        for py in 0..CANVAS_HEIGHT {
            if pixel_grid[px][py] > 0 {
                points.push(FractalPoint { x: px as f64, y: py as f64, intensity: pixel_grid[px][py] });
            }
        }
    }
    points
}