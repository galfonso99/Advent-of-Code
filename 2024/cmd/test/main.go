package main

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"slices"
	"strings"
)

func main() {
	fmt.Printf("Part 1: %d\n", part1())
	fmt.Printf("Part 2: %d\n", part2())
}

func part1() int {
	safeReports := 0

	reports := processInput()

	for _, report := range reports {
		if isReportValid(report) {
			safeReports++
		}
	}

	return safeReports
}

func part2() int {
	safeReports := 0

	reports := processInput()

	fakeCount := 0
	for _, report := range reports {
		copyReport := make([]int, len(report))
		_ = copy(copyReport, report)
		printCopy := make([]int, len(report))
		_ = copy(printCopy, copyReport)
		if isReportValidWithTolerance(report) {
			if !isSafeWithDampener(copyReport) {
				fmt.Println(printCopy)
			}
			safeReports++
		}
		if isSafeWithDampener(report) {
			fakeCount++
		}
	}
		fmt.Print("Fake Count ")
		fmt.Println(fakeCount)

	return safeReports
}

// Loop through the array in pairs and make an array of the differences of the pairs like
// [1, 2, 3, 3, 1, -1, 2]
// While in the same loop make sure to also count the amount of positive diffs and the
// amount of negative diffs
// Outside the loop: if neg_count < 2 {dir = 1} else if pos_count < 2 {dir = -1} else return false
// If dir < 0 Multiply all values by dir (Effectively flipping them to positive

// Once again loop over the diff array look for the first negative value or zero
// Look left and look right and if adding this negative number make the value between 1 and 3
// then do that, if the current value is not first value check left first then check right
// otherwise check right first then check left make sure to check if left or right even exist
// And if they dont you can still choose them

// Now loop over the changed array again and if all the values are withing the range 1 to 3
// return true else return false

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

func processInput() [][]int {
	file, err := os.ReadFile("input/day02.txt")
	if err != nil {
		log.Fatal(err)
	}

	formatInput := strings.Split(strings.TrimSpace(string(file)), "\n")
	var reports [][]int
	for _, v := range formatInput {
		var report []int
		_ = json.Unmarshal([]byte("["+strings.ReplaceAll(v, " ", ",")+"]"), &report)
		reports = append(reports, report)
	}

	return reports
}

func isReportValid(report []int) bool {
	for i := 0; i < len(report)-1; i++ {
		validReportLevels := validateReportLevels(report)

		if !validReportLevels {
			return false
		}
	}
	return true
}

func validateReportLevels(report []int) bool {
	var decreasingDirection bool
	for i := 1; i < len(report); i++ {
		if i == 1 {
			decreasingDirection = report[i-1] > report[i]
		}
		stillDecreasing := report[i-1] > report[i]
		if decreasingDirection != stillDecreasing {
			return false
		}
		difference := absInt(report[i-1], report[i])
		if difference < 1 || difference > 3 {
			return false
		}
	}
	return true
}

func absInt(a, b int) int {
	return maxInt(a, b) - minInt(a, b)
}

func minInt(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func isReportValidWithTolerance(report []int) bool {
	numInvalidLevels, invalidLevelLocations := invalidReportStats(report)

	if numInvalidLevels == 0 {
		return true
	}
	for _, v := range invalidLevelLocations {

		filteredSlice := removeIndex(report, v)

		if isReportValid(filteredSlice) {
			return true
		}
	}
	return false
}

func invalidReportStats(report []int) (numInvalidLevels int, invalidLevelLocations []int) {
	totalInvalidLevels := 0

	for i := 1; i < len(report); i++ {
		validReportLevels := validateReportLevels(report)
		if !validReportLevels {
			invalidLevelLocations = append(invalidLevelLocations, i-1, i)
			totalInvalidLevels++
			continue
		}
	}
	return totalInvalidLevels, slices.Clip(invalidLevelLocations)
}

func removeIndex(slice []int, index int) []int {
	newSlice := make([]int, len(slice))
	copy(newSlice, slice)
	newSlice = slices.Clip(slices.Delete(newSlice, index, index+1))
	return newSlice
}
