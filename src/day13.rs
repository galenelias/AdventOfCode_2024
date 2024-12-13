use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

struct Params {
	a: (i64, i64),
	b: (i64, i64),
	p: (i64, i64),
}

fn cost(a: i64, b: i64) -> i64 {
	a * 3 + b
}

fn sub_solve(params: &Params) -> Option<(i64, i64)> {
	let max_b = std::cmp::min(params.p.0 / params.b.0, params.p.1 / params.b.1);
	for b in (0..=max_b).rev() {
		let a = (params.p.0 - b * params.b.0) / params.a.0;
		if a * params.a.0 + b * params.b.0 == params.p.0 && a * params.a.1 + b * params.b.1 == params.p.1 {
			return Some((a, b));
		}
	}

	None
}



pub fn solve(inputs: Vec<String>) {
	let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
	let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
	let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

	let games = inputs.split(|s| s.is_empty()).collect_vec();

	let mut part1 = 0;
	for game in games {
		let caps_a = re_button_a.captures(&game[0]).unwrap();
		let caps_b = re_button_b.captures(&game[1]).unwrap();
		let caps_p = re_prize.captures(&game[2]).unwrap();

		let params = Params {
			a: (
				caps_a[1].parse::<i64>().unwrap(),
				caps_a[2].parse::<i64>().unwrap(),
			),
			b: (
				caps_b[1].parse::<i64>().unwrap(),
				caps_b[2].parse::<i64>().unwrap(),
			),
			p: (
				caps_p[1].parse::<i64>().unwrap() + 10000000000000,
				caps_p[2].parse::<i64>().unwrap() + 10000000000000,
			),
		};

		println!("Game: {:?}", game);
		if let Some((a, b)) = sub_solve(&params) {
			println!("Solution: a={}, b={}: {}", a, b, a * 3 + b);
			part1 += a * 3 + b;
		} else {
			println!("No solution found");
		}
	}
	println!("Part 1: {}", part1);
}
