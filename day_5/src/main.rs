use std::ops::Range;
use std::str::FromStr;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_5/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let input = parse_input(input)?;

	let to_soil = convert(&input.seeds, &input.seed_to_soil);
	let to_fertilizer = convert(&to_soil, &input.soil_to_fertilizer);
	let to_water = convert(&to_fertilizer, &input.fertilizer_to_water);
	let to_light = convert(&to_water, &input.water_to_light);
	let to_temperature = convert(&to_light, &input.light_to_temperature);
	let to_humidity = convert(&to_temperature, &input.temperature_to_humidity);
	let to_location = convert(&to_humidity, &input.humidity_to_location);

	Ok(to_location.into_iter().min().unwrap())
}

fn convert(src: &[i64], range_map: &[RangeMap]) -> Vec<i64> {
	src.iter()
		.map(|seed| {
			for RangeMap {
				dst_range_start,
				src_range_start,
				len,
			} in range_map
			{
				let src_range = *src_range_start..(*src_range_start + *len);
				if src_range.contains(seed) {
					let dst_range_idx = seed - src_range_start;
					let new_seed = dst_range_start + dst_range_idx;
					return new_seed;
				}
			}
			*seed
		})
		.collect()
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let input = parse_input(input)?;
	let mut seeds = Vec::new();
	for (start, len) in input.seeds.iter().tuples() {
		seeds.push(*start..(*start + len));
	}

	let to_soil = convert_range(&seeds, &input.seed_to_soil);
	let to_fertilizer = convert_range(&to_soil, &input.soil_to_fertilizer);
	let to_water = convert_range(&to_fertilizer, &input.fertilizer_to_water);
	let to_light = convert_range(&to_water, &input.water_to_light);
	let to_temperature = convert_range(&to_light, &input.light_to_temperature);
	let to_humidity = convert_range(&to_temperature, &input.temperature_to_humidity);
	let to_location = convert_range(&to_humidity, &input.humidity_to_location);

	Ok(to_location
		.into_iter()
		.map(|range| range.min().unwrap())
		.min()
		.unwrap())
}

fn convert_range(src: &[Range<i64>], range_map: &[RangeMap]) -> Vec<Range<i64>> {
	let mut out = Vec::new();
	for range in src {
		let mut ranges = Vec::new();
		let mut range = range.clone();

		for RangeMap {
			dst_range_start,
			src_range_start,
			len,
		} in range_map.into_iter().copied()
		{
			if range.start >= src_range_start && range.start < (src_range_start + len)
				|| range.end <= (src_range_start + len) && range.end > src_range_start
			{
				let dst_range_idx = range.start - src_range_start;
				let new_range_start = dst_range_start + dst_range_idx;
				let new_range_end = new_range_start + (range.end - range.start);

				let new_range =
					new_range_start.max(dst_range_start)..new_range_end.min(dst_range_start + len);

				let dangling_range_right = (src_range_start + len)..range.end;
				let dangling_range_left = range.start..src_range_start;

				out.push(new_range);

				match (
					dangling_range_left.is_empty(),
					dangling_range_right.is_empty(),
				) {
					(true, true) => {
						range = 0..0;
						break;
					}
					(true, false) => {
						ranges.push(dangling_range_right.clone());
						range = dangling_range_right;
					}
					(false, true) => {
						ranges.push(dangling_range_left.clone());
						range = dangling_range_left;
					}
					(false, false) => panic!(),
				}
			}
		}
		if !range.is_empty() {
			ranges.push(range.clone());
		}
		out.extend(ranges.into_iter());
	}
	out
}

fn parse_input(input: &str) -> anyhow::Result<Input> {
	let mut lines = input.lines();
	let seeds: Vec<i64> = lines
		.next()
		.unwrap()
		.split(": ")
		.nth(1)
		.unwrap()
		.split(" ")
		.map(|n| n.parse::<i64>())
		.try_collect()?;
	lines.next().unwrap();
	lines.next().unwrap();

	let mut parse_map = || {
		let mut range_maps = Vec::new();
		while let Some(line) = lines.next() {
			if line == "" {
				lines.next().unwrap();
				break;
			}
			range_maps.push(line.parse::<RangeMap>()?);
		}
		Ok::<Vec<_>, anyhow::Error>(range_maps)
	};

	Ok(Input {
		seeds,
		seed_to_soil: parse_map()?,
		soil_to_fertilizer: parse_map()?,
		fertilizer_to_water: parse_map()?,
		water_to_light: parse_map()?,
		light_to_temperature: parse_map()?,
		temperature_to_humidity: parse_map()?,
		humidity_to_location: parse_map()?,
	})
}

#[derive(Debug)]
struct Input {
	seeds: Vec<i64>,
	seed_to_soil: Vec<RangeMap>,
	soil_to_fertilizer: Vec<RangeMap>,
	fertilizer_to_water: Vec<RangeMap>,
	water_to_light: Vec<RangeMap>,
	light_to_temperature: Vec<RangeMap>,
	temperature_to_humidity: Vec<RangeMap>,
	humidity_to_location: Vec<RangeMap>,
}

#[derive(Debug, Copy, Clone)]
struct RangeMap {
	dst_range_start: i64,
	src_range_start: i64,
	len: i64,
}

impl FromStr for RangeMap {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let numbers: Vec<i64> = s.split(" ").map(|n| n.parse::<i64>()).try_collect()?;
		Ok(RangeMap {
			dst_range_start: numbers[0],
			src_range_start: numbers[1],
			len: numbers[2],
		})
	}
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), 35);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EXAMPLE_1).unwrap(), 46);
	}
}
