[String[]]$lines = $PSScriptRoot `
	| Split-Path -Parent `
	| ForEach-Object { Join-Path $_ "day_1" "input.txt" } `
	| Get-Item `
	| Get-Content

function Part1 {
	param ([String[]]$Lines)

	return $Lines `
	| ForEach-Object {
		$line = $_
		$digits = @() 
		for ($idx = 0; $idx -lt $line.Length; $idx++) {
			if ($line[$idx] -in [char]'0'..[char]'9') {
				$digits += $line[$idx] - [char]"0"
			} 
		}
		$digits[0] * 10 + $digits[-1]
	} `
	| Measure-Object -Sum `
	| Select-Object -ExpandProperty Sum
}

function Part2 {
	param ([String[]]$Lines)

	$digitsArr = "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"

	return $Lines `
	| ForEach-Object {
		$line = $_
		$digits = @() 
		for ($idx = 0; $idx -lt $line.Length; $idx++) {
			if ($line[$idx] -in [char]'0'..[char]'9') {
				$digits += $line[$idx] - [char]"0"
			} else {
				for ($jdx = 0; $jdx -lt $digitsArr.Count; $jdx++) {
					if ($line.Substring($idx).StartsWith($digitsArr[$jdx])) {
						$digits += $jdx + 1
						break;
					}
				}
			}
		}
		$digits[0] * 10 + $digits[-1]
	} `
	| Measure-Object -Sum `
	| Select-Object -ExpandProperty Sum
}

Write-Output "Day: 1: $(Part1 -Lines $lines)"
Write-Output "Day: 2: $(Part2 -Lines $lines)"

