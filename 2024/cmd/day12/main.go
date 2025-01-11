package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	byteArray := parseInput()
	part2(byteArray)
}

func parseInput() [][]byte {
	content, err := os.ReadFile("input/day12.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
	lines := strings.Split(trimmed, "\n")
	byteArray := make([][]byte, len(lines))
	for i, str := range lines {
		byteArray[i] = []byte(str)
	}
	return byteArray
}

// Loop through the crops, pathfind all the connected crops that are the same type and
// record the count as the area of that region
// Find a way to find the perimeter of each region
	/* Actually perimeter is way easier than I though, as you path through if next possible move puts you 
	in another crop or outside boundaries, that counts as ONE perimeter */
// For each region multiply the area by the perimeter and add the result to the running sum

func part1(byteArray [][]byte) {
	sum := 0
	seen := make([][]bool, 150)
	for i := range seen { seen[i] = make([]bool, 150) }
	perimeters := make([]int, 0, 5)
	areas := make([]int, 0, 5)
	for i := range byteArray {
		for j := range byteArray[i] {
			if seen[i][j] { 
				continue 
			}
			perim, area := 0, 0
			walk(byteArray, seen, &perim, &area, i, j) 
			perimeters = append(perimeters, perim)
			areas = append(areas, area)
		}
	}
	for i := range perimeters {
		sum += perimeters[i] * areas[i]
	}
	fmt.Println(sum)
}

var dirs = [][2]int {{-1,0}, {0,1}, {1,0}, {0,-1}}

func walk(garden [][]byte, seen [][]bool, perim *int, area *int, i int, j int) {
	// Insert exit condition here, cannot find one right now
	seen[i][j] = true
	*area += 1
	ch := garden[i][j]
	for _, dir := range dirs {
		x, y := i + dir[0], j + dir[1]
		if x < 0 || x >= len(garden) || y < 0 || y >= len(garden[0]) || garden[x][y] != ch {
			*perim += 1
		}
	}
	for _, dir := range dirs {
		x, y := i + dir[0], j + dir[1]
		// If still within region, RECURSE
		if !(x < 0 || x >= len(garden) || y < 0 || y >= len(garden[0]) || garden[x][y] != ch || seen[x][y]) {
			walk(garden, seen, perim, area, x, y)
		} 
	}
}

var corners = [][2]int {{-1, 1}, {-1, -1}, {1, 1}, {1, -1}}

//Find all the corners of the region (outer corners and inner corners) and that will
// be the count of the amount of side in the region
// A crop is at an outer corner if both a vertical displacement and a horizontal displacement
// would put you outside of the region at the same time
// A crop is at an inner corner if a vert movement stays inside the region and a horiz
// movement also stays inside the region WHILE a diagonal movement in the same direction
// as those other moves puts you outside of the region
// Then when you have both area and count of sides multiply them and add the result from 
// each region

func part2(garden [][]byte) {
	sum := 0
	seen := make([][]bool, 150)
	for i := range seen { seen[i] = make([]bool, 150) }
	sideCounts := make([]int, 0, 5)
	areas := make([]int, 0, 5)
	for i := range garden {
		for j := range garden[i] {
			if seen[i][j] { continue }
			perim, area := 0, 0
			walk2(garden, seen, &perim, &area, i, j) 
			sideCounts = append(sideCounts, perim)
			areas = append(areas, area)
		}
	}
	for i := range sideCounts {
		sum += sideCounts[i] * areas[i]
	}
	fmt.Println(sum)
}

func walk2(garden [][]byte, seen [][]bool, sideCount *int, area *int, i int, j int) {
	seen[i][j] = true
	*area += 1
	ch := garden[i][j]
	for _, corner := range corners {
		x1, y1 := i + corner[0], j
		x2, y2 := i, j + corner[1]
		if (x1 < 0 || x1 >= len(garden) || garden[x1][y1] != ch) && 
			(y2 < 0 || y2 >= len(garden[0]) || garden[x2][y2] != ch) {
			*sideCount += 1
		}
		if x1 >= 0 && x1 < len(garden) && y1 >= 0 && y1 < len(garden[0]) && garden[x1][y1] == ch &&
			x2 >= 0 && x2 < len(garden) && y2 >= 0 && y2 < len(garden[0]) && garden[x2][y2] == ch &&
			garden[x1][y2] != ch {
			*sideCount += 1
		}
	}
	for _, dir := range dirs {
		x, y := i + dir[0], j + dir[1]
		// If still within region, RECURSE
		if !(x < 0 || x >= len(garden) || y < 0 || y >= len(garden[0]) || garden[x][y] != ch || seen[x][y]) {
			walk2(garden, seen, sideCount, area, x, y)
		} 
	}
}
