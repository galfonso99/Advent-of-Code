// For average to smaller files use os.ReadFile() (Simple, Best for Smaller Files):
package main

import (
    "fmt"
    "os"
    "strings"
)

func main() {
	content, err := os.ReadFile("input/day01.txt")
    if err != nil { panic(err) }
    lines := strings.Split(string(content), "\n")
    
    for _, line := range lines {
        fmt.Println(line)
    }
}
// For unusually bigger files use bufio.Scanner() (Memory Efficient, Best for Large Files):
package main
import (
    "bufio"
    "fmt"
    "os"
)
func main() {
    file, err := os.Open("input/day01.txt")
    if err != nil { panic(err) }
    defer file.Close()
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := scanner.Text()
        fmt.Println(line)
    }
    if err := scanner.Err(); err != nil {
        panic(err)
    }
}

// strings.TrimSpace() is useful for cleaning input lines
// For numeric inputs, use strconv.Atoi() to convert strings to integers
// Always include error handling
// Use defer file.Close() to ensure file is closed
