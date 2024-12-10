use itertools::Itertools;
use std::collections::HashSet;

fn num_trailheads(
	grid: &Vec<Vec<u32>>,
	r: usize,
	c: usize,
	cur_num: u32,
) -> HashSet<(usize, usize)> {
	let mut result = HashSet::new();
	if grid[r][c] == 9 {
		result.insert((r, c));
		return result;
	}

	if r > 0 && grid[r - 1][c] == cur_num + 1 {
		result.extend(num_trailheads(grid, r - 1, c, cur_num + 1));
	}
	if r < grid.len() - 1 && grid[r + 1][c] == cur_num + 1 {
		result.extend(num_trailheads(grid, r + 1, c, cur_num + 1));
	}
	if c > 0 && grid[r][c - 1] == cur_num + 1 {
		result.extend(num_trailheads(grid, r, c - 1, cur_num + 1));
	}
	if c < grid[0].len() - 1 && grid[r][c + 1] == cur_num + 1 {
		result.extend(num_trailheads(grid, r, c + 1, cur_num + 1));
	}

	return result;
}

fn num_trailheads2(grid: &Vec<Vec<u32>>, r: usize, c: usize, cur_num: u32) -> usize {
	if grid[r][c] == 9 {
		return 1;
	}

	let mut result = 0;
	if r > 0 && grid[r - 1][c] == cur_num + 1 {
		result += num_trailheads2(grid, r - 1, c, cur_num + 1);
	}
	if r < grid.len() - 1 && grid[r + 1][c] == cur_num + 1 {
		result += num_trailheads2(grid, r + 1, c, cur_num + 1);
	}
	if c > 0 && grid[r][c - 1] == cur_num + 1 {
		result += num_trailheads2(grid, r, c - 1, cur_num + 1);
	}
	if c < grid[0].len() - 1 && grid[r][c + 1] == cur_num + 1 {
		result += num_trailheads2(grid, r, c + 1, cur_num + 1);
	}

	return result;
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
		.collect_vec();

	let mut part1 = 0;
	let mut part2 = 0;
	for r in 0..grid.len() {
		for c in 0..grid[0].len() {
			if grid[r][c] == 0 {
				part1 += num_trailheads(&grid, r, c, 0).len();
				part2 += num_trailheads2(&grid, r, c, 0);
			}
		}
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
