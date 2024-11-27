use criterion::{criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, ImageFormat, Rgb};
use std::io::Cursor;

pub fn bench_jpeg_fixed_point(c: &mut Criterion) {
    let width = 1920;
    let height = 1920;
    let src = ImageBuffer::from_pixel(width, height, Rgb([213u8, 156, 64]));

    c.bench_function("Test JPEG floating point", |b| {
        let mut y_plane = vec![0u8; width as usize * height as usize];
        let mut u_plane = vec![0u8; width as usize * height as usize];
        let mut v_plane = vec![0u8; width as usize * height as usize];
        b.iter(|| {
            for (((rgb, y_dst), u_dst), v_dst) in src
                .chunks_exact(3)
                .zip(y_plane.iter_mut())
                .zip(u_plane.iter_mut())
                .zip(v_plane.iter_mut())
            {
                let r: f32 = rgb[0] as f32;
                let g: f32 = rgb[1] as f32;
                let b: f32 = rgb[2] as f32;

                // Coefficients from JPEG File Interchange Format (Version 1.02), multiplied for 255 maximum.
                let y = 0.299 * r + 0.587 * g + 0.114 * b;
                let cb = -0.1687 * r - 0.3313 * g + 0.5 * b + 128.;
                let cr = 0.5 * r - 0.4187 * g - 0.0813 * b + 128.;
                *y_dst = y as u8;
                *u_dst = cb as u8;
                *v_dst = cr as u8;
            }
        });
    });

    c.bench_function("Test JPEG fixed point", |b| {
        let mut y_plane = vec![0u8; width as usize * height as usize];
        let mut u_plane = vec![0u8; width as usize * height as usize];
        let mut v_plane = vec![0u8; width as usize * height as usize];
        b.iter(|| {
            for (((rgb, y_dst), u_dst), v_dst) in src
                .chunks_exact(3)
                .zip(y_plane.iter_mut())
                .zip(u_plane.iter_mut())
                .zip(v_plane.iter_mut())
            {
                let r = rgb[0] as i32;
                let g = rgb[1] as i32;
                let b = rgb[2] as i32;
                const Y_BIAS: i32 = 1 << 15; // + 0.5 is needed to simulate rounding shift right in-place
                const UV_BIAS: i32 = (128 << 16) + (1 << 15); // + 0.5 is needed to simulate rounding shift right in-place
                let y = (19595 * r + 38470 * g + 7471 * b + Y_BIAS) >> 16;
                let cb = (-11059 * r - 21709 * g + 32768 * b + UV_BIAS) >> 16;
                let cr = (32768 * r - 27439 * g - 5329 * b + UV_BIAS) >> 16;
                *y_dst = y as u8;
                *u_dst = cb as u8;
                *v_dst = cr as u8;
            }
        });
    });

    c.bench_function("Test JPEG Encoding", |b| {
        b.iter(|| {
            let mut bytes: Vec<u8> = Vec::new();
            src.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Jpeg)
                .unwrap();
        });
    });
}

criterion_group!(benches, bench_jpeg_fixed_point);
criterion_main!(benches);
