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
	'report_loop: for report in levels_report {
		let mut report_windows = report.windows(2);
		let first_window = report_windows.next().unwrap();
		if first_window[0] == first_window[1] {
			continue;
		}
		let increasing = first_window[0] < first_window[1];
		if first_window[0].abs_diff(first_window[1]) > 3 {
			continue;
		}
		for window in report_windows {
			if increasing && window[0] >= window[1] {
				continue 'report_loop;
			}
			if !increasing && window[0] <= window[1] {
				continue 'report_loop;
			}
			if window[0].abs_diff(window[1]) > 3 {
				continue 'report_loop;
			}
		}
		safe_reports_count += 1;
	}

	println!("{}", safe_reports_count);

	Ok(())
}
