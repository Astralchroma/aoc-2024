use std::{io::stdin, time::Instant};

pub fn main() {
	let start_time = Instant::now();

	let mut word_search: Vec<Vec<i8>> = vec![];

	for line in stdin().lines() {
		let line = line.unwrap();

		word_search.push(
			line.chars()
				.map(|char| match char {
					'M' => -1,
					'A' => 0,
					'S' => 1,
					_ => i8::MAX,
				})
				.collect(),
		);
	}

	let mut found = 0;

	for x in 1..word_search.len() as isize - 1 {
		'cell: for y in 1..word_search.len() as isize - 1 {
			let index = word_search[x as usize][y as usize];

			if index != 0 {
				continue;
			}

			for vector_x in [-1, 1] {
				let other_x = x + vector_x;
				let other_y = y + -1;

				let other_index = word_search[other_x as usize][other_y as usize];

				if other_index.abs() != 1 {
					continue 'cell;
				}

				let opposite_other_x = x + -vector_x;
				let opposite_other_y = y + 1;

				if word_search[opposite_other_x as usize][opposite_other_y as usize] != -other_index
				{
					continue 'cell;
				}
			}

			found += 1;
		}
	}

	let time = Instant::now() - start_time;
	println!("Answer is {found}, calculated in {time:.0?}.");
}
