package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("input/day04.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
    lines := strings.Split(trimmed, "\n")
	byteArray := make([][]byte, len(lines))
	for i, str := range lines {
		byteArray[i] = []byte(str)
	}
	part2(byteArray)
}

func part1(arr [][]byte) {
	coords := make([][2]int, 0, len(arr))
	restOfWord := "MAS"
	foundCount := 0
	dirs := [][2]int {{-1,0}, {0,1}, {1,0}, {0,-1}, {-1,1}, {1,1}, {1,-1}, {-1,-1}}
	// Find all X's
	for i := range arr {
		for j, b := range arr[i] {
			if b == 'X' {
				coords = append(coords, [2]int{i, j})
			}
		}
	}
	for _, coord := range coords {
		for _, dir := range dirs {
			completed := true
			newY := coord[0] + dir[0]
			newX := coord[1] + dir[1]
			for i := range restOfWord {
				if newY < 0 || newY >= len(arr) || newX < 0 || newX >= len(arr[0]) {
					completed = false
					break
				}
				if arr[newY][newX] != restOfWord[i] {
					completed = false
					break
				}
				newY += dir[0]
				newX += dir[1]
			}
			if completed {
				foundCount += 1
			}
		}
	}
	fmt.Println(foundCount)
}

func part2(arr [][]byte) {
	coords := make([][2]int, 0, len(arr))
	foundCount := 0
	diagonals := [2][4]int {{-1, -1, 1, 1}, {-1, 1, 1, -1}}
	// Find all A's
	for i := range arr {
		for j, b := range arr[i] {
			if b == 'A' {
				coords = append(coords, [2]int{i, j})
			}
		}
	}
	for _, coord := range coords {
		localCount := 0
		for _, dia := range diagonals {
			y1 := coord[0] + dia[0]
			x1 := coord[1] + dia[1]
			y2 := coord[0] + dia[2]
			x2 := coord[1] + dia[3]
			if y1 < 0 || y1 >= len(arr) || x1 < 0 || x1 >= len(arr[0]) || 
				y2 < 0 || y2 >= len(arr) || x2 < 0 || x2 >= len(arr[0]) {
				break
			}
			if arr[y1][x1] == 'S' && arr[y2][x2] == 'M' || arr[y1][x1] == 'M' && arr[y2][x2] == 'S' {
				localCount += 1
			}
		}
		if localCount == 2 {
			foundCount += 1
		}
	}
	fmt.Println(foundCount)
}
