use itertools::Itertools;

fn get_combo_operand(regs: &[i64; 3], operand: i64) -> i64 {
	match operand {
		0..=3 => operand,
		4 => regs[0],
		5 => regs[1],
		6 => regs[2],
		_ => unreachable!(),
	}
}

fn run_program(program: &[i64], mut regs: [i64; 3]) -> Vec<i64> {
	let mut result = Vec::new();
	let mut pc = 0;

	while pc < program.len() {
		let op = program[pc];
		let operand = program[pc + 1];
		pc += 2;

		match op {
			0 => regs[0] = regs[0] / (1i64 << get_combo_operand(&regs, operand)), // adv
			1 => regs[1] ^= operand,                                              // bxl
			2 => regs[1] = get_combo_operand(&regs, operand) % 8,                 // bst
			3 => {
				if regs[0] != 0 {
					pc = operand as usize;
				}
			}   // jnz
			4 => regs[1] ^= regs[2],                                              // bxc
			5 => result.push(get_combo_operand(&regs, operand) % 8),              // out
			6 => regs[1] = regs[0] / (1i64 << get_combo_operand(&regs, operand)), // bdv
			7 => regs[2] = regs[0] / (1i64 << get_combo_operand(&regs, operand)), // cdv
			_ => unreachable!(),
		}
	}

	return result;
}

// Reverse engineer the initial value of A by brute forcing the number
// by chunks of 3 bits, starting at the most significant bits first.
// There can be multiple valid values of each set of 3 bits, so we need to use
// a recursive function to try all possible values.
fn subsolve_part2(program: &[i64], regs: &[i64; 3], a_input: i64, digit: usize) -> Option<i64> {
	if digit == program.len() {
		return Some(a_input);
	}
	let a_input = a_input << 3; // Shift it over to start working on the next digit

	// Try all possible values for the next 3 bits of A
	for val in 0..8 {
		let sub_regs = [a_input | val, regs[1], regs[2]];
		let result = run_program(program, sub_regs);

		// Verify that the result matches the last `digit` values of the program
		if result != program[program.len() - digit - 1..] {
			continue;
		}

		if let Some(result) = subsolve_part2(program, regs, a_input | val, digit + 1) {
			return Some(result);
		}
	}

	return None;
}

pub fn solve(inputs: Vec<String>) {
	let (registers, program) = inputs
		.split(|line| line.is_empty())
		.collect_tuple()
		.unwrap();

	let parse_reg = |i: usize| {
		registers[i]
			.split_once(": ")
			.unwrap()
			.1
			.parse::<i64>()
			.unwrap()
	};
	let regs = [parse_reg(0), parse_reg(1), parse_reg(2)];

	let program = program[0]
		.split_once(": ")
		.unwrap()
		.1
		.split(",")
		.map(|s| s.parse::<i64>().unwrap())
		.collect_vec();

	let part1 = run_program(&program, regs.clone());
	println!("Part 1: {}", part1.iter().map(|x| x.to_string()).join(","));

	let result = subsolve_part2(&program, &regs, 0, 0);
	println!(
		"Part 2: {}",
		result.expect("Failed to find a valid part solution")
	);
}
