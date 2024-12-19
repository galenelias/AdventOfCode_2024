use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const GRID_SIZE: usize = 71;

fn has_path(grid: &Vec<Vec<char>>) -> Option<usize> {
	let mut seen = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back((0 as isize, 0 as isize, 0));

	while !queue.is_empty() {
		let (x, y, steps) = queue.pop_front().unwrap();

		if x < 0 || x >= GRID_SIZE as isize || y < 0 || y >= GRID_SIZE as isize {
			continue;
		}

		if grid[y as usize][x as usize] == '#' {
			continue;
		}

		if x as usize == GRID_SIZE - 1 && y as usize == GRID_SIZE - 1 {
			return Some(steps);
		}

		if !seen.insert((x, y)) {
			continue;
		}

		queue.push_back((x - 1, y, steps + 1));
		queue.push_back((x + 1, y, steps + 1));
		queue.push_back((x, y - 1, steps + 1));
		queue.push_back((x, y + 1, steps + 1));
	}
	None
}

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs
		.iter()
		.map(|line| {
			let (x, y) = line.split_once(",").unwrap();
			(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
		})
		.collect_vec();

	let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

	for (x, y) in inputs.iter().take(1024) {
		grid[*y][*x] = '#';
	}

	let part1 = has_path(&grid).unwrap();
	println!("Part 1: {}", part1);

	for (x, y) in inputs.iter().skip(1024) {
		grid[*y][*x] = '#';

		if has_path(&grid).is_none() {
			println!("Part 2: {},{}", x, y);
			break;
		}
	}
}
