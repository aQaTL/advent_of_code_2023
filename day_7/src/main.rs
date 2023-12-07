use anyhow::bail;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_7/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let mut hands = Vec::new();
	for line in input.lines() {
		let (cards_str, bid) = line.split(' ').next_tuple().unwrap();
		let mut cards = [0; 5];
		for idx in 0..5 {
			let c = cards_str.as_bytes()[idx];
			if c.is_ascii_digit() {
				cards[idx] = c - b'0';
			} else {
				cards[idx] = match c {
					b'T' => 10,
					b'J' => 11,
					b'Q' => 12,
					b'K' => 13,
					b'A' => 14,
					_ => bail!("invalid card {}", c as char),
				};
			}
		}

		let bid = bid.parse::<i64>()?;

		let mut card_counts = HashMap::new();
		cards
			.iter()
			.for_each(|c| *card_counts.entry(c).or_default() += 1);
		let mut card_counts: Vec<u8> = card_counts.into_values().collect();
		card_counts.sort_unstable_by_key(|c| Reverse(*c));

		let hand_kind = match card_counts.as_slice() {
			[5] => HandKind::FiveOfAKind,
			[4, 1] => HandKind::FourOfAKind,
			[3, 2] => HandKind::FullHouse,
			[3, 1, 1] => HandKind::ThreeOfAKind,
			[2, 2, 1] => HandKind::TwoPair,
			[2, 1, 1, 1] => HandKind::OnePair,
			[1, 1, 1, 1, 1] => HandKind::HighCard,
			_ => bail!("invalid card pattern {card_counts:?} for {cards_str}"),
		};

		hands.push(Hand {
			cards,
			kind: hand_kind,
			bid,
		});
	}

	hands.sort();
	let total_winnings = hands
		.iter()
		.enumerate()
		.map(|(idx, hand)| hand.bid * (idx as i64 + 1))
		.sum();

	Ok(total_winnings)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let mut hands = Vec::new();
	for line in input.lines() {
		let (cards_str, bid) = line.split(' ').next_tuple().unwrap();
		let mut cards = [0; 5];
		for idx in 0..5 {
			let c = cards_str.as_bytes()[idx];
			if c.is_ascii_digit() {
				cards[idx] = c - b'0';
			} else {
				cards[idx] = match c {
					b'T' => 10,
					b'J' => 1,
					b'Q' => 12,
					b'K' => 13,
					b'A' => 14,
					_ => bail!("invalid card {}", c as char),
				};
			}
		}

		let bid = bid.parse::<i64>()?;

		let mut card_counts = HashMap::new();
		cards
			.iter()
			.for_each(|c| *card_counts.entry(c).or_default() += 1);
		let jocker_count = card_counts.get(&1).copied().unwrap_or_default();
		let mut card_counts: Vec<u8> = card_counts.into_values().collect();
		card_counts.sort_unstable_by_key(|c| Reverse(*c));

		let hand_kind = match card_counts.as_slice() {
			[5] => HandKind::FiveOfAKind,

			[4, 1] if jocker_count == 1 => HandKind::FiveOfAKind,
			[4, 1] if jocker_count == 4 => HandKind::FiveOfAKind,
			[4, 1] => HandKind::FourOfAKind,

			[3, 2] if jocker_count == 2 => HandKind::FiveOfAKind,
			[3, 2] if jocker_count == 3 => HandKind::FiveOfAKind,
			[3, 2] => HandKind::FullHouse,

			[3, 1, 1] if jocker_count == 1 => HandKind::FourOfAKind,
			[3, 1, 1] if jocker_count == 3 => HandKind::FourOfAKind,
			[3, 1, 1] => HandKind::ThreeOfAKind,

			[2, 2, 1] if jocker_count == 1 => HandKind::FullHouse,
			[2, 2, 1] if jocker_count == 2 => HandKind::FourOfAKind,
			[2, 2, 1] => HandKind::TwoPair,

			[2, 1, 1, 1] if jocker_count == 1 => HandKind::ThreeOfAKind,
			[2, 1, 1, 1] if jocker_count == 2 => HandKind::ThreeOfAKind,
			[2, 1, 1, 1] => HandKind::OnePair,

			[1, 1, 1, 1, 1] if jocker_count == 1 => HandKind::OnePair,
			[1, 1, 1, 1, 1] => HandKind::HighCard,
			_ => bail!("invalid card pattern {card_counts:?} for {cards_str}"),
		};

		hands.push(Hand {
			cards,
			kind: hand_kind,
			bid,
		});
	}

	hands.sort_unstable();
	let total_winnings = hands
		.iter()
		.enumerate()
		.map(|(idx, hand)| hand.bid * (idx as i64 + 1))
		.sum();

	Ok(total_winnings)
}

#[derive(Debug, Eq)]
struct Hand {
	cards: [u8; 5],
	kind: HandKind,
	bid: i64,
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.kind.cmp(&other.kind) {
			o @ Ordering::Greater | o @ Ordering::Less => o,
			Ordering::Equal => self.cards.cmp(&other.cards),
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
		self.kind == other.kind
	}
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
enum HandKind {
	HighCard = 1,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), 6440);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EXAMPLE_1).unwrap(), 5905);
	}
}
