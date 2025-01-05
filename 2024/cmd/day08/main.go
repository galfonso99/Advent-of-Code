package main

import (
	"fmt"
	"os"
	// "strconv"
	"strings"
)

func main() {
	antennaMap := parseInput()
	part2(antennaMap)
}

func parseInput() [][]byte {
	content, err := os.ReadFile("input/day08.txt")
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

func part1(antennaMap [][]byte) {
	count := 0
	Map := make(map[byte][][2]int)
	var found [50][50]int
	for i, rows := range antennaMap {
		for j, char := range rows {
			if char == '.' { continue }
			if Map[char] == nil {
				Map[char] = make([][2]int, 0, 5)
			}
			Map[char] = append(Map[char], [2]int {i, j})
		}
	}
	for _, slice := range Map {
		for i := 0; i < len(slice); i++ {
			for j := i + 1; j < len(slice); j++ {
				offset := [2]int {slice[j][0] - slice[i][0], slice[j][1] - slice[i][1]}
				topPos := [2]int {slice[i][0] - offset[0], slice[i][1] - offset[1]}
				bottomPos := [2]int {slice[j][0] + offset[0], slice[j][1] + offset[1]}
				if topPos[0] >= 0 && topPos[0] < len(antennaMap) && topPos[1] >= 0 &&
					topPos[1] < len(antennaMap[0]) && found[topPos[0]][topPos[1]] == 0 {
					found[topPos[0]][topPos[1]] = 1
					count += 1
				}
				if bottomPos[0] >= 0 && bottomPos[0] < len(antennaMap) && bottomPos[1] >= 0 &&
					bottomPos[1] < len(antennaMap[0]) && found[bottomPos[0]][bottomPos[1]] == 0 {
					found[bottomPos[0]][bottomPos[1]] = 1
					count += 1
				}
			}
		}
	}
	fmt.Println(count)

	/* Here is my initial thought process */
	// Initially run through the map and make a hashmap from each char to all the 
	// positions where that char occurs
	// Then using those arrays loop through each possible pair and calculate if the 
	// antinodes would fit into the map, and if they do AND there isnt an antinode 
	// at that location already then add them to the count
	// return that count

	// I got it first try LETS GO, not even an error
}

func part2(antennaMap [][]byte) {
	count := 0
	Map := make(map[byte][][2]int)
	var found [50][50]int
	for i, rows := range antennaMap {
		for j, char := range rows {
			if char == '.' { continue }
			if Map[char] == nil {
				Map[char] = make([][2]int, 0, 5)
			}
			Map[char] = append(Map[char], [2]int {i, j})
		}
	}
	for _, slice := range Map {
		for i := 0; i < len(slice); i++ {
			for j := i + 1; j < len(slice); j++ {
				offset := [2]int {slice[j][0] - slice[i][0], slice[j][1] - slice[i][1]}
				topPos := [2]int {slice[i][0], slice[i][1]}
				// top antinodes
				for topPos[0] >= 0 && topPos[0] < len(antennaMap) && 
						topPos[1] >= 0 && topPos[1] < len(antennaMap[0])  {
					if found[topPos[0]][topPos[1]] == 0 {
						found[topPos[0]][topPos[1]] = 1
						count += 1
						// fmt.Printf("y = %d, x = %d\n", topPos[0], topPos[1])
					}
					topPos = [2]int {topPos[0] - offset[0], topPos[1] - offset[1]}
				}
				bottomPos := [2]int {slice[j][0], slice[j][1]}
				// bottom antinodes
				for bottomPos[0] >= 0 && bottomPos[0] < len(antennaMap) && 
						bottomPos[1] >= 0 && bottomPos[1] < len(antennaMap[0]) {
					if found[bottomPos[0]][bottomPos[1]] == 0 {
						found[bottomPos[0]][bottomPos[1]] = 1
						count += 1
						// fmt.Printf("y = %d, x = %d\n", bottomPos[0], bottomPos[1])
					}
					bottomPos = [2]int {bottomPos[0] + offset[0], bottomPos[1] + offset[1]}
				}
			}
		}
	}
	fmt.Println(count)
	// To get part2 tweak part1 slightly so as to keep finding antinodes until off boundaries
}

