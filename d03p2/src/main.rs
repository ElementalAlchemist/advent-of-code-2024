use std::error::Error;
use std::fs;

enum TrackingState {
	NoInstruction,
	MulM,
	MulU,
	MulL,
	MulOpenParen,
	FirstNumber(u32),
	Separator(u32),
	SecondNumber(u32, u32),
	MulCloseParen(u32, u32),
	EnableD,
	EnableO,
	EnableN,
	EnableApos,
	EnableT,
	EnableOpenParen,
	EnableCloseParen,
	DisableOpenParen,
	DisableCloseParen,
}

impl TrackingState {
	fn new() -> Self {
		Self::NoInstruction
	}

	fn advance_state(&mut self, next_char: char) {
		*self = match self {
			Self::NoInstruction => Self::first_instruction(next_char),
			Self::MulM => {
				if next_char == 'u' {
					Self::MulU
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::MulU => {
				if next_char == 'l' {
					Self::MulL
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::MulL => {
				if next_char == '(' {
					Self::MulOpenParen
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::MulOpenParen => {
				if next_char.is_ascii_digit() {
					Self::FirstNumber((next_char as u32) - 48)
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::FirstNumber(first_num) => {
				if next_char.is_ascii_digit() {
					let first_num_next_digit = (next_char as u32) - 48;
					let new_first_num = *first_num * 10 + first_num_next_digit;
					Self::FirstNumber(new_first_num)
				} else if next_char == ',' {
					Self::Separator(*first_num)
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::Separator(first_num) => {
				if next_char.is_ascii_digit() {
					Self::SecondNumber(*first_num, (next_char as u32) - 48)
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::SecondNumber(first_num, second_num) => {
				if next_char.is_ascii_digit() {
					let second_num_next_digit = (next_char as u32) - 48;
					let new_second_num = *second_num * 10 + second_num_next_digit;
					Self::SecondNumber(*first_num, new_second_num)
				} else if next_char == ')' {
					Self::MulCloseParen(*first_num, *second_num)
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::MulCloseParen(_, _) => Self::first_instruction(next_char),
			Self::EnableD => {
				if next_char == 'o' {
					Self::EnableO
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::EnableO => {
				if next_char == '(' {
					Self::EnableOpenParen
				} else if next_char == 'n' {
					Self::EnableN
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::EnableN => {
				if next_char == '\'' {
					Self::EnableApos
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::EnableApos => {
				if next_char == 't' {
					Self::EnableT
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::EnableT => {
				if next_char == '(' {
					Self::DisableOpenParen
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::EnableOpenParen => {
				if next_char == ')' {
					Self::EnableCloseParen
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::EnableCloseParen => Self::first_instruction(next_char),
			Self::DisableOpenParen => {
				if next_char == ')' {
					Self::DisableCloseParen
				} else {
					Self::first_instruction(next_char)
				}
			}
			Self::DisableCloseParen => Self::first_instruction(next_char),
		}
	}

	fn first_instruction(next_char: char) -> Self {
		match next_char {
			'm' => Self::MulM,
			'd' => Self::EnableD,
			_ => Self::NoInstruction,
		}
	}

	fn operation_result(&self) -> Option<u32> {
		match self {
			Self::MulCloseParen(first_num, second_num) => Some(*first_num * *second_num),
			_ => None,
		}
	}

	fn operation_applies_value(&self) -> Option<bool> {
		match self {
			Self::EnableCloseParen => Some(true),
			Self::DisableCloseParen => Some(false),
			_ => None,
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let input_string = fs::read_to_string("input.txt")?;

	let mut state = TrackingState::new();
	let mut product_sum = 0;
	let mut apply_operator = true;
	for c in input_string.chars() {
		state.advance_state(c);
		if let Some(apply) = state.operation_applies_value() {
			apply_operator = apply;
		}
		if apply_operator {
			if let Some(product) = state.operation_result() {
				product_sum += product;
			}
		}
	}

	println!("{}", product_sum);

	Ok(())
}
