use itertools::Itertools;
use std::collections::HashMap;

enum Operation {
	And,
	Or,
	Xor,
}

struct Gate {
	input1: String,
	input2: String,
	operation: Operation,
	output: String,
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

	let mut gates = gates.iter().map(|line| {
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
	}).collect_vec();

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

		gates.retain(|gate| !registers.contains_key(&gate.output));
	}

	let mut z_registers = registers.iter().filter(|(k, _)| k.starts_with('z')).collect_vec();
	z_registers.sort_by_key(|(k, _)| (*k).clone());

	println!("Part 1: ");
	let mut part1: u64 = 0;
	for (i, reg) in z_registers.iter().enumerate() {
		if *reg.1 {
			part1 += 1 << i;
		}
		print!("{}", if *reg.1 { '1' } else { '0' });
	}
	println!(" ({})", part1);


}


