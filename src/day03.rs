use regex::Regex;

pub fn solve(inputs: Vec<String>) {
	let re_part1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
	let re_part2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

	let part1 = inputs
		.iter()
		.map(|input| {
			re_part1
				.captures_iter(&input)
				.map(|cap| {
					let a = cap[1].parse::<i64>().unwrap();
					let b = cap[2].parse::<i64>().unwrap();
					a * b
				})
				.sum::<i64>()
		})
		.sum::<i64>();

	println!("Part 1: {part1}");

	let mut enabled = true;
	let part2 = inputs
		.iter()
		.map(|input| {
			re_part2
				.captures_iter(input)
				.map(|cap| match &cap[0] {
					"do()" => {
						enabled = true;
						0
					}
					"don't()" => {
						enabled = false;
						0
					}
					_ if enabled => {
						let a = cap[1].parse::<i64>().unwrap();
						let b = cap[2].parse::<i64>().unwrap();
						a * b
					}
					_ => 0,
				})
				.sum::<i64>()
		})
		.sum::<i64>();
	println!("Part 2: {part2}");
}
