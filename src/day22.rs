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
	let mut part2 = 0;

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

	let mut delta_maps : Vec<HashMap<&[i64], i64>> = vec![HashMap::new(); inputs.len()];
	for (i, deltas) in deltas.iter().enumerate() {
		for di in 0..deltas.len() - 4 {
			delta_maps[i].entry(&deltas[di..di + 4]).or_insert(prices[i][di + 4]);
		}
	}

	for d1 in -10..=10 {
		for d2 in -10..=10 {
			for d3 in -10..=10 {
				for d4 in -10..=10 {
					let pattern: [i64; 4] = [d1, d2, d3, d4];
					let mut sum = 0;
					for i in 0..prices.len() {
						if let Some(price) = delta_maps[i].get(&pattern[..]) {
							sum += price;
						}
					}
					if sum > part2 {
						part2 = sum;
					}
				}
			}
		}
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
