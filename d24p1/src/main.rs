use std::collections::HashMap;
use std::error::Error;
use std::fs;

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

struct Gate {
	left_input: String,
	right_input: String,
	kind: GateType,
	output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (mut lines, mut gates) = {
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

	while gates.iter().any(|gate| gate.output.starts_with('z')) {
		let mut unfulfilled_gates = Vec::new();
		for gate in gates {
			if let (Some(left_line), Some(right_line)) = (
				lines.get(&gate.left_input).copied(),
				lines.get(&gate.right_input).copied(),
			) {
				lines.insert(gate.output, gate.kind.evaluate(left_line, right_line));
			} else {
				unfulfilled_gates.push(gate);
			}
		}
		gates = unfulfilled_gates;
	}

	let mut z: u64 = 0;
	for line in lines.into_iter().filter(|(_, value)| *value).map(|(name, _)| name) {
		if let Some(index) = line.strip_prefix('z') {
			let index: u64 = index.parse()?;
			let value = 1 << index;
			z += value;
		}
	}

	println!("{}", z);

	Ok(())
}
