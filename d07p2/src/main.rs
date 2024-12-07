use std::error::Error;
use std::fs;

struct EquationNumbers {
	result: u64,
	operands: Vec<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let equation_values = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut equations: Vec<EquationNumbers> = Vec::new();
		for line in input_string.lines() {
			let (result, operands) = line.split_once(": ").expect("Result-operand delimiter");
			let result: u64 = result.parse()?;
			let operands: Vec<u64> = operands
				.split(' ')
				.map(|operand| operand.parse().expect("Operand is a number"))
				.collect();
			equations.push(EquationNumbers { result, operands });
		}

		equations
	};

	let mut result_sum: u64 = 0;
	for equation in equation_values.iter() {
		let mut totals = Vec::new();
		for operand in equation.operands.iter() {
			if totals.is_empty() {
				totals.push(*operand);
			} else {
				let mut new_totals = Vec::new();
				for total in totals {
					new_totals.push(total + *operand);
					new_totals.push(total * *operand);
					let concatenation = format!("{}{}", total, *operand);
					let concatenation: u64 = concatenation.parse().unwrap();
					new_totals.push(concatenation);
				}
				totals = new_totals;
			}
		}
		if totals.contains(&equation.result) {
			result_sum += equation.result;
		}
	}

	println!("{}", result_sum);

	Ok(())
}
