use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let tree_array : Vec<Vec<u32>> = contents.split("\n").filter(|line| !line.is_empty()).map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
    let col_len = tree_array.len();
    let row_len = tree_array[0].len();
    let mut view_vec = Vec::<u32>::new();

    let mut visible_array : Vec<Vec<u32>> = tree_array.iter().map(|x| x.iter().map(|_| 0).collect::<Vec<u32>>()).collect();

    // Part 1
    for (r_idx, _) in tree_array.iter().enumerate() {

        if (r_idx == row_len-1) | (r_idx == 0) {
            visible_array[r_idx] = visible_array[r_idx].iter().map(|_| 1).collect();
            continue;
        }

        for (c_idx, _) in tree_array[r_idx].iter().enumerate() {

            let current_tree = &tree_array[r_idx][c_idx];

            if (c_idx == col_len-1) | (c_idx == 0) {
                visible_array[r_idx][c_idx] = 1;
                continue;
            }


            let min_left = &tree_array[r_idx][..c_idx].iter().max().unwrap();
            let min_right = &tree_array[r_idx][c_idx+1..].iter().max().unwrap(); 
            let min_up = &tree_array[..r_idx].iter().map(|x| x[c_idx]).max().unwrap();
            let min_down = &tree_array[r_idx+1..].iter().map(|x| x[c_idx]).max().unwrap();

            if (current_tree > min_left) | (current_tree > min_right) | (current_tree > min_up) | (current_tree > min_down) {
                visible_array[r_idx][c_idx] = 1;
            }
            

        }
    }


    // Part 2
    for (r_idx, row) in tree_array.iter().enumerate() {
        for (c_idx, _) in row.iter().enumerate() {

            let current_tree = &tree_array[r_idx][c_idx];
            let left_vec : Vec<&u32> = tree_array[r_idx][..c_idx].iter().rev().collect();
            let right_vec : Vec<&u32> = tree_array[r_idx][c_idx+1..].iter().collect();
            let up_vec : Vec<&u32> = tree_array[..r_idx].iter().map(|x| &x[c_idx]).rev().collect();
            let down_vec : Vec<&u32> = tree_array[r_idx+1..].iter().map(|x| &x[c_idx]).collect();

            let min_left = folding_func(&left_vec, &current_tree);
            let min_right = folding_func(&right_vec, &current_tree);
            let min_up = folding_func(&up_vec, &current_tree);
            let min_down = folding_func(&down_vec, &current_tree);

            view_vec.push(min_down*min_left*min_right*min_up);
        }

    }
    let sum : u32 = visible_array.iter().flatten().sum();
    println!("Part 1: {:#?}", sum);
    let max_score : &u32 = view_vec.iter().max().unwrap();
    println!("Part 2: {}", max_score);
}


fn add_it(not_blocked: &bool) -> u32 {
    if *not_blocked { 1 } else { 0 }
}


fn folding_func<'a>(vec_ref: &'a Vec<&u32>, current_tree: &u32) -> u32 {
    vec_ref
        .iter() // the iterator is created from the reference which is a slice and I need to understand this better
        .cloned() // the iterator creates an iterator over references to &u32, so cloned helps us iterate over &u32 instead of &&u32
        .fold(
            (true, 0), 
            |(nb, count), tree| (nb & (tree < current_tree), count + add_it(&nb))
        )
        .1
}



