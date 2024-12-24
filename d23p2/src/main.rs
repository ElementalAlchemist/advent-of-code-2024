use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn find_maximal_clique(connections: &HashMap<String, HashSet<String>>) -> HashSet<String> {
	let to_be_considered: HashSet<String> = connections.keys().cloned().collect();
	sub_find_maximal_clique(connections, &to_be_considered, &HashSet::new(), &HashSet::new()).unwrap()
}

fn sub_find_maximal_clique(
	connections: &HashMap<String, HashSet<String>>,
	to_be_considered: &HashSet<String>,
	in_group: &HashSet<String>,
	out_of_group: &HashSet<String>,
) -> Option<HashSet<String>> {
	if to_be_considered.is_empty() && out_of_group.is_empty() {
		return Some(in_group.clone());
	}
	let mut considering = to_be_considered.clone();
	let mut out_of_group = out_of_group.clone();
	let empty_set = HashSet::new();
	let mut biggest_clique: Option<HashSet<String>> = None;
	for vertex in to_be_considered.iter() {
		let mut vertex_in_group = in_group.clone();
		vertex_in_group.insert(vertex.clone());
		let vertex_connections = connections.get(vertex).unwrap_or(&empty_set);
		let vertex_considering: HashSet<String> = considering.intersection(vertex_connections).cloned().collect();
		let vertex_out_of_group: HashSet<String> = out_of_group.intersection(vertex_connections).cloned().collect();
		let max_clique =
			sub_find_maximal_clique(connections, &vertex_considering, &vertex_in_group, &vertex_out_of_group);
		if let Some(clique) = max_clique {
			match &mut biggest_clique {
				Some(old_biggest) => {
					if clique.len() > old_biggest.len() {
						*old_biggest = clique;
					}
				}
				None => biggest_clique = Some(clique),
			}
		}
		considering.remove(vertex);
		out_of_group.insert(vertex.clone());
	}
	biggest_clique
}

fn main() -> Result<(), Box<dyn Error>> {
	let connections = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
		for line in input_string.lines() {
			let (one_computer, other_computer) =
				line.split_once('-').expect("Each line should have a pair of computers");
			let one_connection = connections.entry(one_computer.to_string()).or_default();
			one_connection.insert(other_computer.to_string());
			let other_connection = connections.entry(other_computer.to_string()).or_default();
			other_connection.insert(one_computer.to_string());
		}
		connections
	};

	let biggest_group = find_maximal_clique(&connections);

	let mut biggest_group: Vec<String> = biggest_group.into_iter().collect();
	biggest_group.sort_unstable();
	let password = biggest_group.join(",");
	println!("{}", password);

	Ok(())
}
