use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n").filter(|x| !x.is_empty());
    let pairs = lines.map(|line| line.split(",").map(|range_str| range_str.split("-").map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<_>>());
    let mut containment_count = 0;
    let mut overlap_count = 0;
    for p in pairs { 
	if check_closure(&p) {containment_count += 1}
	if check_overlap(&p) {overlap_count += 1}
    }
    println!("Part 1: {}\nPart 2: {}", containment_count, overlap_count);
}

fn check_closure(vec_pair: &Vec<Vec<i32>>) -> bool {
    
    let first_pair = &vec_pair[0];
    let second_pair = &vec_pair[1];

    if first_pair[0] < second_pair[0] { first_pair[1] >= second_pair[1] }
    else if first_pair[0] > second_pair[0] { first_pair[1] <= second_pair[1] }
    else { true }
}

fn check_overlap(vec_pair: &Vec<Vec<i32>>) -> bool {
    
    let first_pair = &vec_pair[0];
    let second_pair = &vec_pair[1];

   ((first_pair[0] <= second_pair[1]) && (second_pair[0] <= first_pair[1])) || ((first_pair[0] >= second_pair[1]) && (second_pair[0] >= first_pair[1]))
}
