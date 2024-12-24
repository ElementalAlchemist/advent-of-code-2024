use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::Write;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GateType {
	And,
	Or,
	Xor,
}

impl GateType {
	fn evaluate(&self, lhs: bool, rhs: bool) -> bool {
		match self {
			GateType::And => lhs && rhs,
			GateType::Or => lhs || rhs,
			GateType::Xor => lhs ^ rhs,
		}
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Gate {
	left_input: String,
	right_input: String,
	kind: GateType,
	output: String,
}

fn evaluate(initial_lines: &HashMap<String, bool>, gates: &[Gate], swaps: &HashMap<&str, &str>) -> u64 {
	let mut lines = initial_lines.clone();
	let mut unresolved_gates: Vec<Gate> = gates.to_vec();
	while unresolved_gates.iter().any(|gate| gate.output.starts_with('z')) {
		let mut new_unresolved_gates = Vec::new();
		for gate in unresolved_gates {
			if let (Some(left_line), Some(right_line)) = (
				lines.get(&gate.left_input).copied(),
				lines.get(&gate.right_input).copied(),
			) {
				if let Some(new_output) = swaps.get(&gate.output.as_str()) {
					lines.insert(new_output.to_string(), gate.kind.evaluate(left_line, right_line));
				} else {
					lines.insert(gate.output, gate.kind.evaluate(left_line, right_line));
				}
			} else {
				new_unresolved_gates.push(gate);
			}
		}
		unresolved_gates = new_unresolved_gates;
	}

	let mut z: u64 = 0;
	for line in lines.iter().filter(|(_, value)| **value).map(|(name, _)| name) {
		if let Some(index) = line.strip_prefix('z') {
			let index: u64 = index.parse().unwrap();
			let value = 1 << index;
			z += value;
		}
	}

	z
}

fn wrong_outputs(goal_number: u64, z: u64) -> (HashSet<String>, HashSet<String>) {
	let mut z_output_should_be_true: HashSet<String> = HashSet::new();
	let mut z_output_should_be_false: HashSet<String> = HashSet::new();
	let mut rem_z = z;
	let mut rem_goal = goal_number;
	let mut rem_index = 0;
	while rem_z > 0 || rem_goal > 0 {
		if rem_z % 2 == 0 && rem_goal % 2 != 0 {
			z_output_should_be_true.insert(format!("z{:02}", rem_index));
		} else if rem_z % 2 != 0 && rem_goal % 2 == 0 {
			z_output_should_be_false.insert(format!("z{:02}", rem_index));
		}
		rem_z >>= 1;
		rem_goal >>= 1;
		rem_index += 1;
	}

	(z_output_should_be_false, z_output_should_be_true)
}

fn write_graph_file(file_name: &str, gates: &[Gate]) -> std::io::Result<()> {
	let mut graph_line_names: HashMap<String, String> = HashMap::new();
	for gate in gates.iter() {
		let output_name = match gate.kind {
			GateType::And => format!("{}_and", gate.output),
			GateType::Or => format!("{}_or", gate.output),
			GateType::Xor => format!("{}_xor", gate.output),
		};
		graph_line_names.insert(gate.output.clone(), output_name);
	}

	let mut graph_file = fs::File::create(file_name)?;
	graph_file.write_all(b"digraph G {\n")?;
	for gate in gates.iter() {
		let left_input = graph_line_names
			.get(&gate.left_input)
			.cloned()
			.unwrap_or_else(|| gate.left_input.clone());
		let right_input = graph_line_names
			.get(&gate.right_input)
			.cloned()
			.unwrap_or_else(|| gate.right_input.clone());
		let output = graph_line_names
			.get(&gate.output)
			.cloned()
			.unwrap_or_else(|| gate.output.clone());

		let edge = format!(" {} -> {};\n {} -> {};\n", left_input, output, right_input, output);
		graph_file.write_all(edge.as_bytes())?;
	}
	graph_file.write_all(b"}")?;

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let (initial_lines, gates) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut lines: HashMap<String, bool> = HashMap::new();
		let mut gates: Vec<Gate> = Vec::new();
		let mut lines_iter = input_string.lines();
		for line in lines_iter.by_ref() {
			if line.is_empty() {
				break;
			}
			let (gate_name, value) = line.split_once(": ").expect("Line should have gate and value");
			let value = match value {
				"1" => true,
				"0" => false,
				_ => unimplemented!(),
			};
			lines.insert(gate_name.to_string(), value);
		}
		for line in lines_iter {
			let (input_part, output) = line
				.split_once(" -> ")
				.expect("Both input and output for gate should be present");
			let mut input_iter = input_part.split(' ');
			let left_gate = input_iter.next().unwrap();
			let kind = input_iter.next().unwrap();
			let right_gate = input_iter.next().unwrap();
			assert!(input_iter.next().is_none());
			let kind = match kind {
				"AND" => GateType::And,
				"OR" => GateType::Or,
				"XOR" => GateType::Xor,
				_ => unimplemented!(),
			};
			gates.push(Gate {
				left_input: left_gate.to_string(),
				kind,
				right_input: right_gate.to_string(),
				output: output.to_string(),
			});
		}

		(lines, gates)
	};

	let mut x: u64 = 0;
	let mut y: u64 = 0;
	for line_name in initial_lines.iter().filter(|(_, value)| **value).map(|(name, _)| name) {
		if let Some(index) = line_name.strip_prefix('x') {
			let index: u64 = index.parse()?;
			x += 1 << index;
		} else if let Some(index) = line_name.strip_prefix('y') {
			let index: u64 = index.parse()?;
			y += 1 << index;
		}
	}

	let goal_number = x + y;

	let z = evaluate(&initial_lines, &gates, &HashMap::new());

	let mut gates_by_input: HashMap<String, Vec<Gate>> = HashMap::new();
	for gate in gates.iter() {
		gates_by_input
			.entry(gate.left_input.clone())
			.or_default()
			.push(gate.clone());
		gates_by_input
			.entry(gate.right_input.clone())
			.or_default()
			.push(gate.clone());
	}

	let (z_output_should_be_false, z_output_should_be_true) = wrong_outputs(goal_number, z);

	println!("{:b}", goal_number);
	println!("{:b}", z);
	println!("{:?} {:?}", z_output_should_be_false, z_output_should_be_true);

	let mut z_output_wrong: Vec<String> = z_output_should_be_true
		.union(&z_output_should_be_false)
		.cloned()
		.collect();
	z_output_wrong.sort_unstable();
	println!("{:?}", z_output_wrong);

	write_graph_file("graph.dot", &gates)?;

	println!();

	let mut swaps: HashMap<&str, &str> = HashMap::new();
	swaps.insert("z16", "qkf");
	swaps.insert("qkf", "z16");
	swaps.insert("z24", "tgr");
	swaps.insert("tgr", "z24");
	swaps.insert("jqn", "cph");
	swaps.insert("cph", "jqn");
	swaps.insert("z12", "kwb");
	swaps.insert("kwb", "z12");

	let z = evaluate(&initial_lines, &gates, &swaps);
	println!("{:b}", goal_number);
	println!("{:b}", z);

	let (z_output_should_be_false, z_output_should_be_true) = wrong_outputs(goal_number, z);
	println!("{:?} {:?}", z_output_should_be_false, z_output_should_be_true);

	let mut z_output_wrong: Vec<String> = z_output_should_be_false
		.union(&z_output_should_be_true)
		.cloned()
		.collect();
	z_output_wrong.sort_unstable();
	println!("{:?}", z_output_wrong);

	let mut swapped_lines: Vec<String> = swaps.keys().map(|line| line.to_string()).collect();
	swapped_lines.sort_unstable();
	println!("{}", swapped_lines.join(","));

	let mut fixed_gates = gates.clone();
	for gate in fixed_gates.iter_mut() {
		if let Some(real_output) = swaps.get(&gate.output.as_str()) {
			gate.output = real_output.to_string();
		}
	}
	write_graph_file("graph-fixed.dot", &fixed_gates)?;

	Ok(())
}
