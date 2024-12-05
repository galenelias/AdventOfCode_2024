use itertools::Itertools;

fn is_valid(update: &[usize], rules: &[(usize, usize)]) -> bool {
	for rule in rules {
		let first_pos = update.iter().position(|x| x == &rule.0);
		let second_pos = update.iter().position(|x| x == &rule.1);

		if let (Some(first_pos), Some(second_pos)) = (first_pos, second_pos) {
			if first_pos > second_pos {
				return false;
			}
		}
	}

	true
}

fn fix_update(update: &mut Vec<usize>, rules: &[(usize, usize)]) {
	loop {
		let mut is_valid = true;
		for rule in rules {
			let first_pos = update.iter().position(|x| x == &rule.0);
			let second_pos = update.iter().position(|x| x == &rule.1);

			if let (Some(first_pos), Some(second_pos)) = (first_pos, second_pos) {
				if first_pos > second_pos {
					is_valid = false;
					update.remove(second_pos);
					update.insert(first_pos, rule.1);
					break;
				}
			}
		}
		if is_valid {
			break;
		}
	}
}

pub fn solve(inputs: Vec<String>) {
	let (rule_lines, mut update_lines) =
		inputs.split_at(inputs.iter().position(|x| x.is_empty()).unwrap());
	update_lines = &update_lines[1..]; // Omit the empty line

	let rules = rule_lines
		.iter()
		.map(|rule_str| {
			let rule_parts = rule_str.split_once('|').unwrap();
			(
				rule_parts.0.parse::<usize>().unwrap(),
				rule_parts.1.parse::<usize>().unwrap(),
			)
		})
		.collect_vec();

	let mut part1 = 0;
	let mut part2 = 0;

	for update_str in update_lines {
		let mut update = update_str
			.split(',')
			.map(|x| x.parse::<usize>().unwrap())
			.collect_vec();

		if is_valid(&update, &rules) {
			part1 += update[update.len() / 2];
		} else {
			fix_update(&mut update, &rules);
			part2 += update[update.len() / 2];
		}
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
