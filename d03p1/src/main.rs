use std::error::Error;
use std::fs;

enum TrackingState {
	NoInstruction,
	MulM,
	MulU,
	MulL,
	OpenParen,
	FirstNumber(u32),
	Separator(u32),
	SecondNumber(u32, u32),
	CloseParen(u32, u32),
}

impl TrackingState {
	fn new() -> Self {
		Self::NoInstruction
	}

	fn advance_state(&mut self, next_char: char) {
		*self = match self {
			Self::NoInstruction => {
				if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::MulM => {
				if next_char == 'u' {
					Self::MulU
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::MulU => {
				if next_char == 'l' {
					Self::MulL
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::MulL => {
				if next_char == '(' {
					Self::OpenParen
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::OpenParen => {
				if next_char.is_ascii_digit() {
					Self::FirstNumber((next_char as u32) - 48)
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::FirstNumber(first_num) => {
				if next_char.is_ascii_digit() {
					let first_num_next_digit = (next_char as u32) - 48;
					let new_first_num = *first_num * 10 + first_num_next_digit;
					Self::FirstNumber(new_first_num)
				} else if next_char == ',' {
					Self::Separator(*first_num)
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::Separator(first_num) => {
				if next_char.is_ascii_digit() {
					Self::SecondNumber(*first_num, (next_char as u32) - 48)
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::SecondNumber(first_num, second_num) => {
				if next_char.is_ascii_digit() {
					let second_num_next_digit = (next_char as u32) - 48;
					let new_second_num = *second_num * 10 + second_num_next_digit;
					Self::SecondNumber(*first_num, new_second_num)
				} else if next_char == ')' {
					Self::CloseParen(*first_num, *second_num)
				} else if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
			Self::CloseParen(_, _) => {
				if next_char == 'm' {
					Self::MulM
				} else {
					Self::NoInstruction
				}
			}
		}
	}

	fn operation_result(&self) -> Option<u32> {
		match self {
			Self::CloseParen(first_num, second_num) => Some(*first_num * *second_num),
			_ => None,
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let input_string = fs::read_to_string("input.txt")?;

	let mut state = TrackingState::new();
	let mut product_sum = 0;
	for c in input_string.chars() {
		state.advance_state(c);
		if let Some(product) = state.operation_result() {
			product_sum += product;
		}
	}

	println!("{}", product_sum);

	Ok(())
}
