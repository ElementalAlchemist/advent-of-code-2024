use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::Write;

const AREA_WIDTH: i32 = 101;
const AREA_HEIGHT: i32 = 103;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

struct Robot {
	position: Coordinate,
	velocity: Coordinate,
}

fn write_grid(file: &mut fs::File, robot_coordinates: &HashSet<Coordinate>) -> std::io::Result<()> {
	file.write_all(b"P1\n")?;
	file.write_all(format!("{} {}\n", AREA_WIDTH, AREA_HEIGHT).as_bytes())?;
	for y in 0..AREA_HEIGHT {
		for x in 0..AREA_WIDTH {
			let coord = Coordinate { x, y };
			let output = match robot_coordinates.contains(&coord) {
				true => b"1",
				false => b"0",
			};
			file.write_all(output)?;
			if x % 70 == 69 {
				file.write_all(b"\n")?;
			}
		}
		file.write_all(b"\n")?;
	}

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let robots = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut robots = Vec::new();
		for line in input_string.lines() {
			let line = line.strip_prefix("p=").expect("Line starts with p=");
			let (position, velocity) = line.split_once(" v=").expect("Line has v=");
			let (position_x, position_y) = position.split_once(',').expect("Position is coordinate");
			let (velocity_x, velocity_y) = velocity.split_once(',').expect("Velocity is coordinate");
			let position_x: i32 = position_x.parse()?;
			let position_y: i32 = position_y.parse()?;
			let velocity_x: i32 = velocity_x.parse()?;
			let velocity_y: i32 = velocity_y.parse()?;
			robots.push(Robot {
				position: Coordinate {
					x: position_x,
					y: position_y,
				},
				velocity: Coordinate {
					x: velocity_x,
					y: velocity_y,
				},
			});
		}

		robots
	};

	for iteration_count in 1..=1_000_000 {
		let mut file = fs::File::create(format!("output_{:08}.pbm", iteration_count))?;

		let mut robot_coordinates: HashSet<Coordinate> = HashSet::new();
		for robot in robots.iter() {
			let x = (robot.position.x + (robot.velocity.x * iteration_count)).rem_euclid(AREA_WIDTH);
			let y = (robot.position.y + (robot.velocity.y * iteration_count)).rem_euclid(AREA_HEIGHT);
			let coord = Coordinate { x, y };
			robot_coordinates.insert(coord);
		}
		write_grid(&mut file, &robot_coordinates)?;
	}

	Ok(())
}
