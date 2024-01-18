package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	f, err := os.Open("./2022/02/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	const (
		Rock     = 1
		Paper    = 2
		Scissors = 3
	)

	elf, me, score := 0, 0, 0
	for scanner.Scan() {
		line := scanner.Text()
		switch line[0] {
		case 'A':
			elf = Rock
		case 'B':
			elf = Paper
		case 'C':
			elf = Scissors
		}

		switch line[2] {
		case 'X': // lose
			switch elf {
			case Rock:
				me = Scissors
			case Paper:
				me = Rock
			case Scissors:
				me = Paper
			}
		case 'Y':
			me = elf
			score += 3
		case 'Z':
			switch elf {
			case Rock:
				me = Paper
			case Paper:
				me = Scissors
			case Scissors:
				me = Rock
			}
			score += 6
		}
		score += me
		// fmt.Println(me, elf, score)
	}
	fmt.Println(score)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
