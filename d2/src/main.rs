fn main() {
    // A, X - Rock
    // B, Y - Paper
    // C, Z - Scissor
    // Part 1
    //let sum: i32 = include_str!("input")
    //    .split("\n")
    //    .map(|x|{
    //        let line = x.split(" ").collect::<Vec<&str>>();            
    //        if line.len() == 2{
    //            let adder = match line[1] {
	//	            "X" => 1,
	//	            "Y" => 2,
    //                "Z" => 3,
    //                &_ => 0
    //            };
    //            if (line[0] == "A" && line[1] == "X") ||
    //                (line[0] == "B" && line[1] == "Y") ||
    //                (line[0] == "C" && line[1] == "Z"){
    //                    return 3 + adder
    //            }
    //            
    //            if (line[0] == "A" && line[1] == "Z") ||
    //                (line[0] == "B" && line[1] == "X") ||
    //                (line[0] == "C" && line[1] == "Y"){
    //                return 0 + adder
    //            }
    //            
    //            if (line[0] == "A" && line[1] == "Y") ||
    //                (line[0] == "B" && line[1] == "Z") ||
    //                (line[0] == "C" && line[1] == "X"){
    //                return 6 + adder
    //            }


    //        }
    //        return 0;
    //    })
    //    .collect::<Vec<i32>>().iter().sum();
    
    // A - Rock -> 1
    // B - Paper -> 2
    // C - Scissor -> 3
    //
    // X - Lose
    // Y - Draw
    // Z - Win
    // Part 2
    let sum: i32 = include_str!("input")
        .split("\n")
        .map(|x|{
            let line = x.split(" ").collect::<Vec<&str>>();
            if line.len() == 2{
                let adder = match line[1] {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    &_ => 0
                };
                // Rock
                if (line[1] == "X" && line[0] == "B") ||
                    (line[1] == "Y" && line[0] == "A") ||
                    (line[1] == "Z" && line[0] == "C"){
                    return 1 + adder
                }
                // Paper
                if (line[1] == "X" && line[0] == "C") ||
                    (line[1] == "Y" && line[0] == "B") ||
                    (line[1] == "Z" && line[0] == "A"){
                    return 2 + adder 
                }
                // Scissor
                if (line[1] == "X" && line[0] == "A") ||
                    (line[1] == "Y" && line[0] == "C") ||
                    (line[1] == "Z" && line[0] == "B"){
                    return 3 + adder
                }
            }
            return 0
        })
    .collect::<Vec<i32>>().iter().sum();
    println!("{:?}", sum)
}
