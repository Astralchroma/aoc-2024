use std::{
	io::{stdin, Read},
	time::Instant,
};
use LookingFor::*;

pub fn main() {
	let start_time = Instant::now();

	let mut mul_enabled = true;
	let mut looking_for = Tag;
	let mut buffer = String::new();
	let mut total: u32 = 0;

	for byte in stdin().bytes() {
		// AoC seems to provide all input in ASCII, so no special logic is needed here
		let char = unsafe { char::from_u32_unchecked(byte.unwrap() as u32) };

		match looking_for {
			Tag => {
				buffer.push(char);

				if buffer.len() > 7 {
					buffer.remove(0);
				}

				if mul_enabled && buffer.ends_with("mul(") {
					looking_for = Val1;
					buffer.clear();
				} else if buffer.ends_with("do()") {
					mul_enabled = true;
					buffer.clear();
				} else if buffer.ends_with("don't()") {
					mul_enabled = false;
					buffer.clear();
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
					looking_for = Tag;
					buffer.clear();
					buffer.push(other);
				}
			},
			Val2(val1) => match char {
				'0'..='9' => {
					buffer.push(char);
				}
				')' => {
					looking_for = Tag;

					let val2: u32 = buffer.parse().expect("buffer should only contain digits");
					total += val1 as u32 * val2;

					buffer.clear();
				}
				other => {
					looking_for = Tag;
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
	Tag,
	Val1,
	Val2(u16),
}
