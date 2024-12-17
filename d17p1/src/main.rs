use std::error::Error;
use std::fs;

fn combo_operand(operand: u8, register_a: u32, register_b: u32, register_c: u32) -> u32 {
	match operand {
		0..=3 => operand.into(),
		4 => register_a,
		5 => register_b,
		6 => register_c,
		_ => unimplemented!(),
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (mut register_a, mut register_b, mut register_c, instructions) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut lines_iter = input_string.lines();
		let a_line = lines_iter.next().expect("Register A line");
		let b_line = lines_iter.next().expect("Register B line");
		let c_line = lines_iter.next().expect("Register C line");

		let a_line = a_line.strip_prefix("Register A: ").expect("Register A line prefix");
		let b_line = b_line.strip_prefix("Register B: ").expect("Register B line prefix");
		let c_line = c_line.strip_prefix("Register C: ").expect("Register C line prefix");

		assert_eq!(lines_iter.next(), Some(""));

		let register_a = a_line.parse()?;
		let register_b = b_line.parse()?;
		let register_c = c_line.parse()?;

		let mut instructions: Vec<u8> = Vec::new();
		let program = lines_iter
			.next()
			.expect("Program line")
			.strip_prefix("Program: ")
			.expect("Program line prefix");
		for num in program.split(',') {
			instructions.push(num.parse()?);
		}
		(register_a, register_b, register_c, instructions)
	};

	let mut instruction_ptr: usize = 0;
	let mut output: Vec<u8> = Vec::new();

	while instruction_ptr < instructions.len() {
		let opcode = instructions[instruction_ptr];
		let operand = instructions[instruction_ptr + 1];
		let mut jump = false;
		match opcode {
			0 => {
				// adv
				let operand = combo_operand(operand, register_a, register_b, register_c);
				register_a /= 2u32.pow(operand);
			}
			1 => {
				// bxl
				let operand: u32 = operand.into();
				register_b ^= operand;
			}
			2 => {
				// bst
				let operand = combo_operand(operand, register_a, register_b, register_c);
				register_b = operand % 8;
			}
			3 => {
				// jnz
				if register_a != 0 {
					instruction_ptr = operand.into();
					jump = true;
				}
			}
			4 => {
				// bxc
				register_b ^= register_c;
			}
			5 => {
				// out
				let operand = combo_operand(operand, register_a, register_b, register_c);
				output.push((operand % 8).try_into().unwrap());
			}
			6 => {
				// bdv
				let operand = combo_operand(operand, register_a, register_b, register_c);
				register_b = register_a / 2u32.pow(operand);
			}
			7 => {
				// cdv
				let operand = combo_operand(operand, register_a, register_b, register_c);
				register_c = register_a / 2u32.pow(operand);
			}
			_ => unimplemented!(),
		}
		if !jump {
			instruction_ptr += 2;
		}
	}

	let output_strs: Vec<String> = output.into_iter().map(|x| x.to_string()).collect();
	let output_string = output_strs.join(",");
	println!("{}", output_string);

	Ok(())
}
