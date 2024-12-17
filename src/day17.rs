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
			0 => {
				// adv
				let numerator = regs[0];
				let denominator = 2i64.pow(get_combo_operand(&regs, operand) as u32);
				regs[0] = numerator / denominator;
			}
			1 => {
				// bxl
				regs[1] = regs[1] ^ operand;
			}
			2 => {
				// bst
				regs[1] = get_combo_operand(&regs, operand) % 8;
			}
			3 => {
				// jnz
				if regs[0] != 0 {
					pc = operand as usize;
				}
			}
			4 => {
				// bxc
				regs[1] = regs[1] ^ regs[2];
			}
			5 => {
				// out
				result.push(get_combo_operand(&regs, operand) % 8);
			}
			6 => {
				// bdv
				let numerator = regs[0];
				let denominator = 2i64.pow(get_combo_operand(&regs, operand) as u32);
				regs[1] = numerator / denominator;
			}
			7 => {
				let numerator = regs[0];
				let denominator = 2i64.pow(get_combo_operand(&regs, operand) as u32);
				regs[2] = numerator / denominator;
			}
			_ => unreachable!(),
		}
	}

	return result;
}

fn run_program2(mut a: i64) -> Vec<i64> {
	let mut result = Vec::with_capacity(9);
	let mut b;
	let mut c;

	loop {
		b = a & 7;
		b ^= 5;
		c = (a >> b) & 7;
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

fn run_program3(mut a: i64, program: &[i64]) -> bool {
	let mut b;
	let mut c;
	let mut i = 0;

	loop {
		b = a & 7;
		b ^= 5;
		c = (a >> b) & 7;
		b ^= c;
		b ^= 6;

		if program[i] != b {
			return false;
		}
		i += 1;
		a >>= 3;
		if a == 0 {
			break;
		}
	}

	return i == program.len();
}

fn run_program4(program: &[i64]) -> i64 {
	let mut a_locked = 0;

	// Lock in the final A value 3 bits at a time
	for trip in 0..program.len() {
		println!("Trying triplet {}", trip);
		let mut found_one = false;
		'outer: for v in 0..8 {
			let mut a = a_locked | v;
			for i in 0..=trip {
				let mut b = a & 7;
				b ^= 5;
				let c = (a >> b) & 7;
				b ^= c;
				b ^= 6;
		
				if program[program.len() - trip - 1 + i] != b {
					continue 'outer;
				}
				a >>= 3;
			}

			println!("Locked in {} for triplet {}. A = {}", v, trip, a_locked);
			if !found_one {
				a_locked |= v;
				a_locked <<= 3;
				found_one = true;
			}
			// break;
		}
	}

	return a_locked;
}

pub fn solve(inputs: Vec<String>) {
	let (registers, program) = inputs
		.split(|line| line.is_empty())
		.collect_tuple()
		.unwrap();

	let mut regs = [0, 0, 0];

	regs[0] = registers[0]
		.split_once(": ")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();
	regs[1] = registers[1]
		.split_once(": ")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();
	regs[2] = registers[2]
		.split_once(": ")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();

	let ops = program[0]
		.split_once(": ")
		.unwrap()
		.1
		.split(",")
		.map(|s| s.parse::<i64>().unwrap())
		.collect_vec();

	let part1 = run_program(&ops, regs.clone());
	println!("Part 1:     {}", part1.iter().map(|x| x.to_string()).join(","));

	let part1_2 = run_program(&ops, regs.clone());
	println!("Part 1 (2): {}", part1_2.iter().map(|x| x.to_string()).join(","));

	let debug1 = run_program2(105690555219968);
	println!("Debug 1: {}", debug1.iter().map(|x| x.to_string()).join(","));

	let debug2 = run_program2(562949953421313);
	println!("Debug 2: {}", debug2.iter().map(|x| x.to_string()).join(","));

	run_program4(&ops);

	// 2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0 (16 digits)
	// 105690555219968
	// 0x602000000000
	// 011'000'000'010'xxx'xxx'xxx'xxx'xxx'xxx'xxx'xxx'xxx'xxx'xxx'xxx

	for a in 105690555219968.. {
	// 	// regs[0] = a;
	// 	// let result = run_program2(a);
	// 	// if result == ops {
	// 	// 	println!("Part 2: {}", a);
	// 	// 	break;
	// 	// }
		if run_program3(a, &ops) {
			println!("Part 2: {}", a);
			break;
		}
		if a % 1000000000 == 0 {
			println!(
				"Trying {}:",
				a,
				// result.iter().map(|x| x.to_string()).join(",")
			);
		}
	}
}
