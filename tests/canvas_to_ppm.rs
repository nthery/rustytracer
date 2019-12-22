use rustytracer::canvas::Canvas;
use rustytracer::color::Color;
use std::fs::File;

#[test]
fn canvas_to_ppm() {
    let mut c = Canvas::new(100, 256);
    for y in 0..256 {
        for x in 0..100 {
            c.set(
                x,
                y,
                &Color::new(y as f64 / 256.0, y as f64 / 256.0, y as f64 / 256.0),
            );
        }
    }

    let file = File::create("/tmp/rainbow.ppm").unwrap();
    c.to_ppm(file).unwrap();
}
