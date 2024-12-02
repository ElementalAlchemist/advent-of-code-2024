use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let levels_report = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut levels: Vec<Vec<u32>> = Vec::new();
		for line in input_string.lines() {
			let parts = line.split(' ');
			let line_levels: Vec<u32> = parts.into_iter().map(|level| level.parse().unwrap()).collect();
			levels.push(line_levels);
		}

		levels
	};

	let mut safe_reports_count: u32 = 0;
	for report in levels_report {
		'skip_loop: for skip_index in 0..=report.len() {
			let mut report_iter = report
				.iter()
				.enumerate()
				.filter(|(index, _)| *index != skip_index)
				.map(|(_, value)| value)
				.copied();
			let mut previous = report_iter.next().unwrap();
			let mut current = report_iter.next().unwrap();
			let mut increasing: Option<bool> = None;
			loop {
				let is_ok = match increasing {
					Some(true) => previous < current && current - previous <= 3,
					Some(false) => previous > current && previous - current <= 3,
					None => {
						increasing = Some(current > previous);
						previous != current && current.abs_diff(previous) <= 3
					}
				};
				if !is_ok {
					continue 'skip_loop;
				}
				previous = current;
				current = match report_iter.next() {
					Some(value) => value,
					None => break,
				};
			}
			safe_reports_count += 1;
			break;
		}
	}

	println!("{}", safe_reports_count);

	Ok(())
}
