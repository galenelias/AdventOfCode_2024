use itertools::Itertools;
use std::collections::HashMap;

const NUM_KEYPAD: [[char; 3]; 4] = [
	['7', '8', '9'],
	['4', '5', '6'],
	['1', '2', '3'],
	['#', '0', 'A'],
];

const DIR_KEYPAD: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

fn key_to_pos<const ROW_COUNT: usize, const COL_COUNT: usize>(
	key: char,
	grid: &[[char; COL_COUNT]; ROW_COUNT],
) -> (usize, usize) {
	for (r, row) in grid.iter().enumerate() {
		for (c, &ch) in row.iter().enumerate() {
			if ch == key {
				return (r, c);
			}
		}
	}
	unreachable!("Invalid key: {}", key);
}

fn path_valid<const ROW_COUNT: usize, const COL_COUNT: usize>(
	start: (usize, usize),
	path: &[char],
	grid: &[[char; COL_COUNT]; ROW_COUNT],
) -> bool {
	let mut pos = start;
	for &ch in path {
		match ch {
			'^' => pos.0 = pos.0.saturating_sub(1),
			'v' => pos.0 = pos.0.saturating_add(1),
			'<' => pos.1 = pos.1.saturating_sub(1),
			'>' => pos.1 = pos.1.saturating_add(1),
			'A' => (),
			_ => unreachable!("Invalid direction: {}", ch),
		}
		if grid[pos.0][pos.1] == '#' {
			return false;
		}
	}
	return true;
}

fn get_seq_cost<const ROW_COUNT: usize, const COL_COUNT: usize>(
	path: &[char],
	grid: &[[char; COL_COUNT]; ROW_COUNT],
	level: usize,
	memo: &mut HashMap<(String, usize), usize>,
) -> usize {
	if level == 0 {
		return path.len();
	}

	let path_str = path.iter().collect::<String>();
	if let Some(&cost) = memo.get(&(path_str.clone(), level)) {
		return cost;
	}

	let mut pos = key_to_pos('A', grid);
	let mut cost = 0;
	for ch in path {
		let next_pos = key_to_pos(*ch, grid);
		cost += get_path_cost(grid, pos, next_pos, level, memo);
		pos = next_pos;
	}

	memo.insert((path_str, level), cost);

	cost
}

fn get_path_cost<const ROW_COUNT: usize, const COL_COUNT: usize>(
	grid: &[[char; COL_COUNT]; ROW_COUNT],
	start: (usize, usize),
	end: (usize, usize),
	level: usize,
	memo: &mut HashMap<(String, usize), usize>,
) -> usize {
	let dr = end.0 as isize - start.0 as isize;
	let dc = end.1 as isize - start.1 as isize;

	let vertical_path = if dr > 0 {
		['v'].repeat(dr as usize)
	} else {
		['^'].repeat(-dr as usize)
	};
	let horizontal_path = if dc > 0 {
		['>'].repeat(dc as usize)
	} else {
		['<'].repeat(-dc as usize)
	};

	let mut path1 = Vec::new();
	path1.extend(vertical_path.clone());
	path1.extend(horizontal_path.clone());
	path1.push('A');

	if dr != 0 && dc != 0 {
		// Try both combinations of moving by row or column first to see which is faster
		let mut path2 = Vec::new();
		path2.extend(horizontal_path);
		path2.extend(vertical_path);
		path2.push('A');

		let path1_cost = if path_valid(start, &path1, grid) {
			get_seq_cost(&path1, &DIR_KEYPAD, level - 1, memo)
		} else {
			usize::MAX
		};

		let path2_cost = if path_valid(start, &path2, grid) {
			get_seq_cost(&path2, &DIR_KEYPAD, level - 1, memo)
		} else {
			usize::MAX
		};

		return std::cmp::min(path1_cost, path2_cost);
	} else {
		// Otherwise, our 'path1' is the only option
		return get_seq_cost(&path1, &DIR_KEYPAD, level - 1, memo);
	}
}

fn subsolve(number: &[char], robot_layers: usize) -> usize {
	let mut memo = HashMap::new();
	get_seq_cost(number, &NUM_KEYPAD, robot_layers + 1, &mut memo)
}

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	let mut part2 = 0;
	for input in inputs {
		let number = input[0..3].parse::<usize>().unwrap();
		let input = input.chars().collect_vec();
		let part1_cost = subsolve(&input, 2);
		let part2_cost = subsolve(&input, 25);

		part1 += number * part1_cost;
		part2 += number * part2_cost;
	}
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
