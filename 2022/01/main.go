package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type elf struct {
	no       int
	calories float64
}

func main() {
	f, err := os.Open("./2022/01/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	counter := 1
	current := elf{no: counter, calories: 0}
	highest := []elf{
		{}, {}, {},
	}

	test := func() {
		if current.calories > highest[2].calories {
			highest[0] = highest[1]
			highest[1] = highest[2]
			highest[2] = current
		} else if current.calories > highest[1].calories {
			highest[0] = highest[1]
			highest[1] = current
		} else if current.calories > highest[0].calories {
			highest[0] = current
		}
	}

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			counter++
			test()
			current = elf{no: counter}
			// fmt.Println(highest)
		} else {
			if s, err := strconv.ParseFloat(line, 32); err == nil {
				current.calories += s
			}
		}
	}
	test()
	fmt.Println(highest)
	result := 0
	for _, v := range highest {
		result += int(v.calories)
	}
	fmt.Println(result)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
