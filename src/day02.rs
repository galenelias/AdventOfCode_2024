use itertools::Itertools;

fn is_safe(report: &[i32]) -> bool {
	let deltas = report
		.windows(2)
		.map(|pair| pair[1] - pair[0])
		.collect_vec();
	let is_decreasing = deltas.iter().all(|&x| x >= -3 && x <= -1);
	let is_increasing = deltas.iter().all(|&x| x >= 1 && x <= 3);
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
			is_safe(report)
				|| (0..report.len()).any(|i| {
					let mut report_clone = report.clone();
					report_clone.remove(i);
					is_safe(&report_clone)
				})
		})
		.count();

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}
