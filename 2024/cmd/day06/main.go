package main

import (
	"fmt"
	"os"
	"strings"
	// "strconv"
)

func main() {
	layout := parseInput()
	part2(layout)
}

func parseInput() [][]byte {
	content, err := os.ReadFile("input/day06.txt")
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

func part1(layout [][]byte) {
	var y, x int
	count := 0
	dirs := [4][2]int {{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	dir_index := 0
	outer: for i, row := range layout {
		for j, pos := range row {
			if pos == '^' {
				y = i
				x = j
				break outer
			}
		}
	}
	layout[y][x] = '.'
	for true {
		curr_pos := layout[y][x]
		if curr_pos == '.' {
			layout[y][x] = 'X'
			count += 1
		} else if curr_pos == '#' {
			y -= dirs[dir_index][0]
			x -= dirs[dir_index][1]
			dir_index = (dir_index + 1) % 4
		}
		y += dirs[dir_index][0]
		x += dirs[dir_index][1]
		if y < 0 || y >= len(layout) || x < 0 || x >= len(layout[0]) {
			break
		}
	}
	fmt.Println(count)
}

func part2(layout [][]byte) {
	var j, i int
	dirs := [4][2]int {{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	dir_index := 0
	j, i = findStart(layout)
	start_pos := [2]int {j, i}
	layout[j][i] = '.'
	count := 0
	for true {
		pos := layout[j][i]
		if pos == '.' {
			layout[j][i] = 'X'
		} else if pos == '#' {
			j -= dirs[dir_index][0]
			i -= dirs[dir_index][1]
			dir_index = (dir_index + 1) % 4
		}
		j += dirs[dir_index][0]
		i += dirs[dir_index][1]
		if j < 0 || j >= len(layout) || i < 0 || i >= len(layout[0]) {
			break
		}
		if layout[j][i] == 'X'|| layout[j][i] == '#' { continue }

		original_spot := layout[j][i]
		// Treat this spot as an obstruction
		layout[j][i] = '#'
		isLoop := foundLoop(layout, start_pos)
		if isLoop {
			count += 1
		}
		layout[j][i] = original_spot
	}
	fmt.Println(count)
}

func foundLoop(layout [][]byte, start_pos [2]int) bool {
	dirs := [4][2]int {{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	var curr_states [131][131][4]int
	looped := false
	y := start_pos[0]
	x := start_pos[1]
	dir_ind := 0
	for true {
		curr_pos := layout[y][x]
		if curr_pos == '.' || curr_pos == 'X' {
			curr_states[y][x][dir_ind] = 1
		} else if curr_pos == '#' {
			y -= dirs[dir_ind][0]
			x -= dirs[dir_ind][1]
			dir_ind = (dir_ind + 1) % 4
		}
		y += dirs[dir_ind][0]
		x += dirs[dir_ind][1]
		if y < 0 || y >= len(layout) || x < 0 || x >= len(layout[0]) {
			break
		}
		if curr_states[y][x][dir_ind] == 1 {
			// fmt.Printf("The obstruction at y: %d, x %d creates a loop\n", j, i)
			looped = true
			break
		}
	}
	return looped
}

func findStart(layout [][]byte) (int, int) {
	var y, x int
	outer: for i, row := range layout {
		for j, pos := range row {
			if pos == '^' {
				y = i
				x = j
				break outer
			}
		}
	}
	return y, x
}
