use std::fs;
use std::collections::VecDeque;

fn parse_line(line : &str) -> VecDeque<String> {

    println!("{}", line);

    let split_line = line[1..line.len()-1]
        .replace("]", ",]")
        .split_inclusive(&['[', ']', ','])
        .filter(|x| *x != ",")
        .map(|x| x.replace(",", ""))
        .map(|x| x.replace("$", ""))
        .collect::<VecDeque<_>>();

    

    split_line

    
}

fn compare(left: &mut VecDeque<String>, right: &mut VecDeque<String>) {

    let mut tabs = 0;
    let mut depth = 0;
    let mut left_counter = 0;
    let mut right_counter = 0;

    while !right.is_empty() && !left.is_empty() {
        let left_item = left.pop_front().unwrap();
        let right_item = right.pop_front().unwrap();

        let l_is_l_brack = !left_item.chars().fold(true, |b, c| b & (c == '['));
        let r_is_r_brack = !right_item.chars().fold(true, |b, c| b & (c == '['));

        let l_is_r_brack = !left_item.chars().fold(true, |b, c| b & (c == '[')); 

        if l_is_l_brack & !r_is_l_brack {
            depth += 1;
            
        }

        println!("{}", left_counter);
        println!("{}{:?}", " ".repeat(tabs*5), left);

        // println!("{} {}", left_item, l_is_brack);
        // println!("{} {}", right_item, r_is_brack);
    }

    if !left.is_empty() && right.is_empty() {
        println!("WRONG -> Right ran out of items before left")
    }
}

fn main() {
    let file_path = "input_test.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

    for pair_lines in contents.split("\n\n") {
        let pairs = pair_lines.split("\n").collect::<Vec<_>>();
        let mut left = parse_line(pairs[0]);
        let mut right = parse_line(pairs[1]);
        //println!("{:?}", left);
        //println!("{:?}", right);

        //println!("{:?}\n{:?}", left, right);
        compare(&mut left, &mut right);

        
    }    
}
