use std::fs;

fn parse_line(line: &str) -> Vec<String> {
    let split_line = line
        .replace("]", ",]")
        .split_inclusive(&['[', ']', ','])
        .filter(|x| *x != ",")
        .map(|x| x.replace(",", ""))
        .collect::<Vec<String>>();

    split_line
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut index = 1;
    let mut count = 0;

    let mut packet_vec: Vec<Vec<String>> = Vec::new();
    packet_vec.push(parse_line(&"[[2]]"));
    packet_vec.push(parse_line(&"[[6]]"));

    for pair_lines in contents.split("\n\n") {
        let pairs = pair_lines.split("\n").collect::<Vec<_>>();
        let mut left = parse_line(pairs[0]);
        let mut right = parse_line(pairs[1]);

        let result = basic_recursion(&mut left, &mut right, &mut 0, &mut 0);

        if result {
            count += index;
            packet_vec.push(right);
            packet_vec.push(left);
        } else {
            packet_vec.push(left);
            packet_vec.push(right);
        }

        index += 1;
    }

    println!("Part 1: {}", count);

    packet_vec.sort_by(|v1, v2| {
        (basic_recursion(&mut v2.to_owned(), &mut v1.to_owned(), &mut 0, &mut 0) as usize).cmp(
            &(basic_recursion(&mut v1.to_owned(), &mut v2.to_owned(), &mut 0, &mut 0) as usize),
        )
    });

    let pv = packet_vec
        .into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut pac2 = 0;
    let mut pac6 = 0;

    for (idx, pac) in pv.iter().enumerate() {
        match &pac[..] {
            "[[2]]" => pac2 = idx + 1,
            "[[6]]" => pac6 = idx + 1,
            _ => (),
        }
    }

    println!("Part 2: {}", pac2 * pac6);
}

fn basic_recursion(
    left: &mut Vec<String>,
    right: &mut Vec<String>,
    l_idx: &mut usize,
    r_idx: &mut usize,
) -> bool {
    let left_val = &left[*l_idx];
    let right_val = &right[*r_idx];

    let left_is_int = match left_val.as_str() {
        "[" => false,
        "]" => false,
        _ => true,
    };

    let right_is_int = match right_val.as_str() {
        "[" => false,
        "]" => false,
        _ => true,
    };

    if left_is_int && right_is_int {
        // both are ints
        if left_val == right_val {
            // ints equal
            *l_idx += 1;
            *r_idx += 1;
            return basic_recursion(left, right, l_idx, r_idx);
        } else {
            return left_val.parse::<usize>().unwrap() < right_val.parse::<usize>().unwrap();
        }
    } else if *left_val == "[" && *right_val == "[" {
        // both are lists
        *l_idx += 1;
        *r_idx += 1;
        return basic_recursion(left, right, l_idx, r_idx);
    } else if *left_val == "]" && *right_val != "]" {
        // left runs out of items before
        return true;
    } else if *right_val == "]" && *left_val != "]" {
        // right runs out of items before
        return false;
    } else if *right_val == "]" && *left_val == "]" {
        *l_idx += 1;
        *r_idx += 1;
        return basic_recursion(left, right, l_idx, r_idx);
    } else if left_is_int && *right_val == "[" {
        left.insert(*l_idx + 1, "]".to_string());
        left.insert(*l_idx, "[".to_string());
        return basic_recursion(left, right, l_idx, r_idx);
    } else if right_is_int && *left_val == "[" {
        right.insert(*r_idx + 1, "]".to_string());
        right.insert(*r_idx, "[".to_string());
        return basic_recursion(left, right, l_idx, r_idx);
    } else {
        // This should never be met
        println!("IM HERE");
        return false;
    }
}
