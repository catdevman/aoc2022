fn main() {
    // A, X - Rock
    // B, Y - Paper
    // C, Z - Scissor
    let sum: i32 = include_str!("input")
        .split("\n")
        .map(|x|{
            let line = x.split(" ").collect::<Vec<&str>>();            
            if line.len() == 2{
                let adder = match line[1] {
		            "X" => 1,
		            "Y" => 2,
                    "Z" => 3,
                    &_ => 0
                };
                if (line[0] == "A" && line[1] == "X") ||
                    (line[0] == "B" && line[1] == "Y") ||
                    (line[0] == "C" && line[1] == "Z"){
                        return 3 + adder
                }
                
                if (line[0] == "A" && line[1] == "Z") ||
                    (line[0] == "B" && line[1] == "X") ||
                    (line[0] == "C" && line[1] == "Y"){
                    return 0 + adder
                }
                
                if (line[0] == "A" && line[1] == "Y") ||
                    (line[0] == "B" && line[1] == "Z") ||
                    (line[0] == "C" && line[1] == "X"){
                    return 6 + adder
                }


            }
            return 0;
        })
        .collect::<Vec<i32>>().iter().sum();
    println!("{:?}", sum)
}
