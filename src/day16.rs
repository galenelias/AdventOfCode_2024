use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Dir {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
	pos: (usize, usize),
	cost: usize,
	dir: Dir,
	path: Vec<(usize, usize)>,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost.cmp(&(other.cost)).reverse()
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn turn_left(dir: &Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Left,
		Dir::Down => Dir::Right,
		Dir::Left => Dir::Down,
		Dir::Right => Dir::Up,
	}
}

fn turn_right(dir: &Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Right,
		Dir::Down => Dir::Left,
		Dir::Left => Dir::Up,
		Dir::Right => Dir::Down,
	}
}

fn advance_pos(pos: (usize, usize), dir: &Dir) -> (usize, usize) {
	match dir {
		Dir::Up => (pos.0 - 1, pos.1),
		Dir::Down => (pos.0 + 1, pos.1),
		Dir::Left => (pos.0, pos.1 - 1),
		Dir::Right => (pos.0, pos.1 + 1),
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let start = grid
		.iter()
		.enumerate()
		.find_map(|(r, row)| {
			row.iter()
				.enumerate()
				.find_map(|(c, ch)| if *ch == 'S' { Some((r, c)) } else { None })
		})
		.unwrap();

	let end = grid
		.iter()
		.enumerate()
		.find_map(|(r, row)| {
			row.iter()
				.enumerate()
				.find_map(|(c, ch)| if *ch == 'E' { Some((r, c)) } else { None })
		})
		.unwrap();

	grid[start.0][start.1] = '.';
	grid[end.0][end.1] = '.';
	let grid = grid;

	let mut queue = BinaryHeap::new();
	let mut seen = HashMap::new();
	let mut on_best_path: HashSet<(usize, usize)> = HashSet::new();
	let mut best_cost = None;

	queue.push(State {
		pos: start,
		cost: 0,
		dir: Dir::Right,
		path: Vec::new(),
	});

	while !queue.is_empty() {
		let mut state = queue.pop().unwrap();

		if grid[state.pos.0][state.pos.1] == '#' {
			continue;
		}

		if let Some(existing_cost) = seen.get(&(state.pos, state.dir)) {
			if state.cost > *existing_cost {
				continue;
			}
		} else {
			seen.insert((state.pos, state.dir), state.cost);
		}

		if state.pos == end {
			if best_cost.is_none() || state.cost <= best_cost.unwrap() {
				if best_cost.is_none() {
					println!("Part 1: {}", state.cost);
					best_cost = Some(state.cost);
				}
				on_best_path.extend(state.path.iter());
			} else {
				break;
			}
		}

		if state.path.is_empty() || state.path.last().unwrap() != &state.pos {
			state.path.push(state.pos);
		}

		queue.push(State {
			pos: state.pos,
			cost: state.cost + 1000,
			dir: turn_left(&state.dir),
			path: state.path.clone(),
		});
		queue.push(State {
			pos: state.pos,
			cost: state.cost + 1000,
			dir: turn_right(&state.dir),
			path: state.path.clone(),
		});
		queue.push(State {
			pos: advance_pos(state.pos, &state.dir),
			cost: state.cost + 1,
			dir: state.dir,
			path: state.path.clone(),
		});
	}

	on_best_path.insert(end);
	println!("Part 2: {}", on_best_path.len());
}
