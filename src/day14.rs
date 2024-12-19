use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
	pos: (i64, i64),
	vel: (i64, i64),
}

pub fn solve(inputs: Vec<String>) {
	let re_input = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();

	let mut robots = inputs
		.iter()
		.map(|line| {
			let caps = re_input.captures(line).unwrap();

			let pos = (
				caps[1].parse::<i64>().unwrap(),
				caps[2].parse::<i64>().unwrap(),
			);
			let vel = (
				caps[3].parse::<i64>().unwrap(),
				caps[4].parse::<i64>().unwrap(),
			);

			Robot { pos, vel }
		})
		.collect_vec();

	const WIDTH: i64 = 101;
	const HEIGHT: i64 = 103;
	const MID_X: i64 = WIDTH / 2;
	const MID_Y: i64 = HEIGHT / 2;

	let mut min_rle_size = (WIDTH * HEIGHT) as usize;
	let mut min_rle_size_second = 0;

	for second in 0.. {
		for robot in robots.iter_mut() {
			robot.pos.0 += robot.vel.0;
			robot.pos.1 += robot.vel.1;

			if robot.pos.0 < 0 {
				robot.pos.0 += WIDTH;
			}
			if robot.pos.1 < 0 {
				robot.pos.1 += HEIGHT;
			}
			if robot.pos.0 >= WIDTH {
				robot.pos.0 -= WIDTH;
			}
			if robot.pos.1 >= HEIGHT {
				robot.pos.1 -= HEIGHT;
			}
		}

		let positions = robots.iter().fold(HashSet::new(), |mut set, robot| {
			set.insert(robot.pos);
			set
		});

		let mut robots_str = String::with_capacity((WIDTH * HEIGHT) as usize);
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				if positions.contains(&(x, y)) {
					robots_str.push('#');
				} else {
					robots_str.push('.');
				}
			}
		}

		// Get run length encoded size of robots_str
		let rle_size = robots_str.chars().chunk_by(|&c| c).into_iter().count();

		if rle_size == min_rle_size && second > 10000 {
			for y in 0..HEIGHT {
				for x in 0..WIDTH {
					if positions.contains(&(x, y)) {
						print!("#");
					} else {
						print!(".");
					}
				}
				println!();
			}
			println!("Part 2: {} ({})", min_rle_size_second, min_rle_size);
			break;
		}

		if rle_size < min_rle_size {
			min_rle_size = rle_size;
			min_rle_size_second = second + 1;
			println!("New minimum: {} at {}", min_rle_size, min_rle_size_second);
		}

		if second == 99 {
			let quad_a = robots
				.iter()
				.filter(|robot| robot.pos.0 < MID_X && robot.pos.1 < MID_Y)
				.count();
			let quad_b = robots
				.iter()
				.filter(|robot| robot.pos.0 > MID_X && robot.pos.1 < MID_Y)
				.count();
			let quad_c = robots
				.iter()
				.filter(|robot| robot.pos.0 < MID_X && robot.pos.1 > MID_Y)
				.count();
			let quad_d = robots
				.iter()
				.filter(|robot| robot.pos.0 > MID_X && robot.pos.1 > MID_Y)
				.count();

			println!("Part 1: {}", quad_a * quad_b * quad_c * quad_d);
		}
	}
}
