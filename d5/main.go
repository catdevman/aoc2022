package main

import (
  "fmt"
  "regexp"
  "strings"
  "strconv"
  _ "embed"
)

var testing_stacks = [][]string{
  []string{
    "N",
    "Z",
  },
  []string{
    "D",
    "C",
    "M",
  },
  []string{
    "P",
  },
}

var stacks = [][]string{
  []string{
    "N",
    "T",
    "B",
    "S",
    "Q",
    "H",
    "G",
    "R",
  },
  []string{
    "J",
    "Z",
    "P",
    "D",
    "F",
    "S",
    "H",
  },
  []string{
    "V",
    "H",
    "Z",
  },
  []string{
    "H",
    "G",
    "F",
    "J",
    "Z",
    "M",
  },
  []string{
    "R",
    "S",
    "M",
    "L",
    "D",
    "C",
    "Z",
    "T",
  },
  []string{
    "J",
    "Z",
    "H",
    "V",
    "W",
    "T",
    "M",
  },
  []string{
    "Z",
    "L",
    "P",
    "F",
    "T",
  },
  []string{
    "S",
    "W",
    "V",
    "Q",
  },
  []string{
    "C",
    "N",
    "D",
    "T",
    "M",
    "L",
    "H",
    "W",
  },
}

//go:embed input
var content string

func main(){
  lines := strings.Split(strings.TrimSpace(content), "\n")
  re := regexp.MustCompile("[0-9]+")
  for _, line := range lines{
    nums := re.FindAllString(line, -1)
    if len(nums) != 3{
      panic("input for moves is wrong")
    }
    count, _ := strconv.Atoi(nums[0])
    c1, _ := strconv.Atoi(nums[1])
    c2, _ := strconv.Atoi(nums[2])
    var tmp_stacks = make([][]string, len(stacks))
    copy(tmp_stacks, stacks)
    var tc1 = tmp_stacks[c1-1]
    var tc2 = tmp_stacks[c2-1]
    if count > len(tc1) {
      count = len(tc1)
    }
    var add = tc1[:count]
    // for part1 remove one of the Reverse func calls
    tc2 = append(Reverse(Reverse(add)), tc2...)
    tc1 = tc1[count:]
    //fmt.Println(count, tc1, tc2, add)
    tmp_stacks[c1-1] = tc1
    tmp_stacks[c2-1] = tc2
    fmt.Println(stacks, tmp_stacks)
    copy(stacks, tmp_stacks)
    fmt.Println(stacks)
  }
  for _, s := range stacks{
    if len(s) >= 1{
      fmt.Print(s[0])
    }
  }
}

func Reverse(input []string) []string{
	inputLen := len(input)
	output := make([]string, inputLen)

	for i, n := range input {
		j := inputLen - i - 1

		output[j] = n
	}

	return output
}
