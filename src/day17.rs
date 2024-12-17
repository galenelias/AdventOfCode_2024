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
			}                       // jnz
			4 => regs[1] ^= regs[2],                                              // bxc
			5 => result.push(get_combo_operand(&regs, operand) % 8),              // out
			6 => regs[1] = regs[0] / (1i64 << get_combo_operand(&regs, operand)), // bdv
			7 => regs[2] = regs[0] / (1i64 << get_combo_operand(&regs, operand)), // cdv }
			_ => unreachable!(),
		}
	}

	return result;
}

fn run_program2(mut a: i64) -> Vec<i64> {
	let mut result = Vec::with_capacity(16);

	loop {
		let mut b = a & 7;
		b ^= 5;
		let c = (a >> b) & 7;
		b ^= c;
		b ^= 6;
		result.push(b);
		a >>= 3;
		if a == 0 {
			break;
		}
	}

	return result;
}

// Reverse engineer the initial value of A by brute forcing the number
// by chunks of 3 bits, starting at the most significant bits first.
// There can be multiple valid values of each set of 3 bits, so we need to use
// a recursive function to try all possible values.
fn solve_part2(program: &[i64], a_input: i64, digit: usize) -> Option<i64> {
	if digit == program.len() {
		// Reached the end! Unshift the answer
		return Some(a_input);
	}
	let a_input = a_input << 3; // Shift if over to start working on the next digit

	// Try all possible values for the next 3 bits
	'outer: for val in 0..8 {
		let mut a = a_input | val;

		// Now verify that all the digits so far still generate correctly
		for i in 0..=digit {
			let mut b = a & 7;
			b ^= 5;
			let c = (a >> b) & 7;
			b ^= c;
			b ^= 6;

			if program[program.len() - digit - 1 + i] != b {
				continue 'outer;
			}
			a >>= 3;
		}

		if let Some(result) = solve_part2(program, a_input | val, digit + 1) {
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

	let parse_reg = |s: &str| s.split_once(": ").unwrap().1.parse::<i64>().unwrap();

	let mut regs = [0, 0, 0];
	regs[0] = parse_reg(&registers[0]);
	regs[1] = parse_reg(&registers[1]);
	regs[2] = parse_reg(&registers[2]);

	let ops = program[0]
		.split_once(": ")
		.unwrap()
		.1
		.split(",")
		.map(|s| s.parse::<i64>().unwrap())
		.collect_vec();

	let part1 = run_program(&ops, regs.clone());
	println!("Part 1: {}", part1.iter().map(|x| x.to_string()).join(","));

	let result = solve_part2(&ops, 0, 0);
	println!(
		"Part 2: {}",
		result.expect("Failed to find a valid part solution")
	);
}
