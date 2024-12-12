package main

import (
    "fmt"
    "os"
    "strings"
	"strconv"
)

func main() {
	content, err := os.ReadFile("input/day02.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
    lines := strings.Split(trimmed, "\n")
	fmt.Println(len(lines))

	nums := make([][]int, len(lines))
    for i, line := range lines {
		values := strings.Split(line, " ")
		if len(line) == 0 { continue }
		nums[i] = make([]int, len(values))
		for j, val := range values {
			num, _ := strconv.Atoi(val)
			nums[i][j] = num
		}
    }
	res := part2(nums)
	fmt.Println(res)
}

func part1(nums [][]int) int {
	count := 0
	for _, arr := range nums {
		if isSafe(arr) {
			count += 1
		}
	}
	return count
}

func isSafe(arr []int) bool {
	if len(arr) == 1 { return true }
	var dir int
	if arr[1] - arr[0] >= 0 {
		dir = 1
	} else {
		dir = -1
	}
	for i := 1; i < len(arr); i++ {
		diff := arr[i] - arr[i-1]
		if diff * dir <= 0 { return false }
		if diff < 0 { diff = -diff }
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

func part2(nums [][]int) int {
	count := 0
	for _, arr := range nums {
		if isSafeWithDampener(arr) {
			count += 1
		}
	}
	return count
}

func isSafeWithDampener(arr []int) bool {
	dir := 0
	diffs := make([]int, len(arr) - 1)
	neg_count, pos_count := calculateDirectionCount(arr, diffs)
	if pos_count < 2 {
		dir = -1
	} else if neg_count < 2 {
		dir = 1
	} else {
		return false // Unsalvageable bc too many unsafes
	}
	if dir == -1 {       // Flip to positive for unified logic
		for i:=0; i<len(diffs);i++ {
			diffs[i] = -diffs[i]
		}
	}
	lifelinesUsed := 0 
	var foundShift bool = findDirectionShiftAndFixIt(diffs)
	if foundShift { lifelinesUsed += 1 }
	if lifelinesUsed == 0 && diffs[0] > 3 {
		diffs[0] = 1
	} else if lifelinesUsed == 0 {
		diffs[len(diffs) - 1] = 1
	}
	// Check all diff values and make sure they are all positive
	for i := 0; i < len(diffs); i++ {
		if diffs[i] < 1 || diffs[i] > 3 { 
			return false
		}
	}
	return true
}

func calculateDirectionCount(arr []int, diffs []int) (int, int) {
	neg_count := 0
	pos_count := 0
	for i := 1; i < len(arr); i++ {
		diffs[i-1] = arr[i] - arr[i-1]
		if diffs[i-1] < 0 {
			neg_count++
		} else if diffs[i-1] > 0 {
			pos_count++
		}
	}
	return neg_count, pos_count
}

func findDirectionShiftAndFixIt(diffs []int) bool {
	for i := 0; i < len(diffs); i++ {
		if diffs[i] > 0 { 
			continue 
		} else {         // Found a negative value which means shift in direction
			leftWasSafe := i == 0 || ( diffs[i-1] > 0 && diffs[i-1] < 4 )
			leftCouldBeSafe := i == 0 || ( diffs[i-1] + diffs[i] > 0 && diffs[i-1] + diffs[i] < 4 )
			rightWasSafe := i == len(diffs) - 1 || ( diffs[i+1] > 0 && diffs[i+1] < 4 )
			rightCouldBeSafe := i == len(diffs) - 1 || ( diffs[i+1] + diffs[i] > 0 && 
					diffs[i+1] + diffs[i] < 4 )
			if !leftWasSafe && leftCouldBeSafe {
				diffs[i-1] += diffs[i]
			} else if !rightWasSafe && rightCouldBeSafe {
				diffs[i+1] += diffs[i]
			} else if leftWasSafe && leftCouldBeSafe {
				if i != 0 {
					diffs[i-1] += diffs[i]
				}
			} else {
				if i != len(diffs) - 1 {
					diffs[i+1] += diffs[i]
				}
			}
			diffs[i] = 1 
			return true
		}
	}
	return false
}

// For part 1
// Make a slice of ints based on the line levels
// Loop over the slice and test if the line is 'safe'
// A line becomes unsafe when it increases and decreases at the same time, or if
//    the difference between adjacent values is less than 1 or bigger than 3
// Keep a counter of the lines which are safe
// Return the count
