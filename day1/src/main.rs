use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n\n");
    let mut split_vec = split
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();

    split_vec.sort_by(|a, b| b.cmp(a));
    
    let part1: i32 = split_vec[0];

    let part2: i32 = split_vec[0..3].iter().sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
