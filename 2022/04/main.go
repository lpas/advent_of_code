package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("./04/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	count := 0
	for scanner.Scan() {
		line := scanner.Text()
		pairs := strings.Split(line, ",")
		first := strings.Split(pairs[0], "-")
		second := strings.Split(pairs[1], "-")
		a, _ := strconv.ParseInt(first[0], 10, 64)
		b, _ := strconv.ParseInt(first[1], 10, 64)
		m, _ := strconv.ParseInt(second[0], 10, 64)
		n, _ := strconv.ParseInt(second[1], 10, 64)
		// fmt.Println(line, a, b, m, n)
		// if a >= m && b <= n || m >= a && n <= b { // step1
		// 	count++
		// 	// fmt.Println(pairs)
		// }

		//    |-----|
		//       |-----|
		//  |---|
		if a <= n && b >= m {
			count++
		}

		// fmt.Println(line, pairs)
	}
	fmt.Println(count)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
