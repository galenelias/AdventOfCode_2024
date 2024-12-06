use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

fn move_guard(
	grid: &Vec<Vec<char>>,
	pos: (isize, isize),
	mut dir: Direction,
) -> Option<((isize, isize), Direction)> {
	let (r, c) = pos;

	loop {
		let (new_r, new_c) = match dir {
			Direction::Up => (r - 1, c),
			Direction::Down => (r + 1, c),
			Direction::Left => (r, c - 1),
			Direction::Right => (r, c + 1),
		};

		// Walk out of bounds, return None.
		if new_r < 0 || new_r >= grid.len() as isize || new_c < 0 || new_c >= grid[0].len() as isize
		{
			return None;
		}

		// Walk into an empty space, return new position.
		if grid[new_r as usize][new_c as usize] != '#' {
			return Some(((new_r, new_c), dir));
		}

		// Else, turn right.
		dir = match dir {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		};
	}
}

fn part1(grid: &Vec<Vec<char>>, mut guard_pos: (isize, isize)) -> HashSet<(isize, isize)> {
	let mut dir = Direction::Up;
	let mut visited = HashSet::new();
	loop {
		visited.insert(guard_pos);

		if let Some((new_pos, new_dir)) = move_guard(&grid, guard_pos, dir) {
			guard_pos = new_pos;
			dir = new_dir;
		} else {
			break;
		}
	}

	println!("Part 1: {}", visited.len());
	return visited;
}

fn is_loop(grid: &Vec<Vec<char>>, mut guard_pos: (isize, isize)) -> bool {
	let mut dir = Direction::Up;
	let mut seen = HashSet::new();

	loop {
		if seen.contains(&(guard_pos, dir)) {
			return true;
		}

		seen.insert((guard_pos, dir));

		if let Some((new_pos, new_dir)) = move_guard(&grid, guard_pos, dir) {
			guard_pos = new_pos;
			dir = new_dir;
		} else {
			return false;
		}
	}
}

fn part2(mut grid: Vec<Vec<char>>, visited: &HashSet<(isize, isize)>, guard_pos: (isize, isize)) {
	let mut part2 = 0;
	for (r, c) in visited {
		let (r, c) = (*r as usize, *c as usize);
		if grid[r][c] == '.' {
			grid[r][c] = '#';
			if is_loop(&grid, guard_pos) {
				part2 += 1;
			}
			grid[r][c] = '.';
		}
	}
	println!("Part 2: {}", part2);
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let guard_pos = grid
		.iter()
		.enumerate()
		.find_map(|(r, row)| {
			row.iter().enumerate().find_map(|(c, ch)| {
				if *ch == '^' {
					Some((r as isize, c as isize))
				} else {
					None
				}
			})
		})
		.unwrap();

	let visited = part1(&grid, guard_pos);
	part2(grid, &visited, guard_pos);
}
