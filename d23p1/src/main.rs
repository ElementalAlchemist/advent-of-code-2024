use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let connections = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
		for line in input_string.lines() {
			let (one_computer, other_computer) =
				line.split_once('-').expect("Each line should have a pair of computers");
			connections
				.entry(one_computer.to_string())
				.or_default()
				.insert(other_computer.to_string());
			connections
				.entry(other_computer.to_string())
				.or_default()
				.insert(one_computer.to_string());
		}
		connections
	};

	let mut groups: HashSet<Vec<String>> = HashSet::new();
	for (computer, grouped_computers) in connections.iter().filter(|(computer, _)| computer.starts_with('t')) {
		for group_computer in grouped_computers.iter() {
			for other_group_computer in connections.get(group_computer).unwrap() {
				if grouped_computers.contains(other_group_computer) {
					let mut group = vec![
						computer.to_string(),
						group_computer.to_string(),
						other_group_computer.to_string(),
					];
					group.sort_unstable();
					groups.insert(group);
				}
			}
		}
	}

	println!("{}", groups.len());

	Ok(())
}
