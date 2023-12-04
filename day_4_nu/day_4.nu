#!/usr/bin/env nu

let lines = $env.FILE_PWD | path join .. day_4 input.txt | open | lines

let cards = (
	$lines | 
	split column -r ':\s+|\s+\|\s+' card winning_numbers my_numbers | 
	each { |x| 
		{ 
			winning_numbers: ($x.winning_numbers | split row " " | where ($it != "") | into int),
			my_numbers: ($x.my_numbers | split row " " | where ($it != "") | into int),
		}
	}
)

let part_1 = (
	$cards | each { |x|
		let count = $x.my_numbers | filter { |n| $n in $x.winning_numbers } | length
		if ($count > 0) {
			1 bit-shl ($count - 1)
		} else {
			0
		}
	} |
	math sum
)

mut won = unfold 1 {|i| if $i <= ($cards | length) { {out: 1, next: ($i + 1)} }}
for card in ($cards | enumerate) {
	let idx = $card.index
	let my_numbers = $card.item.my_numbers
	let winning_numbers = $card.item.winning_numbers
	let count = $my_numbers | filter { |n| $n in $winning_numbers } | length
	if $count == 0 {
		continue
	}
	let current_card_count = $won | get $idx
	for card_idx in (($idx + 1)..($idx + $count)) { 
		$won = ($won | upsert $card_idx (($won | get $card_idx) + $current_card_count))
	}
}
let part_2 = ($won | math sum)

print $"Part 1: ($part_1)"
print $"Part 2: ($part_2)"
