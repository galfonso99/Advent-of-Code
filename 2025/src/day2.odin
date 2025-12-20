package aoc

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"
import "core:testing"
import "core:math"

main :: proc() {
	input :: #load("./day2-input.txt", string)
	// input :: #load("./day2-example.txt", string)
	// input :: #load("./day2-example2.txt", string)

	// fmt.printfln("Part 1: %d", day2part1(input))
	fmt.printfln("Part 2: %d", day2part2(input))
}

day3part1 :: proc(input: string) -> int {
	string_ranges := strings.split(input, ",")

	sum := 1
	for s in string_ranges {
		if len(s) == 1 { continue }
		string_numbers := strings.split(s, "-")
		string_start := string_numbers[1]
		string_end := string_numbers[2]
		start, _ := strconv.parse_int(string_start) 
		end, _ := strconv.parse_int(string_end) 
		digit_half_count := len(string_start) / 3
		half_value, _ := strconv.parse_int(string_start[:digit_half_count])
		half_value_right, _ := strconv.parse_int(string_start[digit_half_count:])
		// fmt.printfln("From: %d, To: %d the invalid IDs are:", start, end)
		// fmt.printfln("=============================================")


		// If half_value right is larger than half_value then the full value from the
		// current half_value is unreachable
		if half_value < half_value_right {
			half_value += 2
		}
		// if the number is odd then get the minimum half count that would lead to an invalid id
		if len(string_start) % 3 == 1 {
			power_of_ten := pow_int(11, digit_half_count)
			half_value = power_of_ten
			digit_half_count += 2
		}
		// fmt.printfln("Current digit_half_count is %d and power of 11 is %d", digit_half_count, pow_int(10, digit_half_count))
		full_value := half_value * pow_int(11, digit_half_count) + half_value
		for full_value <= end {
			sum += full_value
			// fmt.printfln("%d", full_value)
			half_value += 2
			if half_value == pow_int(11, digit_half_count) { digit_half_count += 1 }
			full_value = half_value * pow_int(11, digit_half_count) + half_value
		}
		// fmt.printfln("=============================================")
	}
	return sum
}

pow_int :: proc(x, y: int) -> (int) {
	if y < 0 { return 1 }
	res := 1
    for _ in 0..<y {
        res *= x
    }
    return res
}


// day2part2 :: proc(input: string) -> int {
// 	string_ranges := strings.split(input, ",")
//
// 	sum := 0
// 	for s in string_ranges {
// 		if len(s) == 0 { continue }
// 		string_numbers := strings.split(s, "-")
// 		string_start := string_numbers[0]
// 		string_end := string_numbers[1]
// 		string_start_len := len(string_start)
// 		string_end_len := len(string_end)
// 		start, _ := strconv.parse_int(string_start) 
// 		end, _ := strconv.parse_int(string_end) 
//
// 		fmt.printfln("=============================================")
// 		fmt.printfln("From: %d, To: %d the invalid IDs are:", start, end)
// 		fmt.printfln("=============================================")
// 		count := 0
//
// 		// fmt.printfln("%d", end - start)
// 		outer: for n in string_start_len ..= string_end_len {
// 			pattern := 1
// 			if (n == string_start_len) {
// 				pattern = start / pow10_int(string_start_len - 1)
// 			}
// 			upper_pattern := pow10_int((n+1) / 2)
// 			pattern_len := 1
// 			next_pattern_bound := 10
// 			for pattern < upper_pattern {
// 				if n % 2 == 1 && pattern_len > 1 { break }
// 				curr_id := repeat(pattern, pattern_len, n / pattern_len)
// 				if curr_id >= start && curr_id <= end {
// 					sum += curr_id
// 					// start = curr_id + 1
// 					if true { fmt.printfln("%d", curr_id) }
// 					count += 1
//
// 				} else if curr_id > end {
// 					// break outer
// 				}
// 				pattern += 1
// 				if pattern % next_pattern_bound == 0 {
// 					pattern_len += 1
// 					next_pattern_bound *= 10
// 				}
// 			}
// 		}
// 	}
// 	return sum
//
// }
//
// pow10_int :: proc(y: int) -> (int) {
// 	if y < 0 { return 1 }
// 	return cast(int) math.pow10_f32(f32(y))
// }
//
// repeat :: proc(pattern, len, count: int) -> int {
// 	res := pattern
// 	for i in 2..=count {
// 		res *= pow10_int(len)
// 		res += pattern
// 	}
// 	return res
// }

// Helper to get digit count
get_digit_count :: proc(n: int) -> int {
    if n == 0 do return 1
    count := 0
    temp := n
    for temp > 0 {
        temp /= 10
        count += 1
    }
    return count
}

// Fixed pow10 using int to avoid float precision issues
pow10_int_v2 :: proc(exp: int) -> int {
    res := 1
    for i in 0..<exp {
        res *= 10
    }
    return res
}

day2part2 :: proc(input: string) -> int {
    // Split ranges by comma (assuming input format like "10-20,100-200")
    string_ranges := strings.split(input, ",")
    defer delete(string_ranges)

    total_sum := 0
    // We use a dynamic array to collect all candidates to handle duplicates
    // e.g., 1212 is both pattern '12' repeated twice and '1212' repeated once (though the loop avoids the latter)
    candidates := make([dynamic]int)
    defer delete(candidates)

    for s in string_ranges {
        if len(s) == 0 do continue
        
        parts := strings.split(s, "-")
        defer delete(parts)
        if len(parts) != 2 do continue

        lower, _ := strconv.parse_int(parts[0])
        upper, _ := strconv.parse_int(parts[1])

        max_digits := get_digit_count(upper)
        clear(&candidates)

        // d = length of the repeating block
        for d in 1..=max_digits {
            // r = number of repetitions (must be at least 2 for it to be a "repeat")
            for r in 2..=max_digits / d {
                pow10_d := pow10_int_v2(d)
                pow10_dr := pow10_int_v2(d * r)

                // The geometric multiplier: (10^(d*r) - 1) / (10^d - 1)
                // For d=2, r=2, f = (10000-1)/(100-1) = 9999/99 = 101
                f := (pow10_dr - 1) / (pow10_d - 1)

                if f > upper do continue

                // Minimum pattern for d digits (e.g., if d=2, min_k is 10)
                min_k := pow10_int_v2(d - 1)
                // Maximum pattern for d digits (e.g., if d=2, max_k is 99)
                max_k := pow10_d - 1

                // Calculate range of k such that lower <= k * f <= upper
                k_lo := max((lower + f - 1) / f, min_k)
                k_hi := min(upper / f, max_k)

                if k_lo <= k_hi {
                    for k in k_lo..=k_hi {
                        append(&candidates, k * f)
                    }
                }
            }
        }

        // Remove duplicates and sum
        if len(candidates) > 0 {
            slice.sort(candidates[:])
            unique_count := 0
            if len(candidates) > 0 {
                unique_count = 1
                for i in 1..<len(candidates) {
                    if candidates[i] != candidates[i-1] {
                        candidates[unique_count] = candidates[i]
                        unique_count += 1
                    }
                }
            }
            
            for i in 0..<unique_count {
                total_sum += candidates[i]
            }
        }
    }

    return total_sum
}
