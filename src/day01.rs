use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(inputs: Vec<String>) {
	let parts = inputs
		.iter()
		.map(|x| {
			x.split_whitespace()
				.map(|x| x.parse::<i32>().unwrap())
				.collect_vec()
		})
		.collect_vec();

	let list1 = parts.iter().map(|x| x[0]).sorted().collect_vec();
	let list2 = parts.iter().map(|x| x[1]).sorted().collect_vec();

	let part1 = list1
		.iter()
		.zip(&list2)
		.map(|(a, b)| (a - b).abs())
		.sum::<i32>();
	println!("Part 1: {part1}");

	let list2_freqs = list2.iter().fold(HashMap::new(), |mut acc, x| {
		*acc.entry(x).or_insert(0) += 1;
		acc
	});

	let part2 = list1
		.iter()
		.map(|x| *x * list2_freqs.get(x).unwrap_or(&0))
		.sum::<i32>();
	println!("Part 2: {part2}");
}
