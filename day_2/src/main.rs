use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::delimited;
use nom::IResult;
use regex::Regex;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_2/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<u64> {
	let actual_bag = Bag {
		r: 12,
		g: 13,
		b: 14,
	};

	let games = parse_games(input)?;

	let mut result = 0;
	for game in games {
		if game
			.bags
			.iter()
			.all(|bag| bag.r <= actual_bag.r && bag.g <= actual_bag.g && bag.b <= actual_bag.b)
		{
			result += game.id;
		}
	}
	Ok(result)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
	let games = parse_games(input)?;

	let mut result = 0;
	for game in games {
		let mut min_bag = Bag::default();
		for bag in game.bags {
			min_bag.r = min_bag.r.max(bag.r);
			min_bag.g = min_bag.g.max(bag.g);
			min_bag.b = min_bag.b.max(bag.b);
		}
		result += min_bag.r * min_bag.g * min_bag.b;
	}

	Ok(result)
}

fn parse_games(input: &str) -> anyhow::Result<Vec<Game>> {
	let red = Regex::new(r"(\d+) red")?;
	let green = Regex::new(r"(\d+) green")?;
	let blue = Regex::new(r"(\d+) blue")?;

	let mut games = Vec::new();
	for line in input.lines() {
		let (line, game_id) = delimited(tag("Game "), parse_number::<u64>, tag(": "))(line)
			.map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())?;

		let mut bags = Vec::new();
		for bag_input in line.split("; ") {
			let mut bag = Bag::default();

			if let Some(red_captures) = red.captures(bag_input) {
				bag.r = red_captures[1].parse::<u64>()?;
			}
			if let Some(green_captures) = green.captures(bag_input) {
				bag.g = green_captures[1].parse::<u64>()?;
			}
			if let Some(blue_captures) = blue.captures(bag_input) {
				bag.b = blue_captures[1].parse::<u64>()?;
			}

			bags.push(bag);
		}

		games.push(Game { id: game_id, bags });
	}

	Ok(games)
}

#[derive(Default, Debug)]
struct Bag {
	r: u64,
	g: u64,
	b: u64,
}

struct Game {
	id: u64,
	bags: Vec<Bag>,
}

fn parse_number<T: FromStr>(input: &str) -> IResult<&str, T> {
	map_res(digit1, |num: &str| num.parse::<T>())(input)
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
	#[test]
	fn part_1_example_1() {
		let expected = 8;
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), expected);
	}

	#[test]
	fn part_2_example_1() {
		let expected = 2286;
		assert_eq!(super::part_2(EXAMPLE_1).unwrap(), expected);
	}
}
