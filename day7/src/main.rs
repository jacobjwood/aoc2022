use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut dir_size : HashMap<&str, i64> = HashMap::new();
    let mut dir_stack = Vec::<&str>::new();

    for line in contents.lines() {
        let line_vec = line.split(" ").collect::<Vec<&str>>();
        
        if line_vec[0] == "$" {
            
            match line_vec[1] {
                "cd" => cd(&mut dir_stack, line_vec[2]),
                "ls" => continue,
                _ => (),
            }
        } else {

            if line_vec[0] == "dir" {
                println!("--- Nothing (dir listed)");
                continue
            } else {
                let file_size = line_vec[0].parse::<i64>().unwrap();
                
                println!("--- Adding size {} to {:?}", file_size, dir_stack);
                for dir in dir_stack.iter() {
                    *dir_size.entry(dir).or_insert(0) += file_size;
                    //let mut entry = dir_size.entry(dir).or_insert(0);
                    //println!("{}", entry);
                    //*entry += file_size;
                    //println!("{}", entry);
                }

            }
        }
    }
    println!("{:#?}", dir_size);
    let sum : i64 = dir_size.iter().filter(|(k, v)| **v <= 100000).map(|(k, v)| v).sum();
    println!("{}", sum);
}

// can implement this as a stack with vector and take a hashmap that
// stores the scores (will need to iterate over vec)
// look for cd commands to get dir 
// if .. then pop from stack otherwise add to stack
// after ls, start adding numbers 
// if this gets tricky i'll need to do this
// Define somehow a filesystem struct
// need an enum fs type which handles files and pattern matching

fn get_cmd(cmd: &str) {

}

// need to use lifetime annotations here, but not sure if this can be avoided
fn cd<'a>(dir_stack: &mut Vec<&'a str>, dir: &'a str) {
    if dir == ".." {
        let popped = dir_stack.pop().unwrap();
        println!("popped {}", popped);
    } else {
        println!("pushing {}", dir);
        dir_stack.push(dir);
    }
}