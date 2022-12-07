fn main() {
    let answer = include_str!("input")
        .split("\n")
        .map(|x| x
            .split(",")
            .collect::<Vec<&str>>()
            .map(|y| y.
                split("-")
                .collect::<Vec<&str>>()
            )
    ).collect::<Vec<Vec<&str>>>();
    println!("{:?}", answer);
}
