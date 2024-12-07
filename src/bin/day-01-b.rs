#![feature(iterator_try_collect)]

use std::{collections::HashMap, io::stdin, time::Instant};

pub fn main() {
	let start_time = Instant::now();

	let mut left_list = vec![];
	let mut right_list = HashMap::new();

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
		*right_list.entry(numbers[1]).or_default() += 1;
	}

	let total_similarity = left_list
		.into_iter()
		.map(|left| left * right_list.get(&left).unwrap_or(&0))
		.fold(0, |total, similarity| total + similarity);

	let time = Instant::now() - start_time;
	println!("Answer is {total_similarity}, calculated in {time:.0?}.");
}
