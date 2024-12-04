// #[macro_use] extern crate lazy_static;
extern crate clap;
extern crate emergence;
extern crate itertools;
extern crate num;
extern crate regex;

use clap::Parser;
use std::io::{self, BufRead};

mod day01;
mod day02;
mod day03;
mod day04;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// Reads puzzle input from the specified file
	#[arg(short, long)]
	file: Option<String>,

	/// Reads puzzle input from standard in
	#[arg(short, long)]
	stdin: bool,

	/// Specifies which day's challenge to run
	day: u32,
}

fn main() {
	let cli = Cli::parse();

	let input;
	if let Some(file_name) = cli.file {
		let contents = std::fs::read_to_string(file_name).expect("Can't read input file");
		input = contents.lines().map(String::from).collect();
	} else if cli.stdin {
		let stdin = io::stdin();
		input = stdin
			.lock()
			.lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else {
		let aoc_fetcher = emergence::AoC::new(2024).expect("Couldn't instantiate AoC object");
		let prob_input = aoc_fetcher
			.read_or_fetch(cli.day as usize)
			.expect("Couldn't fetch problem input");
		input = prob_input
			.trim_end_matches('\n')
			.split('\n')
			.map(String::from)
			.collect::<Vec<String>>();
	}

	match cli.day {
		1 => day01::solve(input),
		2 => day02::solve(input),
		3 => day03::solve(input),
		4 => day04::solve(input),

		_ => println!("Oops! Day {} isn't implemented yet!", cli.day),
	}
}
