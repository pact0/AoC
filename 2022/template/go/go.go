package main

import (
	"bufio"
	"log"
	"os"
)

func main() {
	input := readInput("../input")

  println("part 1 ",part1(input))
  println("part 2 ",part2(input))
}

func part2(data []string) int {
  return 0;
}

func part1(data []string) int {
  return 0;
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
