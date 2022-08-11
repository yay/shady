use minifb::{Key, Window, WindowOptions};

pub fn run() {
    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;
    let mut buffer: Vec<u32> = std::vec::from_elem(0, WIDTH * HEIGHT);

    let mut window = Window::new(
        "Software Fragment Shader Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let now = std::time::SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = now.elapsed().unwrap().as_micros() as f64 / 1_000_000.0;
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let u = [
                (x as f64 / WIDTH as f64) * 2.0 - 1.0,
                (y as f64 / WIDTH as f64) * 2.0 - (HEIGHT as f64 / WIDTH as f64),
            ];
            let t = time as f64 * 0.2;
            let tt = (t / 8.0).sin() * 64.0;
            let mut x = u[0] * tt + (t * 2.1).sin() * 4.0;
            let mut y = u[1] * tt + (t * 2.3).cos() * 4.0;
            let mut c = x.sin() + y.sin();
            let zoom = t.sin();
            x = x * zoom * 2.0 + (t * 1.1).sin();
            y = y * zoom * 2.0 + (t * 1.3).cos();
            let xx = (t * 0.7).cos() * x - (t * 0.7).sin() * y;
            let yy = (t * 0.7).sin() * x + (t * 0.7).cos() * y;
            c = ((c + (xx.sin() + yy.sin())).sin() + 1.0) * 0.4;
            let v = 2.0 - (u[0] * u[0] + u[1] * u[1]).sqrt() * 2.0;

            let r = (v * (c + v * 0.4) * 255.0) as u8;
            let g = ((v * (c * c - 0.5 + v * 0.5)) * 255.0) as u8;
            let b = (v * (c * 1.9) * 255.0) as u8;

            *pixel = (r as u32) | (g as u32) << 8 | (b as u32) << 16;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}