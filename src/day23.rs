use itertools::Itertools;
use std::collections::HashMap;

// Fast and dirty implementation of Bron Kerbosch algorithm for finding maximal cliques
fn bron_kerbosch(r : Vec<usize>, mut p : Vec<usize>, mut x : Vec<usize>, adj: &Vec<Vec<bool>>, max_clique: &mut Vec<usize>) {
	if p.is_empty() && x.is_empty() {
		if r.len() > max_clique.len() {
			*max_clique = r;
		}
		return;
	}

	if p.is_empty() {
		return;
	}

	let p_clone = p.clone();
	let u = *p_clone.first().unwrap(); // pivot

	for v in p_clone {
		if u != v && adj[u][v] { // Skip neighbors of our pivot
			continue;
		}

		let mut r_prime = r.clone();
		let mut p_prime = p.clone();
		let mut x_prime = x.clone();
		r_prime.push(v);
		p_prime.retain(|&pv| adj[v][pv]);
		x_prime.retain(|&xv| adj[v][xv]);
		bron_kerbosch(r_prime, p_prime, x_prime, adj, max_clique);

		p.retain(|&pv| pv != v);
		x.push(v);
	}
}


pub fn solve(inputs: Vec<String>) {
	let mut computers = Vec::new();
	let mut computer_to_num = HashMap::new();

	let connections = inputs.iter().map(|line| line.split_once("-").unwrap()).collect_vec();

	for (c1, c2) in &connections {
		if !computer_to_num.contains_key(c1) {
			computer_to_num.insert(c1, computers.len());
			computers.push(c1);
		}
        if !computer_to_num.contains_key(c2) {
            computer_to_num.insert(c2, computers.len());
            computers.push(c2);
        }
	}

	let mut adjacent = vec![vec![false; computers.len()]; computers.len()];

	for (c1, c2) in &connections {
		let i = computer_to_num[c1];
		let j = computer_to_num[c2];
		adjacent[i][j] = true;
		adjacent[j][i] = true;
	}

	let mut part1 = 0;
	for i in 0..computers.len() {
		for j in i..computers.len() {
			if adjacent[i][j] {
				for k in j..computers.len() {
					if adjacent[i][k] && adjacent[j][k] {
						if computers[i].starts_with('t') || computers[j].starts_with('t') || computers[k].starts_with('t') {
							part1 += 1;
						}
					}
				}
			}
		}
	}
	println!("Part 1: {}", part1);

	let mut max_clique = Vec::new();
	bron_kerbosch(Vec::new(), (0..computers.len()).collect_vec(), Vec::new(), &adjacent, &mut max_clique);

	let max_clique_computers = max_clique.iter().map(|&i| computers[i].clone()).sorted().collect_vec();
	println!("Part 2: {}", max_clique_computers.join(","));
}