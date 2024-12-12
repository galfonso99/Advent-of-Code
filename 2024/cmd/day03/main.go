package main

import (
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func main() {
	content, err := os.ReadFile("input/day03.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	part1(string(content))
}

func part1(str string) {
	sum := 0
	a := 0
	b := 0
	sample := "mul(a,b)"
	sequence := 0
	i := 0
	for i < len(str) {
		if sequence == 7 && str[i] == ')' {
			sum += a * b
			sequence = 0
		}
		if sequence == 4 || sequence == 6 {
			strVal := ""
			j := 0
			if !unicode.IsDigit(rune(str[i])) {
				sequence = 0
				i++
				continue
			}
			for unicode.IsDigit(rune(str[i])) && j < 3 {
				strVal += string(str[i])
				j++
				i++
			}
			val, _ := strconv.Atoi(strVal)
			if sequence == 4 {
				a = val
			} else {
				b = val
			}
			sequence++
			continue
		}
		if str[i] == sample[sequence] {
			sequence++
		} else {
			sequence = 0
		}
		i++
	}
	fmt.Println(sum)
}

func Pow(x, n int) int {
   if n == 0 { return 1 }
   if n == 1 { return x }
   y := Pow(x, n/2)
   if n % 2 == 0 { return y*y }
   return x*y*y
}
// func part2(nums [][]int) {
//
// }

