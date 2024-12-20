use itertools::Itertools;
use std::collections::{HashSet, HashMap, BinaryHeap, VecDeque};
use std::cmp::Ordering;

fn bfs2(
	grid: &Vec<Vec<char>>,
	start: (usize, usize),
	end: (usize, usize),
	cheat_length: usize,
) -> usize {
	let mut queue: VecDeque<((usize, usize), usize, Option<usize>)> = VecDeque::new();
	let mut seen = HashSet::new();

	queue.push_back((start, 0, None));

	while !queue.is_empty() {
		let (pos, steps, cheat_remaining) = queue.pop_front().unwrap();

		if grid[pos.0][pos.1] == '#' && cheat_remaining.unwrap_or(0) == 0 {
			continue;
		}

		if pos.0 == 0 || pos.0 == grid.len() - 1 || pos.1 == 0 || pos.1 == grid[0].len() - 1 {
			continue;
		}

		if pos == end {
			return steps;
		}

		if !seen.insert(pos) {
			continue;
		}

		let cheat_left = match cheat_remaining {
			Some(0) => Some(0),
			Some(n) => Some(n - 1),
			None => None,
		};

		queue.push_back(((pos.0 - 1, pos.1), steps + 1, cheat_left));
		queue.push_back(((pos.0 + 1, pos.1), steps + 1, cheat_left));
		queue.push_back(((pos.0, pos.1 - 1), steps + 1, cheat_left));
		queue.push_back(((pos.0, pos.1 + 1), steps + 1, cheat_left));

		if cheat_remaining.is_none() && cheat_length > 0 {
			queue.push_back(((pos.0 - 1, pos.1), steps + 1, Some(cheat_length - 1)));
			queue.push_back(((pos.0 + 1, pos.1), steps + 1, Some(cheat_length - 1)));
			queue.push_back(((pos.0, pos.1 - 1), steps + 1, Some(cheat_length - 1)));
			queue.push_back(((pos.0, pos.1 + 1), steps + 1, Some(cheat_length - 1)));
		}
	}

	unreachable!()
}

fn grid_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
	p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
	pos: (usize, usize),
	steps: usize,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.steps.cmp(&(other.steps)).reverse()
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn bfs(
	grid: &Vec<Vec<char>>,
	start: (usize, usize),
	end: (usize, usize),
	cheat_start: (usize, usize),
	chat_end: (usize, usize),
) -> usize {
	let mut queue = BinaryHeap::new();
	let mut seen = HashSet::new();

	queue.push(State{ pos: start, steps: 0});

	while !queue.is_empty() {
		let state = queue.pop().unwrap();
		let (pos, steps) = (state.pos, state.steps);

		if grid[pos.0][pos.1] == '#' {
			continue;
		}

		if pos.0 == 0 || pos.0 == grid.len() - 1 || pos.1 == 0 || pos.1 == grid[0].len() - 1 {
			continue;
		}

		if pos == end {
			return steps;
		}

		if pos == cheat_start {
			queue.push(State{ pos: chat_end, steps: steps + grid_dist(cheat_start, chat_end)});
			continue;
		}

		if !seen.insert(pos) {
			continue;
		}

		queue.push(State{pos: (pos.0 - 1, pos.1), steps: steps + 1});
		queue.push(State{pos: (pos.0 + 1, pos.1), steps: steps + 1});
		queue.push(State{pos: (pos.0, pos.1 - 1), steps: steps + 1});
		queue.push(State{pos: (pos.0, pos.1 + 1), steps: steps + 1});
	}

	usize::MAX
	// unreachable!()
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

	let base_time = bfs(&grid, start, end, (0,0), (0, 0));

	let mut part1 = 0;
	// let mut cheat_savings = HashMap::new();

	for r in 1..grid.len()-1 {
		println!("Row {}", r);
	    for c in 1..grid[r].len()-1 {
	        if grid[r][c] == '.' {
				for r2 in 1..grid.len()-1 {
					for c2 in 1..grid[r2].len()-1 {
						if (r != r2 || c != c2) && grid[r2][c2] == '.' && grid_dist((r, c), (r2, c2)) <= 20 {
							let time = bfs(&grid, start, end, (r, c), (r2, c2));
							if time < base_time {
								if base_time - time >= 100 {
									part1 += 1;
								}
								// if (base_time - time) >= 50 {
								// 	*cheat_savings.entry(base_time - time).or_insert(0) += 1;
								// 	// println!("Saved {}", base_time - time);
								// }
							}
						}
					}
				}
	        }
	    }
	}

	// for k in cheat_savings.keys().sorted() {
	// 	println!("There are {} cheats that saved {} picoseconds.", cheat_savings[k], k);
	// }

	println!("Part 1: {}", part1);
}
