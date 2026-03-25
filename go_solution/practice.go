package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strconv"
)

func main() {
	fmt.Print("insert amount here:  ")
	input := bufio.NewScanner(os.Stdin)
	input.Scan()
	fmt.Println("Input is ", input.Text())

	amount, err := strconv.Atoi(input.Text())
	if err != nil {
		fmt.Println("Entered amount must be an integer . Encountered error: ", err.Error())
	} else {
		result, err := FindStockValuesForAmount(amount)
		if err != nil {
			fmt.Println("Encountered error ", err.Error())
		} else {
			jsonString, _ := json.Marshal(result)
			fmt.Println("result stock values ", string(jsonString))
		}
	}

}

func FindStockValuesForAmount(amount int) (map[int]int, error) {
	if amount == 0 || amount < 0 {
		return nil, errors.New("invalid amount")
	}

	var returnValue map[int]int
	returnValue = make(map[int]int)

	stock := map[int]int{
		10000: 1,
		5000:  2,
		1000:  5,
		500:   3,
		100:   10,
		50:    10,
		10:    10,
		5:     10,
		1:     10,
	}

	denominations := []int{10000, 5000, 1000, 500, 100, 50, 10, 5, 1}

	for _, k := range denominations {
		if amount >= k {
			q := amount / k
			value := min(q, stock[k])
			amount = amount - (k * value)
			returnValue[k] = value
		}
	}

	if amount == 0 {
		return returnValue, nil
	}

	return nil, errors.New("Amount value is out of stock")
}
