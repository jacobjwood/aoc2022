use std::fs;
use std::collections::VecDeque;

fn parse_line(line : &str) -> VecDeque<(String, usize)> {

    println!("LINE {}", line);

    let split_line = line[1..line.len()-1]
        .replace("[]", "[_]")
        .replace("]", ",]")
        .split_inclusive(&['[', ']', ','])
        .filter(|x| *x != ",")
        .map(|x| x.replace(",", ""))
        .map(|x| x.replace("$", ""))
        .collect::<VecDeque<String>>();

    let mut depth_line : VecDeque<(String, usize)> = VecDeque::new();

    let mut depth = 0;

    for item in split_line {
        match &item[..] {
            "]" => depth -= 1,
            "[" => depth += 1,
            _ => depth_line.push_back((item, depth)),
        }
    }
    

    depth_line

    
}

/*
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
*/

fn compare(left: &mut VecDeque<(String, usize)>, right: &mut VecDeque<(String, usize)>) -> bool {
    println!("LEFT {:?}", left);
    println!("RIGHT {:?}", right);

    let mut max_depth = 0;

    while !right.is_empty() && !left.is_empty() {
        let (l_item, l_depth) = left.pop_front().unwrap();
        let (r_item, r_depth) = right.pop_front().unwrap();

        let mut prev_depth = 0;
        let tmp_max_depth = std::cmp::max(l_depth, r_depth);

        if l_depth == r_depth {
            if l_item == r_item { 
                max_depth = tmp_max_depth;
                continue; 
            }
            else if l_item != "_" && r_item != "_" {
                return l_item < r_item;
            }
        } else {
            if r_depth == max_depth && l_depth < max_depth {
                return true;
            } else if r_depth < max_depth && l_depth == max_depth {
                return false;
            } else {

                if r_item == l_item {
                    max_depth = tmp_max_depth;
                    println!("HERE");
                    continue;
                } else {
                    return r_item > l_item;
                }
            }
        }
        /* 
        if r_item == "_" && l_item != "_" {
            println!("WRONG ORDER");
            return false;
        } else {
            if r_item > l_item {
                println!("RIGHT ORDER");
                return true;
            } else if l_item > r_item {
                println!("WRONG ORDER");
                return false;
            } else {
                println!("{} {}", l_item, r_item);
                if l_item == "_" {
                    if l_depth > r_depth {
                        println!("WRONG ORDER");
                        return false;
                    }
                }
                continue;
            }
        }

        */
    }


    match !left.is_empty() {
        true => {
            println!("WRONG ORDER");
            false
        },
        false => {
            println!("RIGHT ORDER");
            true
        }
    }
    
}

fn main() {
    let file_path = "input_test.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

    let mut index = 1;
    let mut count = 0;

    for pair_lines in contents.split("\n\n") {
        let pairs = pair_lines.split("\n").collect::<Vec<_>>();
        let mut left = parse_line(pairs[0]);
        let mut right = parse_line(pairs[1]);
        //println!("PRINTING PAIRS");
        //println!("{}", pair_lines);
        let output = compare(&mut left, &mut right);
        println!("{}", output);

        if output {
            count += index;
        };
        //println!("{:?}\n{:?}", left, right);
        // compare(&mut left, &mut right);
        index += 1;

        
    }

    println!("{}", count);
}

// so recursion does work
fn basic_recursion(vec: &mut Vec<String>, vec1: &mut Vec<String>) -> bool {
    let item1 = vec.pop().unwrap();
    let item2 = vec1.pop().unwrap();

    if item1 == "]".to_string() {
        return true
    } else {
        basic_recursion(vec, vec1)
    }
}