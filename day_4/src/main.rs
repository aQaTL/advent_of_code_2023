use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_4/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let cards = parse_cards(input);

	let result = cards
		.into_iter()
		.map(|(winning_numbers, my_numbers)| {
			let count = my_numbers
				.into_iter()
				.filter(|n| winning_numbers.contains(n))
				.count();
			if count > 0 {
				1 << (count - 1)
			} else {
				0
			}
		})
		.sum();

	Ok(result)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let cards = parse_cards(input);

	// (card_idx, count)
	let mut won = HashMap::new();
	for (idx, (winning_numbers, my_numbers)) in cards.iter().enumerate() {
		let count = my_numbers
			.iter()
			.filter(|n| winning_numbers.contains(n))
			.count();
		let current_card_count = *won.entry(idx).or_insert(1);
		for card_idx in (idx + 1)..(idx + 1 + count) {
			*won.entry(card_idx).or_insert(1) += current_card_count;
		}
	}

	Ok(won.values().sum())
}

fn parse_cards(input: &str) -> Vec<(Vec<i64>, Vec<i64>)> {
	input
		.lines()
		.map(|line| {
			let numbers = line.split(": ").nth(1).unwrap();
			let mut lists = numbers.split(" | ");
			let winning_numbers: Vec<i64> = lists
				.next()
				.unwrap()
				.split(' ')
				.filter_map(|x| x.parse::<i64>().ok())
				.collect();
			let my_numbers: Vec<i64> = lists
				.next()
				.unwrap()
				.split(' ')
				.filter_map(|x| x.parse::<i64>().ok())
				.collect();
			(winning_numbers, my_numbers)
		})
		.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 13)
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EXAMPLE).unwrap(), 30);
	}
}
