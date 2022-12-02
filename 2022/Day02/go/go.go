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
  var score int = 0
  for _, line := range data {
    switch line {
    case "A X":
      score += 3
    case "A Y":
      score += 4
    case "A Z":
      score += 8
    case "B X":
      score += 1
    case "B Y":
      score += 5
    case "B Z":
      score += 9
    case "C X":
      score += 2
    case "C Y":
      score += 6
    case "C Z":
      score += 7
    }
    
  }
  return score
}

func part1(data []string) int {
  var score int = 0
 for _, line := range data {
    switch (line) {
    case "A X":
      score += 1 + 3;
      break;
    case "B Y":
      score += 2 + 3;
      break;
    case "C Z":
      //draw
      score += 3 + 3;
      break;
    case "A Y":
      score += 6 + 2;
      break;
    case "B Z":
      score += 6 + 3;
      break;
    case "C X":
      score += 6 + 1;
      break;
    case "A Z":
      score += 0 + 3;
      break;
    case "C Y":
      score += 0 + 2;
      break;
    case "B X":
      score += 0 + 1;
  }
  }

  return score
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
