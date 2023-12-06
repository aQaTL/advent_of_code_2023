use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_6/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let (times, distances) = input
		.lines()
		.filter_map(|l| l.split(':').nth(1))
		.next_tuple()
		.unwrap();
	let times = times
		.trim()
		.split(' ')
		.filter_map(|n| n.parse::<i64>().ok());
	let distances = distances
		.trim()
		.split(' ')
		.filter_map(|n| n.parse::<i64>().ok());
	let races: Vec<Race> = times
		.zip(distances)
		.map(|(time, distance)| Race { time, distance })
		.collect();

	let mut won_counts = Vec::new();
	for race in races {
		let mut won = 0;
		for hold_time in 0..=race.time {
			if hold_time * (race.time - hold_time) > race.distance {
				won += 1;
			}
		}
		won_counts.push(won);
	}

	Ok(won_counts.iter().product())
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let (times, distances) = input
		.lines()
		.filter_map(|l| l.split(':').nth(1))
		.next_tuple()
		.unwrap();
	let time = times.trim().replace(" ", "").parse::<i64>()?;
	let distance = distances.trim().replace(" ", "").parse::<i64>()?;

	let mut won = 0;
	for hold_time in 0..=time {
		if hold_time * (time - hold_time) > distance {
			won += 1;
		}
	}

	Ok(won)
}

#[derive(Debug, Copy, Clone)]
struct Race {
	time: i64,
	distance: i64,
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";
	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 288);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EXAMPLE).unwrap(), 71503);
	}
}
