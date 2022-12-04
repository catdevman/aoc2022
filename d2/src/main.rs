fn main() {
    // A, X - Rock
    // B, Y - Paper
    // C, Z - Scissor
    let sum: i32 = include_str!("input")
        .split("\n")
        .map(|x|{
            let line = x.split(" ").collect::<Vec<&str>>();            
            if line.len() == 2{
                if (line[0] == "A" && line[1] == "X") ||
                    (line[0] == "B" && line[1] == "Y") ||
                    (line[0] == "C" && line[1] == "Z"){
                    return 3 + 
                }
                
                if (line[0] == "A" && line[1] == "Z") ||
                    (line[0] == "B" && line[1] == "X") ||
                    (line[0] == "C" && line[1] == "Y"){
                    return 0
                }
                
                if (line[0] == "A" && line[1] == "Y") ||
                    (line[0] == "B" && line[1] == "Z") ||
                    (line[0] == "C" && line[1] == "X"){
                    return 6
                }


            }
            return 0;
        })
        .collect::<Vec<i32>>().iter().sum();
    println!("{:?}", sum)
}
