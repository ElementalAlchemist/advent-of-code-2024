use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum Input {
	Up,
	Right,
	Down,
	Left,
	Activate,
}

impl Input {
	fn press_to_reach(&self, other: Self) -> Vec<Self> {
		match self {
			Self::Up => match other {
				Self::Up => vec![Self::Activate],
				Self::Activate => vec![Self::Right, Self::Activate],
				Self::Left => vec![Self::Down, Self::Left, Self::Activate],
				Self::Down => vec![Self::Down, Self::Activate],
				Self::Right => vec![Self::Down, Self::Right, Self::Activate],
			},
			Self::Activate => match other {
				Self::Up => vec![Self::Left, Self::Activate],
				Self::Activate => vec![Self::Activate],
				Self::Left => vec![Self::Down, Self::Left, Self::Left, Self::Activate],
				Self::Down => vec![Self::Down, Self::Left, Self::Activate],
				Self::Right => vec![Self::Down, Self::Activate],
			},
			Self::Left => match other {
				Self::Up => vec![Self::Right, Self::Up, Self::Activate],
				Self::Activate => vec![Self::Right, Self::Right, Self::Up, Self::Activate],
				Self::Left => vec![Self::Activate],
				Self::Down => vec![Self::Right, Self::Activate],
				Self::Right => vec![Self::Right, Self::Right, Self::Activate],
			},
			Self::Down => match other {
				Self::Up => vec![Self::Up, Self::Activate],
				Self::Activate => vec![Self::Up, Self::Right, Self::Activate],
				Self::Left => vec![Self::Left, Self::Activate],
				Self::Down => vec![Self::Activate],
				Self::Right => vec![Self::Right, Self::Activate],
			},
			Self::Right => match other {
				Self::Up => vec![Self::Up, Self::Left, Self::Activate],
				Self::Activate => vec![Self::Up, Self::Activate],
				Self::Left => vec![Self::Left, Self::Left, Self::Activate],
				Self::Down => vec![Self::Left, Self::Activate],
				Self::Right => vec![Self::Activate],
			},
		}
	}
}

#[derive(Clone, Copy)]
enum NumericKey {
	Zero,
	Activate,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
}

impl NumericKey {
	fn press_to_reach(&self, other: Self) -> Vec<Input> {
		match self {
			Self::Zero => match other {
				Self::Zero => vec![Input::Activate],
				Self::Activate => vec![Input::Right, Input::Activate],
				Self::One => vec![Input::Up, Input::Left, Input::Activate],
				Self::Two => vec![Input::Up, Input::Activate],
				Self::Three => vec![Input::Up, Input::Right, Input::Activate],
				Self::Four => vec![Input::Up, Input::Up, Input::Left, Input::Activate],
				Self::Five => vec![Input::Up, Input::Up, Input::Activate],
				Self::Six => vec![Input::Up, Input::Up, Input::Right, Input::Activate],
				Self::Seven => vec![Input::Up, Input::Up, Input::Up, Input::Left, Input::Activate],
				Self::Eight => vec![Input::Up, Input::Up, Input::Up, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Up, Input::Up, Input::Right, Input::Activate],
			},
			Self::Activate => match other {
				Self::Zero => vec![Input::Left, Input::Activate],
				Self::Activate => vec![Input::Activate],
				Self::One => vec![Input::Up, Input::Left, Input::Left, Input::Activate],
				Self::Two => vec![Input::Left, Input::Up, Input::Activate],
				Self::Three => vec![Input::Up, Input::Activate],
				Self::Four => vec![Input::Up, Input::Up, Input::Left, Input::Left, Input::Activate],
				Self::Five => vec![Input::Left, Input::Up, Input::Up, Input::Activate],
				Self::Six => vec![Input::Up, Input::Up, Input::Activate],
				Self::Seven => vec![
					Input::Up,
					Input::Up,
					Input::Up,
					Input::Left,
					Input::Left,
					Input::Activate,
				],
				Self::Eight => vec![Input::Left, Input::Up, Input::Up, Input::Up, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Up, Input::Up, Input::Activate],
			},
			Self::One => match other {
				Self::Zero => vec![Input::Right, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Right, Input::Right, Input::Down, Input::Activate],
				Self::One => vec![Input::Activate],
				Self::Two => vec![Input::Right, Input::Activate],
				Self::Three => vec![Input::Right, Input::Right, Input::Activate],
				Self::Four => vec![Input::Up, Input::Activate],
				Self::Five => vec![Input::Up, Input::Right, Input::Activate],
				Self::Six => vec![Input::Up, Input::Right, Input::Right, Input::Activate],
				Self::Seven => vec![Input::Up, Input::Up, Input::Activate],
				Self::Eight => vec![Input::Up, Input::Up, Input::Right, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Up, Input::Right, Input::Right, Input::Activate],
			},
			Self::Two => match other {
				Self::Zero => vec![Input::Down, Input::Activate],
				Self::Activate => vec![Input::Down, Input::Right, Input::Activate],
				Self::One => vec![Input::Left, Input::Activate],
				Self::Two => vec![Input::Activate],
				Self::Three => vec![Input::Right, Input::Activate],
				Self::Four => vec![Input::Left, Input::Up, Input::Activate],
				Self::Five => vec![Input::Up, Input::Activate],
				Self::Six => vec![Input::Up, Input::Right, Input::Activate],
				Self::Seven => vec![Input::Left, Input::Up, Input::Up, Input::Activate],
				Self::Eight => vec![Input::Up, Input::Up, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Up, Input::Right, Input::Activate],
			},
			Self::Three => match other {
				Self::Zero => vec![Input::Left, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Down, Input::Activate],
				Self::One => vec![Input::Left, Input::Left, Input::Activate],
				Self::Two => vec![Input::Left, Input::Activate],
				Self::Three => vec![Input::Activate],
				Self::Four => vec![Input::Left, Input::Left, Input::Up, Input::Activate],
				Self::Five => vec![Input::Left, Input::Up, Input::Activate],
				Self::Six => vec![Input::Up, Input::Activate],
				Self::Seven => vec![Input::Left, Input::Left, Input::Up, Input::Up, Input::Activate],
				Self::Eight => vec![Input::Left, Input::Up, Input::Up, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Up, Input::Activate],
			},
			Self::Four => match other {
				Self::Zero => vec![Input::Right, Input::Down, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Right, Input::Right, Input::Down, Input::Down, Input::Activate],
				Self::One => vec![Input::Down, Input::Activate],
				Self::Two => vec![Input::Right, Input::Down, Input::Activate],
				Self::Three => vec![Input::Right, Input::Right, Input::Down, Input::Activate],
				Self::Four => vec![Input::Activate],
				Self::Five => vec![Input::Right, Input::Activate],
				Self::Six => vec![Input::Right, Input::Right, Input::Activate],
				Self::Seven => vec![Input::Up, Input::Activate],
				Self::Eight => vec![Input::Up, Input::Right, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Right, Input::Right, Input::Activate],
			},
			Self::Five => match other {
				Self::Zero => vec![Input::Down, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Down, Input::Down, Input::Right, Input::Activate],
				Self::One => vec![Input::Left, Input::Down, Input::Activate],
				Self::Two => vec![Input::Down, Input::Activate],
				Self::Three => vec![Input::Down, Input::Right, Input::Activate],
				Self::Four => vec![Input::Left, Input::Activate],
				Self::Five => vec![Input::Activate],
				Self::Six => vec![Input::Right, Input::Activate],
				Self::Seven => vec![Input::Left, Input::Up, Input::Activate],
				Self::Eight => vec![Input::Up, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Activate],
			},
			Self::Six => match other {
				Self::Zero => vec![Input::Left, Input::Down, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Down, Input::Down, Input::Activate],
				Self::One => vec![Input::Left, Input::Left, Input::Down, Input::Activate],
				Self::Two => vec![Input::Left, Input::Down, Input::Activate],
				Self::Three => vec![Input::Down, Input::Activate],
				Self::Four => vec![Input::Left, Input::Left, Input::Activate],
				Self::Five => vec![Input::Left, Input::Activate],
				Self::Six => vec![Input::Activate],
				Self::Seven => vec![Input::Left, Input::Left, Input::Up, Input::Activate],
				Self::Eight => vec![Input::Left, Input::Up, Input::Activate],
				Self::Nine => vec![Input::Up, Input::Activate],
			},
			Self::Seven => match other {
				Self::Zero => vec![Input::Right, Input::Down, Input::Down, Input::Down, Input::Activate],
				Self::Activate => vec![
					Input::Right,
					Input::Right,
					Input::Down,
					Input::Down,
					Input::Down,
					Input::Activate,
				],
				Self::One => vec![Input::Down, Input::Down, Input::Activate],
				Self::Two => vec![Input::Right, Input::Down, Input::Down, Input::Activate],
				Self::Three => vec![Input::Right, Input::Right, Input::Down, Input::Down, Input::Activate],
				Self::Four => vec![Input::Down, Input::Activate],
				Self::Five => vec![Input::Right, Input::Down, Input::Activate],
				Self::Six => vec![Input::Right, Input::Right, Input::Down, Input::Activate],
				Self::Seven => vec![Input::Activate],
				Self::Eight => vec![Input::Right, Input::Activate],
				Self::Nine => vec![Input::Right, Input::Right, Input::Activate],
			},
			Self::Eight => match other {
				Self::Zero => vec![Input::Down, Input::Down, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Down, Input::Down, Input::Down, Input::Right, Input::Activate],
				Self::One => vec![Input::Down, Input::Down, Input::Left, Input::Activate],
				Self::Two => vec![Input::Down, Input::Down, Input::Activate],
				Self::Three => vec![Input::Down, Input::Down, Input::Right, Input::Activate],
				Self::Four => vec![Input::Left, Input::Down, Input::Activate],
				Self::Five => vec![Input::Down, Input::Activate],
				Self::Six => vec![Input::Down, Input::Right, Input::Activate],
				Self::Seven => vec![Input::Left, Input::Activate],
				Self::Eight => vec![Input::Activate],
				Self::Nine => vec![Input::Activate],
			},
			Self::Nine => match other {
				Self::Zero => vec![Input::Left, Input::Down, Input::Down, Input::Down, Input::Activate],
				Self::Activate => vec![Input::Down, Input::Down, Input::Down, Input::Activate],
				Self::One => vec![Input::Left, Input::Left, Input::Down, Input::Down, Input::Activate],
				Self::Two => vec![Input::Left, Input::Down, Input::Down, Input::Activate],
				Self::Three => vec![Input::Down, Input::Down, Input::Activate],
				Self::Four => vec![Input::Left, Input::Left, Input::Down, Input::Activate],
				Self::Five => vec![Input::Left, Input::Down, Input::Activate],
				Self::Six => vec![Input::Down, Input::Activate],
				Self::Seven => vec![Input::Left, Input::Left, Input::Activate],
				Self::Eight => vec![Input::Left, Input::Activate],
				Self::Nine => vec![Input::Activate],
			},
		}
	}

	fn append_to_number(&self, number: usize) -> usize {
		let next_digit = match self {
			Self::Zero => 0,
			Self::Activate => return number,
			Self::One => 1,
			Self::Two => 2,
			Self::Three => 3,
			Self::Four => 4,
			Self::Five => 5,
			Self::Six => 6,
			Self::Seven => 7,
			Self::Eight => 8,
			Self::Nine => 9,
		};
		number * 10 + next_digit
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let codes: Vec<Vec<NumericKey>> = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut codes = Vec::new();
		for line in input_string.lines() {
			let mut code = Vec::new();
			for c in line.chars() {
				code.push(match c {
					'0' => NumericKey::Zero,
					'A' => NumericKey::Activate,
					'1' => NumericKey::One,
					'2' => NumericKey::Two,
					'3' => NumericKey::Three,
					'4' => NumericKey::Four,
					'5' => NumericKey::Five,
					'6' => NumericKey::Six,
					'7' => NumericKey::Seven,
					'8' => NumericKey::Eight,
					'9' => NumericKey::Nine,
					_ => unimplemented!(),
				});
			}
			codes.push(code);
		}
		codes
	};

	let mut number_robot = NumericKey::Activate;
	let mut first_robot = Input::Activate;
	let mut second_robot = Input::Activate;

	let mut complexity_sum = 0;
	for code in codes {
		let mut numeric_part = 0;
		for key in code.iter() {
			numeric_part = key.append_to_number(numeric_part);
		}

		let mut number_directions = Vec::new();
		for key in code {
			for direction in number_robot.press_to_reach(key) {
				number_directions.push(direction);
			}
			number_robot = key;
		}

		let mut first_directions = Vec::new();
		for number_direction in number_directions {
			for direction in first_robot.press_to_reach(number_direction) {
				first_directions.push(direction);
			}
			first_robot = number_direction;
		}

		let mut second_directions = Vec::new();
		for first_direction in first_directions {
			for direction in second_robot.press_to_reach(first_direction) {
				second_directions.push(direction);
			}
			second_robot = first_direction;
		}

		complexity_sum += second_directions.len() * numeric_part;
	}

	println!("{}", complexity_sum);

	Ok(())
}
