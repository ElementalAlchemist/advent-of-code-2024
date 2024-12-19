use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Color {
	White,
	Blue,
	Black,
	Red,
	Green,
}

impl Color {
	fn from_char(c: char) -> Self {
		match c {
			'w' => Self::White,
			'u' => Self::Blue,
			'b' => Self::Black,
			'r' => Self::Red,
			'g' => Self::Green,
			_ => unimplemented!(),
		}
	}
}

#[derive(Default)]
struct TowelColorTrieNode {
	end: bool,
	next: HashMap<Color, TowelColorTrieNode>,
}

#[derive(Default)]
struct TowelColorTrie {
	head: HashMap<Color, TowelColorTrieNode>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (towel_patterns, desired_layouts) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut towels: Vec<Vec<Color>> = Vec::new();
		let mut lines_iter = input_string.lines();
		let towels_line = lines_iter.next().expect("Towels list");
		assert!(lines_iter.next().map(|s| s.is_empty()).unwrap_or(false));

		for towel_str in towels_line.split(", ") {
			let mut towel = Vec::new();
			for c in towel_str.chars() {
				towel.push(Color::from_char(c));
			}
			towels.push(towel);
		}

		let mut desired_layouts: Vec<Vec<Color>> = Vec::new();
		for line in lines_iter {
			let mut layout = Vec::new();
			for c in line.chars() {
				layout.push(Color::from_char(c));
			}
			desired_layouts.push(layout);
		}

		(towels, desired_layouts)
	};

	let mut color_trie = TowelColorTrie::default();
	for towel_pattern in towel_patterns {
		let mut color_iter = towel_pattern.into_iter();
		let first_color = color_iter.next().expect("At least one color");
		let mut trie_node = color_trie.head.entry(first_color).or_default();
		for color in color_iter {
			trie_node = trie_node.next.entry(color).or_default();
		}
		trie_node.end = true;
	}
	let color_trie = color_trie; // clear mut

	let mut possible_designs = 0;
	for design in desired_layouts {
		let mut color_iter = design.into_iter();
		let first_color = color_iter.next().expect("At least one color");
		let Some(first_pointer) = color_trie.head.get(&first_color) else {
			continue;
		};
		let mut pointers = vec![first_pointer];
		for color in color_iter {
			let mut new_start = false;
			let mut new_pointers = Vec::new();
			for pointer in pointers {
				if !new_start && pointer.end {
					new_start = true;
					if let Some(color_node) = color_trie.head.get(&color) {
						new_pointers.push(color_node);
					}
				}
				let Some(next_pointer) = pointer.next.get(&color) else {
					continue;
				};
				new_pointers.push(next_pointer);
			}
			pointers = new_pointers;
			if pointers.is_empty() {
				break;
			}
		}
		if pointers.iter().any(|pointer| pointer.end) {
			possible_designs += 1;
		}
	}

	println!("{}", possible_designs);

	Ok(())
}
