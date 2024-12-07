use std::time::Instant;

pub fn main() {
	let start_time = Instant::now();

	let time = Instant::now() - start_time;
	println!("Answer is potato, calculated in {time:.0?}.");
}
