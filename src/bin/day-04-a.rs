use std::{io::stdin, time::Instant};

pub fn main() {
	let start_time = Instant::now();

	let mut word_search: Vec<Vec<u8>> = vec![];

	for line in stdin().lines() {
		let line = line.unwrap();

		word_search.push(
			line.chars()
				.map(|char| match char {
					'X' => 0,
					'M' => 1,
					'A' => 2,
					'S' => 3,
					_ => unreachable!("input should only contain X, M, A, and S"),
				})
				.collect(),
		);
	}

	let mut found = 0;

	for x in 0..word_search.len() as isize {
		for y in 0..word_search.len() as isize {
			let index = word_search[x as usize][y as usize];

			if index != 0 {
				continue;
			}

			for vector_x in -1..=1 {
				'vector: for vector_y in -1..=1 {
					if vector_x == 0 && vector_y == 0 {
						continue;
					}

					let mut other_x = x;
					let mut other_y = y;

					for target_index in 1..4 {
						other_x += vector_x;
						other_y += vector_y;

						if !(0..140).contains(&other_x) || !(0..140).contains(&other_y) {
							continue 'vector;
						}

						let other_index = word_search[other_x as usize][other_y as usize];

						if other_index != target_index {
							continue 'vector;
						}
					}

					found += 1;
				}
			}
		}
	}

	let time = Instant::now() - start_time;
	println!("Answer is {found}, calculated in {time:.0?}.");
}
