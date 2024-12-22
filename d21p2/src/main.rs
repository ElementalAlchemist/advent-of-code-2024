use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Input {
	Up,
	Right,
	Down,
	Left,
	Activate,
}

impl Input {
	fn all_keys() -> Vec<Self> {
		vec![Self::Up, Self::Right, Self::Down, Self::Left, Self::Activate]
	}

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
				Self::Down => vec![Self::Left, Self::Down, Self::Activate],
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
				Self::Up => vec![Self::Left, Self::Up, Self::Activate],
				Self::Activate => vec![Self::Up, Self::Activate],
				Self::Left => vec![Self::Left, Self::Left, Self::Activate],
				Self::Down => vec![Self::Left, Self::Activate],
				Self::Right => vec![Self::Activate],
			},
		}
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
	fn all_keys() -> Vec<Self> {
		vec![
			Self::Zero,
			Self::Activate,
			Self::One,
			Self::Two,
			Self::Three,
			Self::Four,
			Self::Five,
			Self::Six,
			Self::Seven,
			Self::Eight,
			Self::Nine,
		]
	}

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

	let input_keys = Input::all_keys();
	let mut input_distance_map: HashMap<Input, HashMap<Input, usize>> = HashMap::new();
	for key_1 in input_keys.iter().copied() {
		let mut single_key_map: HashMap<Input, usize> = HashMap::new();
		for key_2 in input_keys.iter().copied() {
			let mut directions = key_1.press_to_reach(key_2);
			for iter in 1..24 {
				println!("{:?} {:?}: {}", key_1, key_2, iter);
				let mut robot = Input::Activate;
				let mut next_directions = Vec::new();
				for current_direction in directions {
					for direction in robot.press_to_reach(current_direction) {
						next_directions.push(direction);
					}
					robot = current_direction;
				}
				directions = next_directions;
			}
			println!("{:?} {:?}: Distance from {}", key_1, key_2, directions.len());
			let mut robot = Input::Activate;
			let mut distance = 0;
			for next_key in directions {
				distance += robot.press_to_reach(next_key).len();
				robot = next_key;
			}
			single_key_map.insert(key_2, distance);
		}
		input_distance_map.insert(key_1, single_key_map);
	}

	let numeric_keys = NumericKey::all_keys();
	let mut key_distance_map: HashMap<NumericKey, HashMap<NumericKey, usize>> = HashMap::new();
	for key_1 in numeric_keys.iter().copied() {
		let mut single_key_map: HashMap<NumericKey, usize> = HashMap::new();
		for key_2 in numeric_keys.iter().copied() {
			let directions = key_1.press_to_reach(key_2);
			let mut robot = Input::Activate;
			let mut distance = 0;
			for next_key in directions {
				distance += *input_distance_map.get(&robot).unwrap().get(&next_key).unwrap();
				robot = next_key;
			}
			single_key_map.insert(key_2, distance);
		}
		key_distance_map.insert(key_1, single_key_map);
	}

	let mut complexity_sum = 0;
	for code in codes {
		let mut code_directions = 0;
		let mut numeric_part = 0;
		let mut current_key = NumericKey::Activate;
		for key in code {
			numeric_part = key.append_to_number(numeric_part);
			code_directions += *key_distance_map.get(&current_key).unwrap().get(&key).unwrap();
			current_key = key;
		}

		complexity_sum += code_directions * numeric_part;
	}

	println!("{}", complexity_sum);

	Ok(())
}
// 402548389073148 is too high
// 154115708116294 for example