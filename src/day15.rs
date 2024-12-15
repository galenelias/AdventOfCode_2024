use itertools::Itertools;

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>, robot_pos: (isize, isize)) {
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if (r as isize, c as isize) == robot_pos {
				print!("@");
			} else {
				print!("{}", grid[r][c]);
			}
		}
		println!();
	}
	println!();
}

fn gps_sum(grid: &Vec<Vec<char>>) -> usize {
	grid.iter()
		.enumerate()
		.map(|(r, row)| {
			row.iter()
				.enumerate()
				.map(|(c, ch)| if *ch == 'O' || *ch == '[' { r * 100 + c } else { 0 })
				.sum::<usize>()
		})
		.sum()
}

fn try_move(grid: &mut Vec<Vec<char>>, robot_pos: &mut (isize, isize), dr: isize, dc: isize) {
	let (r, c) = (robot_pos.0, robot_pos.1);

	for steps in 1.. {
		let new_r = r + steps * dr;
		let new_c = c + steps * dc;

		if new_r < 0 || new_r >= grid.len() as isize || new_c < 0 || new_c >= grid[0].len() as isize
		{
			break;
		}
		let ch = grid[new_r as usize][new_c as usize];
		if ch == '.' {
			// Found empty space, move robot and shift any boxes over
			// Shift boxes. Any steps above 1 means we have a continous line of boxes to move
			*robot_pos = (r + dr, c + dc);
			if steps > 1 {
				grid[new_r as usize][new_c as usize] = 'O';
				grid[(r + dr) as usize][(c + dc) as usize] = '.';
			}
			break;
		} else if ch == '#' {
			break;
		} else if ch == 'O' {
			continue;
		} else {
			unreachable!();
		}
	}
}

fn part1(mut grid: Vec<Vec<char>>, movements: &[char], mut robot_pos: (isize, isize)) {
	for movement in movements {
		match movement {
			'^' => try_move(&mut grid, &mut robot_pos, -1, 0),
			'v' => try_move(&mut grid, &mut robot_pos, 1, 0),
			'<' => try_move(&mut grid, &mut robot_pos, 0, -1),
			'>' => try_move(&mut grid, &mut robot_pos, 0, 1),
			_ => {}
		}
		// print_grid(&grid, robot_pos);
	}

	println!("Part 1: {}", gps_sum(&grid));
}

fn try_move_box(grid: &mut Vec<Vec<char>>, box_pos: (isize, isize), dr: isize, dc: isize, do_move: bool) -> bool {
	if dc == -1 {
		let ch = grid[box_pos.0 as usize][(box_pos.1 - 1) as usize];
		let can_move = match ch {
			'.' => true,
			'#' => false,
			']' => try_move_box(grid, (box_pos.0, box_pos.1 - 2), dr, dc, do_move),
			_ => unreachable!(),
		};
		if can_move && do_move {
			grid[box_pos.0 as usize][(box_pos.1 - 1) as usize] = '[';
			grid[box_pos.0 as usize][box_pos.1 as usize] = ']';
			grid[box_pos.0 as usize][(box_pos.1 + 1) as usize] = '.';
		}
		return can_move;
	}
	else if dc == 1 {
		let ch = grid[box_pos.0 as usize][(box_pos.1 + 2) as usize];
		let can_move = match ch {
			'.' => true,
			'#' => false,
			'[' => try_move_box(grid, (box_pos.0, box_pos.1 + 2), dr, dc, do_move),
			_ => unreachable!(),
		};
		if can_move && do_move {
			grid[box_pos.0 as usize][box_pos.1 as usize] = '.';
			grid[box_pos.0 as usize][(box_pos.1 + 1) as usize] = '[';
			grid[box_pos.0 as usize][(box_pos.1 + 2) as usize] = ']';
		}
		return can_move;
	}
	else {
		let ch1 = grid[(box_pos.0 + dr) as usize][box_pos.1 as usize];
		let ch2 = grid[(box_pos.0 + dr) as usize][(box_pos.1 + 1) as usize];
		let can_move_1 = match ch1 {
			'.' => true,
			'#' => false,
			']' => try_move_box(grid, (box_pos.0 + dr, box_pos.1 - 1), dr, dc, do_move),
			'[' => try_move_box(grid, (box_pos.0 + dr, box_pos.1), dr, dc, do_move),
			_ => unreachable!(),
		};
		let can_move_2 = match ch2 {
			'.' => true,
			'#' => false,
			'[' => try_move_box(grid, (box_pos.0 + dr, box_pos.1 + 1), dr, dc, do_move),
			']' => true,
			_ => unreachable!(),
		};

		if can_move_1 && can_move_2 && do_move {
			grid[(box_pos.0 + dr) as usize][box_pos.1 as usize] = '[';
			grid[(box_pos.0 + dr) as usize][(box_pos.1 + 1) as usize] = ']';
			grid[box_pos.0 as usize][box_pos.1 as usize] = '.';
			grid[box_pos.0 as usize][(box_pos.1 + 1) as usize] = '.';
		}
		return can_move_1 && can_move_2;
	}
}

fn try_move2(grid: &mut Vec<Vec<char>>, robot_pos: &mut (isize, isize), dr: isize, dc: isize) {
	let (r, c) = (robot_pos.0, robot_pos.1);

	let new_r = r + dr;
	let new_c = c + dc;

	if new_r < 0 || new_r >= grid.len() as isize || new_c < 0 || new_c >= grid[0].len() as isize
	{
		return;
	}

	let ch = grid[new_r as usize][new_c as usize];
	match ch {
		'.' => *robot_pos = (new_r, new_c),
		'#' => return,
		'[' => {
			if try_move_box(grid, (new_r, new_c), dr, dc, false) {
				*robot_pos = (new_r, new_c);
				try_move_box(grid, (new_r, new_c), dr, dc, true);
			}
		}
		']' => {
			if try_move_box(grid, (new_r, new_c - 1), dr, dc, false) {
				*robot_pos = (new_r, new_c);
				try_move_box(grid, (new_r, new_c - 1), dr, dc, true);
			}
		}
		_ => unreachable!(),
	}
}

fn part2(grid_p1: Vec<Vec<char>>, movements: &[char], mut robot_pos: (isize, isize)) {
	let mut grid = grid_p1
		.iter()
		.map(|row| {
			row.iter()
				.map(|ch| match ch {
					'O' => ['[', ']'],
					'#' => ['#', '#'],
					'.' => ['.', '.'],
					_ => unreachable!(),
				})
				.flatten()
				.collect_vec()
		})
		.collect_vec();

	robot_pos.1 *= 2;

	for movement in movements {
		match movement {
			'^' => try_move2(&mut grid, &mut robot_pos, -1, 0),
			'v' => try_move2(&mut grid, &mut robot_pos, 1, 0),
			'<' => try_move2(&mut grid, &mut robot_pos, 0, -1),
			'>' => try_move2(&mut grid, &mut robot_pos, 0, 1),
			_ => {}
		}
		// print_grid(&grid, robot_pos);
	}
	println!("Part 2: {}", gps_sum(&grid));
}

pub fn solve(inputs: Vec<String>) {
	let (grid_str, movements_str) = inputs
		.split(|line| line.is_empty())
		.collect_tuple()
		.unwrap();
	let mut grid = grid_str
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let robot_pos = grid
		.iter()
		.enumerate()
		.find_map(|(r, row)| {
			row.iter().enumerate().find_map(|(c, ch)| {
				if *ch == '@' {
					Some((r as isize, c as isize))
				} else {
					None
				}
			})
		})
		.unwrap();

	grid[robot_pos.0 as usize][robot_pos.1 as usize] = '.';
	let movements = movements_str
		.iter()
		.map(|line| line.chars().collect_vec())
		.flatten()
		.collect_vec();

	part1(grid.clone(), &movements, robot_pos);
	part2(grid.clone(), &movements, robot_pos);
}
