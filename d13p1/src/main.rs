use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

struct MachineConfiguration {
	a: Coordinate,
	b: Coordinate,
	prize: Coordinate,
}

#[derive(Default)]
struct MachineConfigurationBuilder {
	a: Option<Coordinate>,
	b: Option<Coordinate>,
	prize: Option<Coordinate>,
}

impl MachineConfigurationBuilder {
	fn has_any_data(&self) -> bool {
		self.a.is_some() || self.b.is_some() || self.prize.is_some()
	}

	fn build(&self) -> Option<MachineConfiguration> {
		match (self.a, self.b, self.prize) {
			(Some(a), Some(b), Some(prize)) => Some(MachineConfiguration { a, b, prize }),
			_ => None,
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let machine_configurations = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut configurations: Vec<MachineConfiguration> = Vec::new();
		let mut current_configuration = MachineConfigurationBuilder::default();
		for line in input_string.lines() {
			if line.is_empty() {
				let new_configuration = current_configuration.build().expect("Complete configuration");
				configurations.push(new_configuration);
				continue;
			}
			if let Some(info) = line.strip_prefix("Button A: X+") {
				let (x, y) = info.split_once(", Y+").expect("Split delimiter exists");
				let x: u32 = x.parse()?;
				let y: u32 = y.parse()?;
				current_configuration.a = Some(Coordinate { x, y });
			} else if let Some(info) = line.strip_prefix("Button B: X+") {
				let (x, y) = info.split_once(", Y+").expect("Split delimiter exists");
				let x: u32 = x.parse()?;
				let y: u32 = y.parse()?;
				current_configuration.b = Some(Coordinate { x, y });
			} else if let Some(info) = line.strip_prefix("Prize: X=") {
				let (x, y) = info.split_once(", Y=").expect("Split delimiter exists");
				let x: u32 = x.parse()?;
				let y: u32 = y.parse()?;
				current_configuration.prize = Some(Coordinate { x, y });
			}
		}
		if current_configuration.has_any_data() {
			let new_configuration = current_configuration.build().expect("Complete configuration");
			configurations.push(new_configuration);
		}

		configurations
	};

	let mut token_count = 0;
	for claw_machine in machine_configurations {
		let mut a_count: u32 = 0;
		let mut b_count = (claw_machine.prize.x / claw_machine.b.x).min(claw_machine.prize.y / claw_machine.b.y);
		while a_count > 0 || b_count > 0 {
			let mut current_position = Coordinate {
				x: (claw_machine.a.x * a_count) + (claw_machine.b.x * b_count),
				y: (claw_machine.a.y * a_count) + (claw_machine.b.y * b_count),
			};
			while b_count > 0
				&& (current_position.x > claw_machine.prize.x || current_position.y > claw_machine.prize.y)
			{
				b_count -= 1;
				current_position.x -= claw_machine.b.x;
				current_position.y -= claw_machine.b.y;
			}
			while current_position.x < claw_machine.prize.x || current_position.y < claw_machine.prize.y {
				a_count += 1;
				current_position.x += claw_machine.a.x;
				current_position.y += claw_machine.a.y;
			}
			if current_position == claw_machine.prize {
				token_count += a_count * 3 + b_count;
				break;
			}
			if b_count == 0 && (current_position.x > claw_machine.prize.x || current_position.y > claw_machine.prize.y)
			{
				break;
			}
		}
	}

	println!("{}", token_count);

	Ok(())
}
