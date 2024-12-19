use itertools::Itertools;

fn concat_nums(num1: i64, num2: i64) -> i64 {
	let num2_digits = num2.checked_ilog10().unwrap() + 1;
	num1 * 10i64.pow(num2_digits as u32) + num2
}

fn can_total(goal_total: i64, parts: &[i64], op_count: usize) -> bool {
	let bits = parts.len() - 1;

	// Represent possible sets of operations as a binary/ternary number, where each digit represents an operation.
	// Apply the operation by taking the digit's value mod op_count.
	for mut perm in 0..op_count.pow(bits as u32) {
		let mut total = parts[0];
		for i in 0..parts.len() - 1 {
			match perm % op_count {
				0 => total += parts[i + 1],
				1 => total *= parts[i + 1],
				2 => total = concat_nums(total, parts[i + 1]),
				_ => unreachable!(),
			};
			perm /= op_count;
		}

		if total == goal_total {
			return true;
		}
	}
	false
}

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	let mut part2 = 0;
	for input in inputs {
		let (total, parts) = input.split_once(": ").unwrap();
		let total = total.parse::<i64>().unwrap();
		let parts = parts
			.split_whitespace()
			.map(|x| x.parse::<i64>().unwrap())
			.collect_vec();

		if can_total(total, &parts, 2) {
			part1 += total;
			part2 += total;
		} else if can_total(total, &parts, 3) {
			part2 += total;
		}
	}

	println!("Part1: {}", part1);
	println!("Part2: {}", part2);
}
