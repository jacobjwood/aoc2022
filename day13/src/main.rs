use std::fs;
use std::collections::VecDeque;

fn parse_line(line : &str) -> Vec<String> {

    println!("LINE {}", line);

    let split_line = line
        //.replace("[]", "[_]")
        .replace("]", ",]")
        .split_inclusive(&['[', ']', ','])
        .filter(|x| *x != ",")
        .map(|x| x.replace(",", ""))
        .collect::<Vec<String>>();

    /*
    let mut depth_line : VecDeque<(String, usize)> = VecDeque::new();

    let mut depth = 0;

    for item in split_line {
        match &item[..] {
            "]" => depth -= 1,
            "[" => depth += 1,
            _ => depth_line.push_back((item, depth)),
        }
    }
    
    */
    split_line

    
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

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

    let mut index = 1;
    let mut count = 0;

    let mut packet_vec : Vec<Vec<String>> = Vec::new();
    packet_vec.push(parse_line(&"[[2]]"));
    packet_vec.push(parse_line(&"[[6]]"));

    for pair_lines in contents.split("\n\n") {

        println!("{}", "=".repeat(40));
        println!("NEW LINE");
        println!("{}", "=".repeat(40));
        let pairs = pair_lines.split("\n").collect::<Vec<_>>();
        let mut left = parse_line(pairs[0]);
        let mut right = parse_line(pairs[1]);

        println!("left {:?}", left);
        println!("right {:?}", right);

        let result = basic_recursion(&mut left, &mut right, &mut 0, &mut 0);

        println!("RESULT: {}", result);
        if result {
            count += index;
            packet_vec.push(right);
            packet_vec.push(left);
        } else {
            packet_vec.push(left);
            packet_vec.push(right);
        }

        // std::thread::sleep_ms(10000);

        index += 1;
        //println!("PRINTING PAIRS");
        //println!("{}", pair_lines);
        /*
        let output = compare(&mut left, &mut right);
        // println!("{}", output);

        if output {
            count += index;
        };
        //println!("{:?}\n{:?}", left, right);
        // compare(&mut left, &mut right);
        index += 1;
        */
        println!("\n\n");
        
    }

    println!("Part 1: {}", count);

    println!("{:?}", packet_vec);

    packet_vec.sort_by(|v1, v2| (basic_recursion(&mut v2.to_owned(), &mut v1.to_owned(), &mut 0, &mut 0) as usize).cmp(&(basic_recursion(&mut v1.to_owned(), &mut v2.to_owned(), &mut 0, &mut 0) as usize)));

    let pv = packet_vec.into_iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<String>>();

    println!("{:#?}", pv);

    let mut pac2 = 0;
    let mut pac6 = 0;

    for (idx, pac) in pv.iter().enumerate() {
        match &pac[..] {
            "[[2]]" => pac2 = idx + 1,
            "[[6]]" => pac6 = idx + 1,
            _ => (),
        }
    }

    println!("{}", pac2 * pac6);



}

// so recursion does work
fn basic_recursion(left: &mut Vec<String>, right: &mut Vec<String>, l_idx: &mut usize, r_idx: &mut usize) -> bool {

    //let item1 = vec.pop_front().unwrap();
    //let item2 = vec1.pop_front().unwrap();
    let mut left_val = &left[*l_idx];
    let mut right_val = &right[*r_idx];
    
    println!("LEFT_VAL {} RIGHT VAL {}", left_val, right_val);

    let left_is_int = match left_val.as_str() {
        "[" => false,
        "]" => false,
        _ => true
    };

    let right_is_int = match right_val.as_str() {
        "[" => false,
        "]" => false,
        _ => true
    };

    if left_is_int && right_is_int { // both are ints
        if left_val == right_val { // ints equal
            *l_idx += 1;
            *r_idx += 1;
            return basic_recursion(left, right, l_idx, r_idx);
        } else {
            return left_val.parse::<usize>().unwrap() < right_val.parse::<usize>().unwrap();
        }
    } else if *left_val == "[" && *right_val == "[" { // both are lists
        *l_idx += 1;
        *r_idx += 1;
        return basic_recursion(left, right, l_idx, r_idx);
    } else if *left_val == "]" && *right_val != "]" { // left runs out of items before 
        return true;
    } else if *right_val == "]" && *left_val != "]" { // right runs out of items before
        return false;
    } else if *right_val == "]" && *left_val == "]" {
        *l_idx += 1;
        *r_idx += 1;
        return basic_recursion(left, right, l_idx, r_idx);
    // THESE FEW WILL BE TRICKY
    } else if left_is_int && *right_val == "[" {
        let lv = left_val.to_owned();
        left.insert(*l_idx+1, "]".to_string());
        left.insert(*l_idx, "[".to_string());
        // *l_idx += 1;
        // *r_idx += 1;
        // *l_idx -= 1;
        return basic_recursion(left, right, l_idx, r_idx);
    } else if right_is_int && *left_val == "[" {
        let rv = right_val.to_owned();
        println!("HERE JEFF");
        right.insert(*r_idx+1, "]".to_string());
        right.insert(*r_idx, "[".to_string());
        println!("RIGHT AFTER {:?}", right);
        // *l_idx += 1;
        // *r_idx -= 1;
        return basic_recursion(left, right, l_idx, r_idx);
    } else {
        // This should never be met
        println!("IM HERE");
        std::thread::sleep_ms(40000000);
        return false;
    }
}

fn sort_vecs(vec1: &Vec<String>, vec2: &Vec<String>) {
    let mut vec1_copy = vec1.to_owned();
    let mut vec2_copy = vec2.to_owned();
}