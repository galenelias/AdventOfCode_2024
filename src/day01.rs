use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	println!(
		"Part 1: {}",
		inputs
			.iter()
			.map(|x| x.parse::<i32>().unwrap())
			.sum::<i32>()
	);
}
