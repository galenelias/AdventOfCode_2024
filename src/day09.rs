#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct File {
	start: usize,
	size: usize,
	id: usize,
}

impl File {
	fn end(&self) -> usize {
		self.start + self.size
	}
}

fn checksum(files: &Vec<File>) -> usize {
	let mut checksum = 0;
	for file in files {
		for i in 0..file.size {
			checksum += file.id * (file.start + i);
		}
	}
	checksum
}

fn part1(mut files: Vec<File>) {
	// Just try to move every file, rather than bother detecting when we're already defragged
	for _ in 0..files.len() {
		let mut last_file = files.pop().unwrap();

		// while loop, since we invalidate files.len() in the loop, so need the condition to re-evaluate 
		let mut i = 0;
		while i < files.len() {
			let gap = if i < files.len() - 1 {
				files[i+1].start - files[i].end()
			} else {
				usize::MAX
			};

			if gap > last_file.size {
				// If entire file fits, move it and process the next file
				last_file.start = files[i].end();
				files.insert(i+1, last_file);
				break;
			} else if gap > 0 {
				// Otherwise, split the file and move the first part and keep looping
				last_file.size -= gap;
				files.insert(i+1, File { start: files[i].end(), size: gap, id: last_file.id});
			}
			i += 1;
		}
	}

	println!("Part 1: {}", checksum(&files));
}

fn part2(mut files: Vec<File>) {
	for x in (0..files.len()).rev() {
	    let cur_file_pos = files.iter().position(|f| f.id == x).unwrap();
		let cur_file = &files[cur_file_pos];

	    // Find first gap which can fit the file
	    for i in 0..files.len() {
			if files[i].start >= cur_file.start {
				break;
			}

	        if i < files.len() - 1 {
	            if files[i+1].start - files[i].end() >= cur_file.size {
					let mut popped_file = files.remove(cur_file_pos);
	                popped_file.start = files[i].end();
	                files.insert(i+1, popped_file);
	                break;
	            }
	        } 
	    }
	}

	println!("Part 2: {}", checksum(&files));
}

pub fn solve(inputs: Vec<String>) {
	let mut files = Vec::new();
	let mut position = 0;

	for (i, c) in inputs[0].chars().enumerate() {
		let val = c.to_digit(10).unwrap() as usize;
		if i % 2 == 0 && val != 0 {
			files.push(File{ start: position, size: val, id: i / 2});
		}
		position += val;
	}

	part1(files.clone());
	part2(files);
}