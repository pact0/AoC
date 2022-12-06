package main

import (
	"bufio"
	"log"
	"os"
)

func main() {
	// input := readInput("../input")
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
