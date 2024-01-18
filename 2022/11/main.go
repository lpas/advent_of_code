package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("./2022/11/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	type Operation struct {
		operation string
		value     int64 // -1 is old value
	}
	type Monkey struct {
		items     []int64
		operation Operation
		testValue int64
		iftrue    int64
		iffalse   int64
		inspects  int64
	}

	var monkeys []Monkey
	var value int64
	for scanner.Scan() {
		monkey := Monkey{}
		scanner.Text() // monkey X:

		scanner.Scan()
		fields := strings.Fields(scanner.Text()) //  Starting items: 54, 65, 75, 74
		for _, v := range fields[2:] {
			value, _ = strconv.ParseInt(strings.Trim(v, ","), 10, 64)
			monkey.items = append(monkey.items, value)
		}

		scanner.Scan()
		fields = strings.Fields(scanner.Text()) // Operation: new = old + 6
		value = int64(-1)
		if fields[5] != "old" {
			value, _ = strconv.ParseInt(fields[5], 10, 64)
		}
		monkey.operation = Operation{operation: fields[4], value: value}

		scanner.Scan()
		fields = strings.Fields(scanner.Text()) //   Test: divisible by 13
		value, _ = strconv.ParseInt(fields[3], 10, 64)
		monkey.testValue = value

		scanner.Scan()
		fields = strings.Fields(scanner.Text()) //       If true: throw to monkey 1
		value, _ = strconv.ParseInt(fields[5], 10, 64)
		monkey.iftrue = value

		scanner.Scan()
		fields = strings.Fields(scanner.Text()) //       If false: throw to monkey 3
		value, _ = strconv.ParseInt(fields[5], 10, 64)
		monkey.iffalse = value

		scanner.Scan()
		scanner.Text() // empty

		monkeys = append(monkeys, monkey)
	}

	// for _, monkey := range monkeys {
	// 	fmt.Println(monkey.items)
	// }

	operate := func(value int64, operation Operation) (result int64) {
		if operation.value == -1 {
			result = value
		} else {
			result = operation.value
		}

		switch operation.operation {
		case "*":
			result *= value
		case "+":
			result += value
		}

		return result
	}
	rounds := 10000

	commonDenominator := int64(1)
	for _, monkey := range monkeys {
		commonDenominator *= monkey.testValue
	}

	for j := 0; j < rounds; j++ {
		for i := range monkeys {
			monkey := &monkeys[i]
			for len(monkey.items) > 0 {
				monkey.inspects++
				item := operate(monkey.items[0], monkey.operation)
				// item /= 3 // step1
				if item > commonDenominator {
					item %= commonDenominator
				}

				monkey.items = monkey.items[1:]
				var toMonkey int64
				if item%monkey.testValue == 0 {
					toMonkey = monkey.iftrue
				} else {
					toMonkey = monkey.iffalse
				}
				monkeys[toMonkey].items = append(monkeys[toMonkey].items, item)

				// fmt.Println(item, toMonkey)

			}
		}
	}
	for _, monkey := range monkeys {
		fmt.Println(monkey.items, monkey.inspects)
	}
	sort.Slice(monkeys, func(i, j int) bool {
		return monkeys[i].inspects > monkeys[j].inspects
	})
	fmt.Println(monkeys[0].inspects * monkeys[1].inspects)
	// for _, monkey := range monkeys {
	// fmt.Println(monkey.items, monkey.inspects)
	// }
	// fmt.Println(monkeys)

	// fmt.Printf("%#v", monkeys)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
