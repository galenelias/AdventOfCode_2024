use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use rand::prelude::*;

#[derive(Debug, Clone)]
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

fn propagate(gates: &Vec<Gate>, source: &str, dest: &str, gate_sources: &mut HashMap<String, HashSet<String>>) {
	for gate in gates.iter().filter(|gate| gate.input1 == dest || gate.input2 == dest) {
		gate_sources.entry(gate.output.clone()).or_insert(HashSet::new()).insert(source.to_string());
		propagate(gates, source, &gate.output, gate_sources);
	}
}

fn sub_solve(mut gates: Vec<Gate>, mut registers: HashMap<String, bool>) -> u64 {
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

	let mut result: u64 = 0;
	for (i, reg) in z_registers.iter().enumerate() {
		if *reg.1 {
			result += 1 << i;
		}
	}

	result
}

const NUM_DIGITS: usize = 45;

fn try_add(mut gates: Vec<Gate>, op1: u64, op2: u64) -> u64 {
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

	let gates = gates.iter().map(|line| {
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

	let part1 = sub_solve(gates.clone(), registers.clone());
	println!("Part 1: {}", part1);

	let mut gate_sources: HashMap<String, HashSet<String>> = HashMap::new();
	for start_reg in registers.keys() {
		propagate(&gates, start_reg, start_reg, &mut gate_sources);
	}

	for i in 0..45 {
		let z_reg = format!("z{0:02}", i);

		let inputs = gate_sources.get(&z_reg).unwrap().iter().sorted().cloned().collect_vec();
		let mut expected = Vec::new();
		for x in 0..=i {
			expected.push(format!("x{0:02}", x));
		}
		for y in 0..=i {
			expected.push(format!("y{0:02}", y));
		}

		if inputs != expected {
			println!("{}: actual = {:?}, expected = {:?}", z_reg, inputs, expected);
		}

		// for gate in gates.iter().filter(|gate| gate.output == z_reg) {
			// println!("{}: {:?}", z_reg, gate_sources.get(&z_reg).unwrap().iter().sorted().collect_vec());
		// }
	}

	const MASK: u64 = (1 << NUM_DIGITS) - 1;
	let mut good_bits: u64 = MASK;

	for _ in 0..10 {
		let op1 = rand::random::<u64>() & MASK;
		let op2 = rand::random::<u64>() & MASK;
		let expected = (op1 + op2) & MASK;

		let actual = try_add(gates.clone(), op1, op2);
		let mismatched_bits = expected ^ actual;
		good_bits &= !mismatched_bits;

		println!("Good bits: {:045b}", good_bits);
	}
}


