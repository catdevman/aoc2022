package main

import(
  "strings"
  "strconv"
  "fmt"
  _ "embed"
)

//go:embed input
var content string

func main(){
  lines := strings.Split(strings.TrimSpace(content), "\n")
  part1 := 0
  part2 := 0
  for _, line := range lines {
    ranges := strings.Split(line, ",")
    if len(ranges) == 2{
      group1 := strings.Split(ranges[0], "-")
      group2 := strings.Split(ranges[1], "-")
      s1, _ := strconv.Atoi(group1[0])
      e1, _ := strconv.Atoi(group1[1])
      s2, _ := strconv.Atoi(group2[0])
      e2, _ := strconv.Atoi(group2[1])
      if s1 <= s2 && e2 <= e1 || s2 <= s1 && e1 <= e2 {
        part1 += 1
      }
      if (s2 <= s1 && s1 <= e2 || s2 <= e1 && e1 <= e2) ||
			(s1 <= s2 && s2 <= e1 || s1 <= e2 && e2 <= e1) {
        part2 += 1
      }
    }
  }
  fmt.Println("part1 =", part1, "part2 = ", part2)
}
