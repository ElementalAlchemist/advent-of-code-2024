use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let connections = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut connections: HashMap<String, BTreeSet<String>> = HashMap::new();
		for line in input_string.lines() {
			let (one_computer, other_computer) =
				line.split_once('-').expect("Each line should have a pair of computers");
			let one_connection = connections.entry(one_computer.to_string()).or_default();
			one_connection.insert(one_computer.to_string());
			one_connection.insert(other_computer.to_string());
			let other_connection = connections.entry(other_computer.to_string()).or_default();
			other_connection.insert(other_computer.to_string());
			other_connection.insert(one_computer.to_string());
		}
		connections
	};

	let mut groups: HashSet<BTreeSet<String>> = HashSet::new();
	for grouped_computers in connections.values() {
		let mut connection_group = BTreeSet::new();
		for computer in grouped_computers.iter() {
			if connection_group.is_subset(connections.get(computer).unwrap()) {
				connection_group.insert(computer.clone());
			}
		}
		groups.insert(connection_group);
	}

	let max_group_size = groups.iter().map(|group| group.len()).max().unwrap();
	let biggest_group = groups.iter().find(|group| group.len() == max_group_size).unwrap();
	let biggest_group: Vec<String> = biggest_group.iter().cloned().collect();
	let password = biggest_group.join(",");
	println!("{}", password);

	Ok(())
}
