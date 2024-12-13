use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Coordinate {
	x: i64,
	y: i64,
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
				let x: i64 = x.parse()?;
				let y: i64 = y.parse()?;
				current_configuration.a = Some(Coordinate { x, y });
			} else if let Some(info) = line.strip_prefix("Button B: X+") {
				let (x, y) = info.split_once(", Y+").expect("Split delimiter exists");
				let x: i64 = x.parse()?;
				let y: i64 = y.parse()?;
				current_configuration.b = Some(Coordinate { x, y });
			} else if let Some(info) = line.strip_prefix("Prize: X=") {
				let (x, y) = info.split_once(", Y=").expect("Split delimiter exists");
				let x: i64 = x.parse()?;
				let y: i64 = y.parse()?;
				let x: i64 = x + 10000000000000;
				let y: i64 = y + 10000000000000;
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
		// Set this up as a system of equations:
		// AX * a + BX * b = PRIZEX
		// AY * a + BY * b = PRIZEY
		//
		// The first machine from the example would be
		// 94a + 22b = xxx8400
		// 34a + 67b = xxx5400
		//
		// Rearranging to find the number of A presses:
		// a = (xxx8400 - 22b) / 94
		// a = (xxx5400 - 67b) / 34
		//
		// From here, need to find a 'b' such that both 'a's are equal:
		// (xxx8400 - 22b) / 94 = (xxx5400 - 67b) / 34
		// Or, in the original equation terms:
		// (PRIZEX - BX * b) / AX = (PRIZEY - BY * b) / AY
		// Rearranging further:
		// (PRIZEX / AX) - (BX * b / AX) = (PRIZEY / AY) - (BY * b / AY)
		// (PRIZEX / AX) - (PRIZEY / AY) = (BX * b / AX) - (BY * b / AY)
		// (PRIZEX * AY) - (PRIZEY * AX) = (BX * b * AY) - (BY * b * AX)
		// (PRIZEX * AY) - (PRIZEY * AX) = (BX * AY - BY * AX) * b
		// b = (PRIZEX * AY - PRIZEY * AX) / (BX * AY - BY * AX)

		let negate_all = claw_machine.prize.x * claw_machine.a.y < claw_machine.prize.y * claw_machine.a.x;
		let b = if negate_all {
			(claw_machine.prize.y * claw_machine.a.x - claw_machine.prize.x * claw_machine.a.y)
				/ (claw_machine.b.y * claw_machine.a.x - claw_machine.b.x * claw_machine.a.y)
		} else {
			(claw_machine.prize.x * claw_machine.a.y - claw_machine.prize.y * claw_machine.a.x)
				/ (claw_machine.b.x * claw_machine.a.y - claw_machine.b.y * claw_machine.a.x)
		};
		let a = (claw_machine.prize.x - claw_machine.b.x * b) / claw_machine.a.x;
		if a < 0 || b < 0 {
			continue;
		}
		if claw_machine.a.x * a + claw_machine.b.x * b == claw_machine.prize.x
			&& claw_machine.a.y * a + claw_machine.b.y * b == claw_machine.prize.y
		{
			token_count += a * 3 + b;
		}
	}

	println!("{}", token_count);

	Ok(())
}
