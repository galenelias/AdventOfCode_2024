use itertools::Itertools;
use regex::Regex;

struct Params {
	a: (i64, i64),
	b: (i64, i64),
	p: (i64, i64),
}

fn cost(a: i64, b: i64) -> i64 {
	a * 3 + b
}

fn sub_solve(params: &Params) -> Option<(i64, i64)> {
	let determinant = params.a.0 * params.b.1 - params.a.1 * params.b.0;
	if determinant == 0 {
		return None;
	}

	let determinant_a = params.p.0 * params.b.1 - params.p.1 * params.b.0;
	let determinant_b = params.a.0 * params.p.1 - params.a.1 * params.p.0;
	
	let a = determinant_a / determinant;
	let b = determinant_b / determinant;

	if a * params.a.0 + b * params.b.0 == params.p.0
		&& a * params.a.1 + b * params.b.1 == params.p.1
	{
		Some((a, b))
	} else {
		None
	}
}

pub fn solve(inputs: Vec<String>) {
	let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
	let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
	let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

	let games = inputs.split(|s| s.is_empty()).collect_vec();

	let mut part1 = 0;
	let mut part2 = 0;
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
				caps_p[1].parse::<i64>().unwrap(),
				caps_p[2].parse::<i64>().unwrap(),
			),
		};

		let params_part2 = Params {
			a: params.a,
			b: params.b,
			p: (
				params.p.0 + 10000000000000,
				params.p.1 + 10000000000000,
			),
		};

		if let Some((a, b)) = sub_solve(&params) {
			part1 += cost(a, b);
		}

		if let Some((a, b)) = sub_solve(&params_part2) {
			part2 += cost(a, b);
		}
	}
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
