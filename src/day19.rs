use itertools::Itertools;
use std::collections::HashMap;

fn can_make<'a>(
	pattern: &'a [char],
	towels: &[Vec<char>],
	memo: &mut HashMap<&'a [char], usize>,
) -> usize {
	if pattern.is_empty() {
		return 1;
	}

	if let Some(&result) = memo.get(pattern) {
		return result;
	}

	let result = towels
		.iter()
		.filter(|towel| pattern.starts_with(towel))
		.map(|towel| can_make(&pattern[towel.len()..], towels, memo))
		.sum();

	memo.insert(pattern, result);
	result
}

pub fn solve(inputs: Vec<String>) {
	let towels = inputs[0]
		.split(", ")
		.map(|x| x.chars().collect_vec())
		.collect_vec();

	let mut part1 = 0;
	let mut part2 = 0;
	for pattern in inputs[2..].iter().map(|x| x.chars().collect_vec()) {
		let arrangements = can_make(&pattern, &towels, &mut HashMap::new());
		if arrangements > 0 {
			part1 += 1;
		}
		part2 += arrangements;
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
