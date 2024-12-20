use itertools::Itertools;
use std::collections::VecDeque;

fn grid_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
	p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
	pos: (usize, usize),
	steps: usize,
}

fn compute_distances(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<usize>> {
	let mut dist = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
	let mut queue = VecDeque::new();

	queue.push_back(State {
		pos: start,
		steps: 0,
	});

	while !queue.is_empty() {
		let state = queue.pop_front().unwrap();
		let (pos, steps) = (state.pos, state.steps);

		if grid[pos.0][pos.1] == '#' {
			continue;
		}

		if pos.0 == 0 || pos.0 == grid.len() - 1 || pos.1 == 0 || pos.1 == grid[0].len() - 1 {
			continue;
		}

		if dist[pos.0][pos.1] != usize::MAX {
			continue;
		}

		dist[pos.0][pos.1] = steps;

		queue.push_back(State {
			pos: (pos.0 - 1, pos.1),
			steps: steps + 1,
		});
		queue.push_back(State {
			pos: (pos.0 + 1, pos.1),
			steps: steps + 1,
		});
		queue.push_back(State {
			pos: (pos.0, pos.1 - 1),
			steps: steps + 1,
		});
		queue.push_back(State {
			pos: (pos.0, pos.1 + 1),
			steps: steps + 1,
		});
	}

	dist
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

	let dist_from_start = compute_distances(&grid, start);
	let dist_to_end = compute_distances(&grid, end);
	let base_time = dist_from_start[end.0][end.1];

	// Find all (Cheat Start, Cheat End) positions which would save steps
	// The cheat acts as a portal stitching distance to Cheat Start with the distance
	// from Cheat End to the finish with the manhattan distance between Cheat Start and Cheat End
	let find_cheats = |cheat_len: usize| {
		let mut cheats = 0;
		for r1 in 1..grid.len() - 1 {
			for c1 in 1..grid[r1].len() - 1 {
				if grid[r1][c1] == '.' {
					for r2 in 1..grid.len() - 1 {
						for c2 in 1..grid[r2].len() - 1 {
							if (r1 != r2 || c1 != c2)
								&& grid[r2][c2] == '.' && grid_dist((r1, c1), (r2, c2)) <= cheat_len
							{
								let time = dist_from_start[r1][c1]
									+ dist_to_end[r2][c2] + grid_dist((r1, c1), (r2, c2));
								if time < base_time {
									if base_time - time >= 100 {
										cheats += 1;
									}
								}
							}
						}
					}
				}
			}
		}
		cheats
	};

	println!("Part 1: {}", find_cheats(2));
	println!("Part 2: {}", find_cheats(20));
}
