use std::collections::HashMap;

use num::Integer;

pub fn solve(inputs: Vec<String>) {
	let mut stones = inputs[0]
		.split_whitespace()
		.map(|s| s.parse::<u64>().unwrap())
		.fold(HashMap::new(), |mut acc, x| {
			*acc.entry(x).or_insert(0) += 1;
			acc
		});

	for i in 0..75 {
		let mut new_stones = HashMap::with_capacity(stones.len());
		for (stone, freq) in stones {
			if stone == 0 {
				*new_stones.entry(1).or_insert(0) += freq;
			} else {
				let num_digits = stone.checked_ilog10().unwrap() + 1;
				if num_digits.is_even() {
					// split number into first half of digits and second half of digits
					let first_half = stone / 10u64.pow(num_digits / 2);
					let second_half = stone % 10u64.pow(num_digits / 2);
					*new_stones.entry(first_half).or_insert(0) += freq;
					*new_stones.entry(second_half).or_insert(0) += freq;
				} else {
					*new_stones.entry(stone * 2024).or_insert(0) += freq;
				}
			}
		}
		stones = new_stones;
		if i == 24 {
			println!("Part1: {}", stones.values().sum::<u64>());
		}
	}

	println!("Part2: {}", stones.values().sum::<u64>());
}
