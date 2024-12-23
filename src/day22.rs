use std::collections::HashMap;

fn next_secret(mut current: i64) -> i64 {
	current ^= current * 64;
	current %= 16777216;
	current ^= current / 32;
	current %= 16777216;
	current ^= current * 2048;
	current % 16777216
}

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;

	let mut prices = vec![Vec::with_capacity(2000); inputs.len()];
	let mut deltas = vec![Vec::with_capacity(2000); inputs.len()];
	for (i, input) in inputs.iter().enumerate() {
		let mut num = input.parse::<i64>().unwrap();

		prices[i].push(num % 10);

		for _ in 0..2000 {
			num = next_secret(num);
			prices[i].push(num % 10);
			deltas[i].push(prices[i][prices[i].len() - 1] - prices[i][prices[i].len() - 2]);
		}

		part1 += num;
	}

	let mut delta_maps: Vec<HashMap<&[i64], i64>> = vec![HashMap::new(); inputs.len()];
	for (i, deltas) in deltas.iter().enumerate() {
		for di in 0..deltas.len() - 4 {
			delta_maps[i]
				.entry(&deltas[di..di + 4])
				.or_insert(prices[i][di + 4]);
		}
	}

	let mut bananas_per_delta: HashMap<&[i64], i64> = HashMap::new();
	for delta_map in &delta_maps {
		for (delta, price) in delta_map.iter() {
			*bananas_per_delta.entry(delta).or_insert(0) += price;
		}
	}

	let part2 = bananas_per_delta.values().max().unwrap();

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
