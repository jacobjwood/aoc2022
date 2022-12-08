use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut dir_size : HashMap<String, i64> = HashMap::new();
    let mut dir_stack = Vec::<&str>::new();
    let mut running_total_check : i64 = 0;
    let mut vec_str : String = String::new();

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
                running_total_check += file_size;

                vec_str = dir_stack.join("$");

                println!("{}", vec_str);
                for idx in 0..dir_stack.len() {
                    let v = dir_stack[0..=idx].join("$");
                    println!("JEFF {:?}", v);
                    *dir_size.entry(v).or_insert(0) += file_size;
                    
                    //let mut entry = dir_size.entry(dir).or_insert(0);
                    //println!("{}", entry);
                    //*entry += file_size;
                    //println!("{}", entry);
                }
                //let size_tracker : Vec<i64> = dir_stack.iter().map(|x| *dir_size.get(x).unwrap()).collect();
                //println!("--- Adding size {} to {:?} now has size {:?}", file_size, dir_stack, size_tracker);

            }
        }
    }
    println!("{:#?}", dir_size);
    let sum : i64 = dir_size.iter().filter(|(k, v)| **v <= 100000).map(|(k, v)| v).sum();
    //let filtered : HashMap<String, i64> = dir_size.into_iter().filter(|(k, v)| *v <= 100000).collect();
    //println!("{:#?}", filtered);
    //assert_eq!(running_total_check, 40528671);
    println!("{}", sum);

    // 70mil
    // 30mil needed
    let unused_space : i64 = 70000000 - dir_size.get(&String::from("/")).unwrap();
    let needed_space : i64 = 30000000 - unused_space;
    println!("{}", needed_space);
    let filtered : i64 = dir_size.into_iter().filter(|(k, v)| v >= &needed_space).map(|(k, v)| v).min().unwrap();
    println!("{:?}", filtered);
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