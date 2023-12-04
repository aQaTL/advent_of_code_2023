[String[]]$lines = $PSScriptRoot `
	| Split-Path -Parent `
	| ForEach-Object { Join-Path $_ "day_4" "input.txt" } `
	| Get-Item `
	| Get-Content

$cards = $lines | ForEach-Object{
	$_card_idx, $numbers = $_ -split ": "
	$winning_numbers, $my_numbers = $numbers.Split(" | ")
	[Int64[]]$w = (($winning_numbers -split " ") | Where-Object { $_ -ne "" })
	[Int64[]]$m = (($my_numbers -split " ") | Where-Object { $_ -ne "" })
	[PSCustomObject]@{
		winning_numbers = $w
		my_numbers = $m
	}
}

$part1 = $cards `
	| ForEach-Object{
		$winning_numbers, $my_numbers = $_.winning_numbers, $_.my_numbers
		$count = $my_numbers `
			| Where-Object { $_ -in $winning_numbers } `
			| Measure-Object `
			| Select-Object -ExpandProperty Count
		if ($count -gt 0) {
			1 -shl ($count - 1)
		} else {
			0
		}
	} `
	| Measure-Object -Sum `
	| Select-Object -ExpandProperty Sum

$won = @{}
0..($cards.Count - 1) | ForEach-Object { $won[$_] = 1 }

for ($idx = 0; $idx -lt $cards.Count; $idx++) {
	$count = $cards[$idx].my_numbers `
		| Where-Object { $_ -in $cards[$idx].winning_numbers } `
		| Measure-Object `
		| Select-Object -ExpandProperty Count
	if ($count -eq 0) {
		continue
	}
	$current_card_count = $won[$idx]
	(($idx + 1)..($idx + $count)) | ForEach-Object {
		$won[$_] += $current_card_count
	}
}

$part2 = $won.Values | Measure-Object -Sum | Select-Object -ExpandProperty Sum

Write-Output "Part 1: $part1"
Write-Output "Part 2: $part2"
