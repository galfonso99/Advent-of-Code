package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	machines := parseInput()
	part2(machines)
}

func parseInput() [][3][2]int {
	content, err := os.ReadFile("input/day13.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
	machineSplits := strings.Split(trimmed, "\n\n")
	machines := make([][3][2]int, len(machineSplits))
	for j := range machineSplits {
		lines := strings.Split(machineSplits[j], "\n")
		for i, line := range lines {
			xIndex := strings.IndexByte(line, 'X')
			commaIndex := strings.IndexByte(line, ',')
			xVal, _ := strconv.Atoi(line[xIndex+2:commaIndex])
			machines[j][i][0] = xVal
			yIndex := strings.IndexByte(line, 'Y')
			yVal, _ := strconv.Atoi(line[yIndex+2:])
			machines[j][i][1] = yVal
		}
	}
	return machines
}

/* Intuition at the end of file */

type MachineInfo struct {
	aButtonX int
	aButtonY int
	bButtonX int
	bButtonY int
	endPointX int
	endPointY int
}

func part1(machines [][3][2]int) {
	var sum int = 0
	for _, machine := range machines {
		info := MachineInfo {
			aButtonX: machine[0][0],
			aButtonY: machine[0][1],
			bButtonX: machine[1][0],
			bButtonY: machine[1][1],
			endPointX: machine[2][0],
			endPointY: machine[2][1],
		}
		tokens := 0
		aCount, bCount := solve(info)
		if aCount * info.aButtonX + bCount * info.bButtonX == info.endPointX &&
			aCount * info.aButtonY + bCount * info.bButtonY == info.endPointY {
			tokens = aCount * 3 + bCount 
		}
		sum += tokens
	}
	fmt.Println(sum)
}

func solve(info MachineInfo) (int, int) {
	x1 := info.aButtonX
	y1 := info.bButtonX
	equals1 := info.endPointX
	x2 := info.aButtonY
	y2 := info.bButtonY
	equals2 := info.endPointY

	a := (equals1 * y2 - equals2 * y1) / (x1 * y2 - x2 * y1)
	b := (equals2 * x1 - equals1 * x2) / (x1 * y2 - x2 * y1)
	// fmt.Println(a, b)
	return a, b
}

type MachineInfo64 struct {
	aButtonX int64
	aButtonY int64
	bButtonX int64
	bButtonY int64
	endPointX int64
	endPointY int64
}

func part2(machines [][3][2]int) {
	var sum int64 = 0
	for _, machine := range machines {
		info := MachineInfo64 {
			aButtonX: int64(machine[0][0]),
			aButtonY: int64(machine[0][1]),
			bButtonX: int64(machine[1][0]),
			bButtonY: int64(machine[1][1]),
			endPointX: int64(machine[2][0] + 10000000000000),
			endPointY: int64(machine[2][1] + 10000000000000),
		}
		var tokens int64 = 0
		aCount, bCount := solve2(info)
		if aCount * info.aButtonX + bCount * info.bButtonX == info.endPointX &&
			aCount * info.aButtonY + bCount * info.bButtonY == info.endPointY {
			tokens = aCount * 3 + bCount 
		}
		sum += tokens
	}
	fmt.Println(sum)
}

func solve2(info MachineInfo64) (int64, int64) {
	x1 := info.aButtonX
	y1 := info.bButtonX
	equals1 := info.endPointX
	x2 := info.aButtonY
	y2 := info.bButtonY
	equals2 := info.endPointY

	a := (equals1 * y2 - equals2 * y1) / (x1 * y2 - x2 * y1)
	b := (equals2 * x1 - equals1 * x2) / (x1 * y2 - x2 * y1)
	// fmt.Println(a, b)
	return a, b
}

// For part 1
// You could use the same method as part 2, or using a simpler method like below
// First parse the input
// start with all b presses that surpass the goal
// and then subtract from b presses and add a presses as necessary
// After this multiply the amount of a presses by 3 and add it to the amount of b presses
// Then add it to the running sum for all machines

// For part 2 
// Use Cramer's Rule to find a and b
// Then Check if plugging in a and b get you to the target location
// And if so then add it to the sum
