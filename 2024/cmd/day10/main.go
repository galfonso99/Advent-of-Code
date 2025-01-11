package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	topMap := parseInput()
	part2(topMap)
}

func parseInput() [][]int {
	content, err := os.ReadFile("input/day10.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
    lines := strings.Split(trimmed, "\n")
	topoMap := make([][]int, len(lines))
	for i, line := range lines {
		topoMap[i] = make([]int, len(line))
		for j := 0; j < len(line); j++ {
			digit := line[j]
			topoMap[i][j], _ = strconv.Atoi(string(digit))
		}
	}
	return topoMap
}
// Have a found 2d array for the found 9s for each starting 0
// Find and store the positions of all the zeroes in the map
// From each starting position launch a maze finding operation using backtracking
// where if the next value is not 1 more than the curr the path stops and backtracks
// Otherwise the path stops when it encounters a 9 and then if that position has not
// been found for that specific tailhead, then add one to the count

func part1(topoMap [][]int) {
	// fmt.Println(topoMap)
	tailHeads := make([][2]int, 0, 5)
	count := 0
	for i, row := range topoMap {
		for j, num := range row {
			if num == 0 {
				tailHeads = append(tailHeads, [2]int {i, j})
			}
		}
	}
	// fmt.Println(tailHeads)
	for _, pos := range tailHeads {
		localCount := 0
		found := make([][]bool, len(topoMap))
		for i, _ := range topoMap { found[i] = make([]bool, len(topoMap[0])) }
		traverse(topoMap, found, &localCount, pos, 0)
		count += localCount
	}
	fmt.Println(count)

}
var dirs = [][2]int {{-1,0}, {0,1}, {1,0}, {0,-1}}

func traverse(topoMap [][]int, found [][]bool, localCount *int, pos [2]int, ind int) {
	if ind == 9 && !found[pos[0]][pos[1]] { 
		*localCount += 1 
		found[pos[0]][pos[1]] = true
		return
	}
	for _, dir := range dirs {
		newPos := [2]int {pos[0] + dir[0], pos[1] + dir[1]}
		if newPos[0] < 0 || newPos[0] >= len(topoMap) || newPos[1] < 0 || newPos[1] >= len(topoMap[0]) {
			continue
		}
		if topoMap[newPos[0]][newPos[1]] == ind + 1 {
			traverse(topoMap, found, localCount, newPos, ind+1)
		}
	}
}

// Part2 is a complete copy of part1 but don't mind duplicate paths to get to the same end
// meaning don't keep track of found ends
func part2(topoMap [][]int) {
	tailHeads := make([][2]int, 0, 5)
	count := 0
	for i, row := range topoMap {
		for j, num := range row {
			if num == 0 {
				tailHeads = append(tailHeads, [2]int {i, j})
			}
		}
	}
	for _, pos := range tailHeads {
		localCount := 0
		traverse2(topoMap, &localCount, pos, 0)
		count += localCount
	}
	fmt.Println(count)
}

func traverse2(topoMap [][]int, localCount *int, pos [2]int, ind int) {
	if ind == 9 { 
		*localCount += 1 
		return
	}
	for _, dir := range dirs {
		newPos := [2]int {pos[0] + dir[0], pos[1] + dir[1]}
		if newPos[0] < 0 || newPos[0] >= len(topoMap) || newPos[1] < 0 || newPos[1] >= len(topoMap[0]) {
			continue
		}
		if topoMap[newPos[0]][newPos[1]] == ind + 1 {
			traverse2(topoMap, localCount, newPos, ind+1)
		}
	}
}
