use num_traits::Float;
use std::fs::File;
use std::io::Write;
use std::cmp::Ordering;
use byteorder::ByteOrder;
use byteorder::NativeEndian;
use std::mem;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;


fn max<T: Float>(a: T, b: T) -> T {
    let result = a.partial_cmp(&b);
    match result {
        Some(Ordering::Less) => b,
        Some(Ordering::Greater) => a,
        Some(Ordering::Equal) => a,
        None => panic!("Could not compare floats"),
    }
}

fn min<T: Float>(a: T, b: T) -> T {
    let result = a.partial_cmp(&b);
    match result {
        Some(Ordering::Less) => a,
        Some(Ordering::Greater) => b,
        Some(Ordering::Equal) => a,
        None => panic!("Could not compare floats"),
    }
}

#[derive(Copy, Clone)]
struct Vec3<T: Float> {
    x: T,
    y: T,
    z: T,
}

fn render() {
    let mut framebuffer: Vec<Vec3<f32>> = Vec::new();
    framebuffer.resize(WIDTH * HEIGHT, Vec3 { x: 0.0, y: 0.0, z: 0.0 });

    // Create simple image
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            framebuffer[(i + j * WIDTH)] = Vec3 { x: j as f32 / HEIGHT as f32, y: i as f32 / WIDTH as f32, z: 0.0 };
        }
    }

    // Write to disk
    let mut image = File::create("./out.ppm").expect("Could not create file");
    let header = format!("P6\n{} {}\n255\n", WIDTH, HEIGHT);
    image.write(header.as_bytes()).expect("Could not write header");

    for i in 0..HEIGHT * WIDTH {
        [framebuffer[i].x, framebuffer[i].y, framebuffer[i].z].iter().map(|value| {
            let value = (255.0 * max(0.0, min(1.0, *value))) as u8;
            u8::to_ne_bytes(value)
        }).for_each(|array| {
            image.write(&array).expect("Could not writer byte array to image");
        });
    }
}

fn main() {
    render();
}
