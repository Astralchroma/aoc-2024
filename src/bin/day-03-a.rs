use std::{
	io::{stdin, Read},
	time::Instant,
};
use LookingFor::*;

pub fn main() {
	let start_time = Instant::now();

	let mut looking_for = Mul;
	let mut buffer = String::new();
	let mut total: u32 = 0;

	for byte in stdin().bytes() {
		// AoC seems to provide all input in ASCII, so no special logic is needed here
		let char = unsafe { char::from_u32_unchecked(byte.unwrap() as u32) };

		match looking_for {
			Mul => {
				buffer.push(char);

				if buffer == "mul(" {
					looking_for = Val1;
					buffer.clear();
					continue;
				}

				if buffer.len() > 3 {
					buffer.remove(0);
				}
			}
			Val1 => match char {
				'0'..='9' => {
					buffer.push(char);
				}
				',' => {
					looking_for = Val2(buffer.parse().expect("buffer should only contain digits"));
					buffer.clear();
				}
				other => {
					looking_for = Mul;
					buffer.clear();
					buffer.push(other);
				}
			},
			Val2(val1) => match char {
				'0'..='9' => {
					buffer.push(char);
				}
				')' => {
					looking_for = Mul;

					let val2: u32 = buffer.parse().expect("buffer should only contain digits");
					total += val1 as u32 * val2;

					buffer.clear();
				}
				other => {
					looking_for = Mul;
					buffer.clear();
					buffer.push(other);
				}
			},
		}
	}

	let time = Instant::now() - start_time;
	println!("Answer is {total}, calculated in {time:.0?}.");
}

enum LookingFor {
	Mul,
	Val1,
	Val2(u16),
}
