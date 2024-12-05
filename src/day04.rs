use itertools::Itertools;

fn is_xmas(grid: &Vec<Vec<char>>, r: usize, c: usize, dr: isize, dc: isize) -> bool {
	const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

	for i in 0..XMAS.len() as isize {
		let r = r.checked_add_signed(dr * i);
		let c = c.checked_add_signed(dc * i);
		if let Some((row, col)) = r.zip(c) {
			if row < grid.len() && col < grid[0].len() {
				if grid[row][col] != XMAS[i as usize] {
					return false;
				} else if i == XMAS.len() as isize - 1 {
					return true;
				}
			}
		}
	}
	return false;
}

fn is_x_mas(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
	if r == 0 || c == 0 || r == grid.len() - 1 || c == grid[0].len() - 1 || grid[r][c] != 'A' {
		return false;
	}

	let mut letters = vec![
		grid[r - 1][c - 1],
		grid[r - 1][c + 1],
		grid[r + 1][c + 1],
		grid[r + 1][c - 1],
	];
	letters.sort();

	if letters != ['M', 'M', 'S', 'S'] {
		return false;
	}

	// We just need to confirm that the M's and S's are adjacent to each other
	// which means the diagnals must be different
	return grid[r - 1][c - 1] != grid[r + 1][c + 1];
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut part1 = 0;
	for r in 0..grid.len() {
		for c in 0..grid[0].len() {
			for dr in -1..2 {
				for dc in -1..2 {
					if dr == 0 && dc == 0 {
						continue;
					}
					if is_xmas(&grid, r, c, dr, dc) {
						part1 += 1;
					}
				}
			}
		}
	}
	println!("Part 1: {part1}");

	let part2 = (0..grid.len())
		.map(|r| {
			(0..grid[r].len())
				.filter(|c| is_x_mas(&grid, r, *c))
				.count()
		})
		.sum::<usize>();

	println!("Part 2: {part2}");
}
