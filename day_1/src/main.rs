use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_1/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &String) -> anyhow::Result<u32> {
	let mut result = 0;

	for x in input.lines() {
		let mut first = None;
		let mut last = None;
		for b in x.as_bytes() {
			if b.is_ascii_digit() {
				if first.is_none() {
					first = (b - b'0').into();
					continue;
				} else {
					last = (b - b'0').into();
				}
			}
		}
		let first = first.unwrap();
		let last = last.unwrap_or(first);
		result += (first * 10 + last) as u32;
	}

	Ok(result)
}

fn part_2(input: &String) -> anyhow::Result<u32> {
	let mut result = 0;

	let digits = [
		"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];

	for x in input.lines() {
		let mut res_digits = vec![];
		for (idx, b) in x.as_bytes().iter().enumerate() {
			if b.is_ascii_digit() {
				res_digits.push(b - b'0');
			} else if let Some((digit, _)) = digits
				.iter()
				.find_position(|digit| x.as_bytes()[idx..].starts_with(digit.as_bytes()))
			{
				res_digits.push(digit as u8);
			}
		}
		let first = res_digits.first().unwrap();
		let last = res_digits.last().unwrap_or(first);
		result += (first * 10 + last) as u32;
	}

	Ok(result)
}
