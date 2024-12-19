use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut total_seen = HashSet::new();
	let mut part1 = 0;
	let mut part2 = 0;

	for r in 0..grid.len() {
		for c in 0..grid[0].len() {
			if total_seen.contains(&(r as isize, c as isize)) {
				continue;
			}

			let mut seen = HashSet::new();
			let mut queue = VecDeque::new();
			let ch = grid[r][c];

			queue.push_back((r as isize, c as isize));

			while !queue.is_empty() {
				let (r, c) = queue.pop_front().unwrap();

				if r < 0 || r >= grid.len() as isize || c < 0 || c >= grid[0].len() as isize {
					continue;
				}

				if grid[r as usize][c as usize] != ch {
					continue;
				}

				if !seen.insert((r, c)) {
					continue;
				}

				queue.push_back((r - 1, c));
				queue.push_back((r + 1, c));
				queue.push_back((r, c - 1));
				queue.push_back((r, c + 1));
			}

			let mut total_fence = 0;
			for (r, c) in seen.iter().copied() {
				if !seen.contains(&(r - 1, c)) {
					total_fence += 1; // Top fence
				}

				if !seen.contains(&(r + 1, c)) {
					total_fence += 1; // Bottom fence
				}

				if !seen.contains(&(r, c - 1)) {
					total_fence += 1; // Left fence
				}

				if !seen.contains(&(r, c + 1)) {
					total_fence += 1; // Right fence
				}
			}

			// Count unique fence by figuring out if we're not a continuation of a bence above/left
			let mut unique_fence = 0;
			for (r, c) in seen.iter().copied() {
				if !seen.contains(&(r - 1, c))
					&& (!seen.contains(&(r, c - 1)) || seen.contains(&(r - 1, c - 1)))
				{
					unique_fence += 1; // Top fence
				}

				if !seen.contains(&(r + 1, c))
					&& !(seen.contains(&(r, c - 1)) && !seen.contains(&(r + 1, c - 1)))
				{
					unique_fence += 1; // Bottom fence
				}

				if !seen.contains(&(r, c - 1))
					&& !(seen.contains(&(r - 1, c)) && !seen.contains(&(r - 1, c - 1)))
				{
					unique_fence += 1; // Left fence
				}

				if !seen.contains(&(r, c + 1))
					&& !(seen.contains(&(r - 1, c)) && !seen.contains(&(r - 1, c + 1)))
				{
					unique_fence += 1; // Right fence
				}
			}

			part1 += total_fence * seen.len();
			part2 += unique_fence * seen.len();
			total_seen.extend(seen);
		}
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
