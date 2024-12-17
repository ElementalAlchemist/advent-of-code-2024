use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	// This program is derived from direct analysis of the input. As such, we don't read the input this time.
	let mut intended_output: Vec<u64> = vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 2, 5, 5, 0, 3, 3, 0];

	let mut possible_a = vec![0];
	while let Some(output) = intended_output.pop() {
		let mut new_possible_a = Vec::new();
		for a in possible_a {
			let start_a = a << 3;
			for b in 0..8 {
				let check_a = start_a + b;
				let c = check_a >> (b ^ 1);
				if (b ^ 4 ^ c) & 7 == output {
					new_possible_a.push(check_a);
				}
			}
		}
		possible_a = new_possible_a;
	}

	possible_a.sort_unstable();
	println!("{:?}", possible_a);

	Ok(())
}
