#![feature(iterator_try_collect)]

use std::{io::stdin, time::Instant};

pub fn main() {
	let start_time = Instant::now();

	let mut safe = 0;

	'reports: for line in stdin().lines() {
		let original_levels = line
			.unwrap()
			.split_whitespace()
			.map(str::parse::<u8>)
			.try_collect::<Vec<_>>()
			.expect("columns should be valid integers");

		let mut levels = original_levels.clone();
		'dampener: for remove_index in 0..=original_levels.len() {
			let report_increasing = levels[0] < levels[1];

			for index in 0..levels.len() - 1 {
				let current = levels[index];
				let next = levels[index + 1];

				let increasing = current < next;
				let consistent_with_report = increasing == report_increasing;

				if !consistent_with_report {
					if remove_index != original_levels.len() {
						levels = original_levels.clone();
						levels.remove(remove_index);
					}

					continue 'dampener;
				}

				let level_diff = u8::abs_diff(current, next);
				let diff_in_range = (1..=3).contains(&level_diff);

				if !diff_in_range {
					if remove_index != original_levels.len() {
						levels = original_levels.clone();
						levels.remove(remove_index);
					}

					continue 'dampener;
				}
			}

			safe += 1;
			continue 'reports;
		}
	}

	let time = Instant::now() - start_time;
	println!("Answer is {safe}, calculated in {time:.0?}.");
}
