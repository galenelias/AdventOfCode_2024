use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// TODO: How to take range as an actual std::ops::Range?
fn sub_solve(
	antennas: &HashMap<char, Vec<(isize, isize)>>,
	grid_height: isize,
	grid_width: isize,
	dist_mult_range_min: isize,
	dist_mult_range_max: isize,
) -> usize {
	let mut antinodes = HashSet::new();
	for (_ant, positions) in antennas.iter() {
		for (a1, a2) in positions.iter().tuple_combinations() {
			for i in dist_mult_range_min..=dist_mult_range_max {
				let anti1 = (a1.0 + i * (a1.0 - a2.0), a1.1 + i * (a1.1 - a2.1));
				let anti2 = (a2.0 + i * (a2.0 - a1.0), a2.1 + i * (a2.1 - a1.1));

				let mut added_one = false;
				if anti1.0 >= 0 && anti1.0 < grid_height && anti1.1 >= 0 && anti1.1 < grid_width {
					antinodes.insert(anti1);
					added_one = true;
				}

				if anti2.0 >= 0 && anti2.0 < grid_height && anti2.1 >= 0 && anti2.1 < grid_width {
					antinodes.insert(anti2);
					added_one = true;
				}

				if !added_one {
					break;
				}
			}
		}
	}

	antinodes.len()
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if grid[r][c] != '.' {
				antennas
					.entry(grid[r][c])
					.or_default()
					.push((r as isize, c as isize));
			}
		}
	}

	let grid_height = grid.len() as isize;
	let grid_width = grid[0].len() as isize;
	println!(
		"Part 1: {}",
		sub_solve(&antennas, grid_height, grid_width, 1, 1)
	);
	println!(
		"Part 2: {}",
		sub_solve(
			&antennas,
			grid_height,
			grid_width,
			0,
			std::cmp::max(grid.len(), grid[0].len()) as isize
		)
	);
}
