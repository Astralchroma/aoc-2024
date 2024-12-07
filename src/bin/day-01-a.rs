#![feature(iterator_try_collect)]

use std::{io::stdin, iter, time::Instant};

pub fn main() {
	let start_time = Instant::now();

	let mut left_list = vec![];
	let mut right_list = vec![];

	for (index, line) in stdin().lines().enumerate() {
		let numbers = line
			.unwrap()
			.split_whitespace()
			.map(str::parse::<u32>)
			.try_collect::<Vec<_>>()
			.expect("columns should be valid integers");

		if numbers.len() > 2 {
			println!("More columns then expected on line {index}, input may be malformed!");
		}

		left_list.push(numbers[0]);
		right_list.push(numbers[1]);
	}

	left_list.sort();
	right_list.sort();

	let total_distance = iter::zip(left_list.into_iter(), right_list.into_iter())
		.map(|(left, right)| u32::abs_diff(left, right))
		.fold(0, |total, distance| total + distance);

	let time = Instant::now() - start_time;
	println!("Answer is {total_distance}, calculated in {time:.0?}.");
}
