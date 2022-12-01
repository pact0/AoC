package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
  sums := []int{0}
  var length int = 0

	input := readInput("../input")

	for _, line := range input {
		if line != "" {
      var n, _ = strconv.Atoi(line)
      sums[length] = sums[length] + n
		} else {
      sums = append(sums, 0)
      length += 1
    }
	}
	sort.Sort(sort.Reverse(sort.IntSlice(sums[:])))

  println("part 1 ", sums[0])
  println("part 2 ", sums[0] + sums[1] + sums[2])
    
}

func readInput(file string) []string {
	file_contents, err := os.Open(file)

	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file_contents)

	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines
}
