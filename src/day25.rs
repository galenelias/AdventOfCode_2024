use itertools::Itertools;

fn key_fits(key: &[usize; 5], lock: &[usize; 5]) -> bool {
	for i in 0..5 {
		if key[i] > 5 - lock[i] {
			return false;
		}
	}
	true
}

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();
	let keys_or_locks = inputs.split(|line| line.is_empty()).collect_vec();

	let locks = keys_or_locks
		.iter()
		.filter(|grid| grid.first().unwrap().iter().all(|c| *c == '#'))
		.collect_vec();
	let keys = keys_or_locks
		.iter()
		.filter(|grid| grid.last().unwrap().iter().all(|c| *c == '#'))
		.collect_vec();

	let locks: Vec<[usize; 5]> = locks
		.iter()
		.map(|grid| {
			let mut lock = [0; 5];
			for c in 0..5 {
				for r in 0..=5 {
					if grid[r + 1][c] == '.' {
						lock[c] = r;
						break;
					}
				}
			}
			lock
		})
		.collect_vec();

	let keys: Vec<[usize; 5]> = keys
		.iter()
		.map(|grid| {
			let mut key = [0; 5];
			for c in 0..5 {
				for r in 0..=5 {
					if grid[5 - r][c] == '.' {
						key[c] = r;
						break;
					}
				}
			}
			key
		})
		.collect_vec();

	let part1 = keys
		.iter()
		.cartesian_product(locks.iter())
		.filter(|(key, lock)| key_fits(key, lock))
		.count();
	println!("Part 1: {}", part1);
}
