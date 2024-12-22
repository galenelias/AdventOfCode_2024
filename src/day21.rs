use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const NUM_KEYPAD: [[char; 3]; 4] = [
	['7', '8', '9'],
	['4', '5', '6'],
	['1', '2', '3'],
	['#', '0', 'A'],
];

const DIR_KEYPAD: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
	pos: (usize, usize),
	offset: usize,
	seq: Vec<char>,
	cost: usize,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost.cmp(&(other.cost)).reverse()
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn key_to_pos(key: char) -> (usize, usize) {
	for (r, row) in DIR_KEYPAD.iter().enumerate() {
		for (c, &ch) in row.iter().enumerate() {
			if ch == key {
				return (r, c);
			}
		}
	}
	unreachable!("Invalid key: {}", key);
}

fn numpad_seq_len(moves: &[char]) -> usize {
	let mut queue = BinaryHeap::new();

	queue.push(State {
		pos: (3, 2),
		offset: 0,
		seq: vec![],
		cost: 0,
	});

	let mut visited = HashSet::new();
	while !queue.is_empty() {
		let state = queue.pop().unwrap();
		let (pos, cost) = (state.pos, state.cost);

		println!("State: {:?}", state);

		// let prev_letter = if state.offset == 0 { outer_prev_letter } else { moves[state.offset - 1] };
		let prev_letter = if state.seq.is_empty() { 'A' } else { *state.seq.last().unwrap() };
		// let prev_letter = 'A';
		// if state.offset > 0 {
		// 	println!("Prev letter: {}", prev_letter);
		// }

		if state.offset == moves.len() {
			// if layer == 2 {
			println!("NumPad: {} = {} ({})", moves.iter().join(""), state.seq.iter().join(""), cost);
			// }
			// memo.insert((layer, outer_prev_letter, moves.iter().cloned().collect_vec()), cost);
			return cost;
		}

		if NUM_KEYPAD[pos.0][pos.1] == '#' {
			continue;
		} else if NUM_KEYPAD[pos.0][pos.1] == moves[state.offset] {
			// Push button
			let mut memo: HashMap<(usize, char, Vec<char>), usize> = HashMap::new();
			queue.push(State {
				pos: pos,
				offset: state.offset + 1,
				seq: state.seq.iter().cloned().chain(['A']).collect(),
				cost: cost + seq_len(&['A'], 2, prev_letter, &mut memo),
			});
		} else {
			if !visited.insert((pos, state.offset)) {
				continue;
			}

			if pos.0 > 0 {
				let mut memo: HashMap<(usize, char, Vec<char>), usize> = HashMap::new();
				queue.push(State {
					pos: (pos.0 - 1, pos.1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['^']).collect(),
					cost: cost + seq_len(&['^'], 2, prev_letter, &mut memo),
				});
			}
			if pos.0 < NUM_KEYPAD.len() - 1 {
				let mut memo: HashMap<(usize, char, Vec<char>), usize> = HashMap::new();
				queue.push(State {
					pos: (pos.0 + 1, pos.1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['v']).collect(),
					cost: cost + seq_len(&['v'], 2, prev_letter, &mut memo),
				});
			}
			if pos.1 > 0 {
				let mut memo: HashMap<(usize, char, Vec<char>), usize> = HashMap::new();
				queue.push(State {
					pos: (pos.0, pos.1 - 1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['<']).collect(),
					cost: cost + seq_len(&['<'], 2, prev_letter, &mut memo),
				});
			}
			if pos.1 < NUM_KEYPAD[0].len() - 1 {
				let mut memo: HashMap<(usize, char, Vec<char>), usize> = HashMap::new();
				queue.push(State {
					pos: (pos.0, pos.1 + 1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['>']).collect(),
					cost: cost + seq_len(&['>'], 2, prev_letter, &mut memo),
				});
			}
		}
	}

	unreachable!()
}

fn seq_len(moves: &[char], layer: usize, outer_prev_letter: char, memo: &mut HashMap<(usize, char, Vec<char>), usize>) -> usize {
	if layer == 0 {
		return moves.len();
	}

	if let Some(result) = memo.get(&(layer, outer_prev_letter, moves.iter().cloned().collect_vec())) {
		// println!("Memo'd {} -> {} = {}", prev_letter, moves.iter().join(""), result);
		return *result;
	}

	// println!("Call: layer {}, outer_prev_letter = {}, moves = {}", layer, outer_prev_letter, moves.iter().join(""));

	let mut queue = BinaryHeap::new();

	queue.push(State {
		pos: key_to_pos(outer_prev_letter),
		offset: 0,
		seq: vec![],
		cost: 0,
	});

	let mut visited = HashSet::new();
	while !queue.is_empty() {
		let state = queue.pop().unwrap();
		let (pos, cost) = (state.pos, state.cost);

		// let prev_letter = if state.offset == 0 { outer_prev_letter } else { moves[state.offset - 1] };
		let prev_letter = if state.seq.is_empty() { outer_prev_letter } else { *state.seq.last().unwrap() };
		// if state.offset > 0 {
		// 	println!("Prev letter: {}", prev_letter);
		// }

		if state.offset == moves.len() {
			// if layer == 1 {
				println!("Layer {}: {} -> {} = {} ({})", layer, outer_prev_letter, moves.iter().join(""), state.seq.iter().join(""), cost);
			// }
			// memo.insert((layer, outer_prev_letter, moves.iter().cloned().collect_vec()), cost);
			return cost;
		}

		if DIR_KEYPAD[pos.0][pos.1] == '#' {
			continue;
		} else if DIR_KEYPAD[pos.0][pos.1] == moves[state.offset] {
			// Push button
			queue.push(State {
				pos: pos,
				offset: state.offset + 1,
				seq: state.seq.iter().cloned().chain(['A']).collect(),
				cost: cost + seq_len(&['A'], layer - 1, prev_letter, memo),
			});
		} else {

			if !visited.insert((pos, state.offset)) {
				continue;
			}

			if pos.0 > 0 {
				queue.push(State {
					pos: (pos.0 - 1, pos.1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['^']).collect(),
					cost: cost + seq_len(&['^'], layer - 1, prev_letter, memo),
				});
			}
			if pos.0 < DIR_KEYPAD.len() - 1 {
				queue.push(State {
					pos: (pos.0 + 1, pos.1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['v']).collect(),
					cost: cost + seq_len(&['v'], layer - 1, prev_letter, memo),
				});
			}
			if pos.1 > 0 {
				queue.push(State {
					pos: (pos.0, pos.1 - 1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['<']).collect(),
					cost: cost + seq_len(&['<'], layer - 1, prev_letter, memo),
				});
			}
			if pos.1 < DIR_KEYPAD[0].len() - 1 {
				queue.push(State {
					pos: (pos.0, pos.1 + 1),
					offset: state.offset,
					seq: state.seq.iter().cloned().chain(['>']).collect(),
					cost: cost + seq_len(&['>'], layer - 1, prev_letter, memo),
				});
			}
		}
	}

	unreachable!()
}

// fn get_sequence(input: &str) -> Vec<char> {
// 	input.chars().collect()
// }

pub fn solve(_inputs: Vec<String>) {
	// let temp = numpad_seq_len("029A".chars().collect_vec().as_slice());
	let temp = numpad_seq_len("0".chars().collect_vec().as_slice());
	// let temp = seq_len("v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect_vec().as_slice(), 1, 'A', &mut memo);
	// let mut memo = HashMap::new();
	// let temp = seq_len("<A^A^^>AvvvA".chars().collect_vec().as_slice(), 2, 'A', &mut memo);
	// let temp = seq_len("<A^A>^^AvvvA".chars().collect_vec().as_slice(), 2, 'A', &mut memo);
	// let temp = seq_len("<".chars().collect_vec().as_slice(), 2, 'A', &mut memo);
	// let temp = seq_len("v<<A>>^A".chars().collect_vec().as_slice(), 1, 'A', &mut memo);
	// let temp = seq_len(&['^', '^', '<', 'A'], 3);
	// let temp = seq_len(&['^'], 3);
	println!("{}", temp);
	// let mut part1 = 0;
	// for input in inputs {
	// 	let num = input[0..3].parse::<usize>().unwrap();
	// 	let sequence = get_sequence(&input);
	// 	println!("{}, {}", num, sequence.len());
	// 	part1 += num * sequence.len();
	// }

	// println!("Part 1: {}", part1);
}
