use std::fs;

fn parse_line(line : &str) -> Vec<String> {
    let split_line = line
        .split_inclusive(&['[', ']', ','])
        .filter(|x| *x != ",")
        .map(|x| x.replace(",", ""))
        .collect::<Vec<_>>();

    println!("{:?}", split_line);
    
    for item in split_line {
        println!("{}", item.chars().fold(true, |b, c| b & c.is_alphanumeric()));
    }

    Vec::<String>::new()

    
}

enum item {
    Vec(Box<item>),
    usize,
}

fn main() {
    let file_path = "input_test.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

    for pair_lines in contents.split("\n\n") {
        let pairs = pair_lines.split("\n").collect::<Vec<_>>();
        let left = parse_line(pairs[0]);
        let right = parse_line(pairs[1]);
        println!("{:?}", left);
        println!("{:?}", right);

        
    }    
}
