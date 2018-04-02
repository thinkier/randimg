extern crate image;
extern crate rand;

use rand::{OsRng, Rng};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
	let mut buffer = vec![0u8; WIDTH * HEIGHT * 3];

	fill_random(&mut buffer);

	image::save_buffer("out.png", buffer.as_slice(), WIDTH as u32, HEIGHT as u32, image::RGB(8)).expect("failed to save image buffer");
}

fn fill_random(slice: &mut Vec<u8>) {
	OsRng::new().ok().unwrap().fill_bytes(slice.as_mut_slice())
}