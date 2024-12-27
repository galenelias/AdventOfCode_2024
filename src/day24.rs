use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Operation {
	And,
	Or,
	Xor,
}

#[derive(Debug, Clone)]
struct Gate {
	input1: String,
	input2: String,
	operation: Operation,
	output: String,
}

const MASK: u64 = (1 << NUM_DIGITS) - 1;

fn sub_solve(mut gates: Vec<Gate>, mut registers: HashMap<String, bool>) -> Option<u64> {
	while !gates.is_empty() {
		for gate in &gates {
			if let Some(in1) = registers.get(&gate.input1) {
				if let Some(in2) = registers.get(&gate.input2) {
					let result = match gate.operation {
						Operation::And => *in1 && *in2,
						Operation::Or => *in1 || *in2,
						Operation::Xor => *in1 ^ *in2,
					};

					registers.insert(gate.output.clone(), result);
				}
			}
		}

		let previous_len = gates.len();
		gates.retain(|gate| !registers.contains_key(&gate.output));

		// If we failed to make forward progress, this is an invalid circuit
		if gates.len() == previous_len {
			return None;
		}
	}

	let mut z_registers = registers
		.iter()
		.filter(|(k, _)| k.starts_with('z'))
		.collect_vec();
	z_registers.sort_by_key(|(k, _)| (*k).clone());

	let mut result: u64 = 0;
	for (i, reg) in z_registers.iter().enumerate() {
		if *reg.1 {
			result += 1 << i;
		}
	}

	Some(result)
}

const NUM_DIGITS: usize = 45;

fn validate_adder(operands: &[(u64, u64)], gates: &Vec<Gate>) -> u64 {
	let mut good_bits = MASK;
	for &(op1, op2) in operands {
		let expected = (op1 + op2) & MASK;

		if let Some(actual) = try_add(gates.clone(), op1, op2) {
			let mismatched_bits = expected ^ actual;
			good_bits &= !mismatched_bits;
		} else {
			// If our circuit is invalid, we can't trust any of the bits
			return 0;
		}
	}

	good_bits
}

fn try_add(gates: Vec<Gate>, op1: u64, op2: u64) -> Option<u64> {
	let mut registers = HashMap::new();
	for i in 0..NUM_DIGITS {
		registers.insert(format!("x{0:02}", i), (op1 & (1 << i)) != 0);
		registers.insert(format!("y{0:02}", i), (op2 & (1 << i)) != 0);
	}

	sub_solve(gates, registers)
}

pub fn solve(inputs: Vec<String>) {
	let (initial_values, gates) = inputs
		.split(|line| line.is_empty())
		.collect_tuple()
		.unwrap();

	let mut registers = HashMap::new();
	for line in initial_values {
		let (reg, val) = line.split_once(": ").unwrap();
		let val = val.parse::<i32>().unwrap() != 0;

		registers.insert(reg.to_string(), val);
	}

	let gates = gates
		.iter()
		.map(|line| {
			let (inputs, output) = line.split_once(" -> ").unwrap();
			let (input1, operation, input2) = inputs.split_whitespace().collect_tuple().unwrap();
			let operation = match operation {
				"AND" => Operation::And,
				"OR" => Operation::Or,
				"XOR" => Operation::Xor,
				_ => panic!("Unknown operation: {}", operation),
			};

			Gate {
				input1: input1.to_string(),
				input2: input2.to_string(),
				operation,
				output: output.to_string(),
			}
		})
		.collect_vec();

	let part1 = sub_solve(gates.clone(), registers.clone()).unwrap();
	println!("Part 1: {}", part1);

	let mut bad_gates = Vec::new();

	for gate in &gates {
		let consuming_gates = gates.iter().filter(|g| g.input1 == gate.output || g.input2 == gate.output).collect_vec();

		let source_xy = (gate.input1.starts_with('x') || gate.input1.starts_with('y'))
			&& (gate.input2.starts_with('x') || gate.input2.starts_with('y'));
		let dest_z = gate.output.starts_with('z');

		// The first adder is assumed to fine, and doesn't follow lots of the 'normal' rules, so ignore it
		if source_xy && gate.input1.ends_with("00") {
			continue;
		}

		if source_xy && gate.operation == Operation::Or {
			println!("Found bad gate (1): {:?}", gate); // Only AND and XOR are valid for gates processing and x and y input
			bad_gates.push(gate.output.clone());
			continue;
		}

		if dest_z && gate.output != "z45" && gate.operation != Operation::Xor {
			println!("Found bad gate (2): {:?}", gate); // Only XOR gates can drive z registers
			bad_gates.push(gate.output.clone());
			continue;
		}

		if gate.operation == Operation::And {
			if consuming_gates.len() != 1 || consuming_gates[0].operation != Operation::Or {
				println!("Found bad gate (3): {:?}", gate); // We should always be ORing the result of our AND gates
				bad_gates.push(gate.output.clone());
				continue;
			}
		}

		if source_xy && gate.operation == Operation::Xor {
			let consuming_xor = consuming_gates.iter().find(|g| g.operation == Operation::Xor);
			if consuming_xor.is_none() && gate.output != "z00" {
				println!("Found bad gate (5): {:?}", gate); // XOR gates must be consumed by another XOR gate
				bad_gates.push(gate.output.clone());
				continue;
			}
		}

		if !dest_z && !source_xy && gate.operation == Operation::Xor {
			println!("Found bad gate (6): {:?}", gate); // Gates that don't process input registers, or output to z registers, can't be XOR gates
			bad_gates.push(gate.output.clone());
			continue;
		}
	}

	bad_gates.sort();

	let mut random_operands = Vec::new();
	random_operands.push((0, 0));
	random_operands.push((MASK, MASK));
	random_operands.push((0, MASK));
	random_operands.push((MASK, 0));
	random_operands.push((
		0b01010101010101010101010101010101010101010101,
		0b0101010101010101010101010101010101010101,
	));

	for permutation in bad_gates.iter().permutations(8) {
		if permutation[0] > permutation[1] || permutation[2] > permutation[3] {
			continue;
		}

		let mut new_gates = gates.clone();
		for i in (0..permutation.len()).step_by(2) {
			for gate in &mut new_gates {
				if &gate.output == permutation[i] {
					gate.output = permutation[i + 1].clone();
				} else if &gate.output == permutation[i + 1] {
					gate.output = permutation[i].clone();
				}
			}
		}

		let new_good_bits = validate_adder(&random_operands, &new_gates);
		if new_good_bits == MASK {
			println!("Part 2: {}", bad_gates.join(","));
			break;
		}
	}
}
