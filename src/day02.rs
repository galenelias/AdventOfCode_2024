use itertools::Itertools;

fn is_safe(report: &[i32]) -> bool {
	let mut is_increasing = true;
	let mut is_decreasing = true;
	for i in 1..report.len() {
		let diff = report[i] - report[i - 1];
		if !(diff >= 1 && diff <= 3) {
			is_increasing = false;
		}

		if !(diff >= -3 && diff <= -1) {
			is_decreasing = false;
		}
	}

	return is_decreasing || is_increasing;
}

pub fn solve(inputs: Vec<String>) {
	let reports = inputs
		.iter()
		.map(|line| {
			line.split_whitespace()
				.map(|x| x.parse::<i32>().unwrap())
				.collect_vec()
		})
		.collect_vec();
	let part1 = reports.iter().filter(|report| is_safe(report)).count();
	let part2 = reports
		.iter()
		.filter(|&report| {
			let mut any_safe = is_safe(report);
			for i in 0..report.len() {
				let mut report_clone = report.clone();
				report_clone.remove(i);

				any_safe = any_safe || is_safe(&report_clone);
			}

			return any_safe;
		})
		.count();

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}
