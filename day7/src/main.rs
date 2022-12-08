use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut dir_size: HashMap<String, i64> = HashMap::new();
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
                continue;
            } else {
                let file_size = line_vec[0].parse::<i64>().unwrap();

                for idx in 0..dir_stack.len() {
                    let v = dir_stack[0..=idx].join("$");
                    // deref here to add to the underlying int, not the ref
                    *dir_size.entry(v).or_insert(0) += file_size;
                }
            }
        }
    }

    let sum: i64 = dir_size
        .iter()
        .filter(|(_, v)| **v <= 100000) // double deref is a bit funky
        .map(|(_, v)| v)
        .sum();
    println!("Part 1: {}", sum);

    // 70mil
    // 30mil needed
    let unused_space: i64 = 70000000 - dir_size.get(&String::from("/")).unwrap();
    let needed_space: i64 = 30000000 - unused_space;

    // So here, into_iter works but iter doesn't
    // This is because into iter consumes the variable as it doesn't have the Copy trait
    // into_iter calls self so consumes itself, while iter() and iter_mut() do not and instead provide references (or mutable refs)
    // iter() and iter_mut() require the Copy trait (or clone)
    // Clone and Copy are different in how they manage memory but again, I have no idea on this right now
    // I do not understand traits properly yet
    let filtered: i64 = dir_size
        .into_iter()
        .filter(|(_, v)| *v >= needed_space)
        .map(|(_, v)| v)
        .min()
        .unwrap();

    // Won't run below because dir_size is consumed
    // println!("{:?}", dir_size);
    println!("Part 2: {:?}", filtered);
}

// need to use lifetime annotations here, but not sure if this can be avoided as I don't yet fully understand them
fn cd<'a>(dir_stack: &mut Vec<&'a str>, dir: &'a str) {
    if dir == ".." {
        dir_stack.pop().unwrap();
    } else {
        dir_stack.push(dir);
    }
}
