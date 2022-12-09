use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // will try and start with tuples
    // start by modelling H and T
    let (mut h_x, mut h_y) = (0, 0);
    let (mut t_x, mut t_y) = (0, 0);

    let mut t_states : HashSet<(i32, i32)> = HashSet::new();
    let mut knot_vec : Vec<(i32, i32)> = Vec::new();
    t_states.insert((t_x, t_y));
    
    for line in contents.lines() {
        let cmd_vec = line.split(" ").collect::<Vec<&str>>();

        for step in 1..=cmd_vec[1].parse::<i32>().unwrap() {

            match &cmd_vec[0] {
                &"U" => h_y += 1,
                &"D" => h_y -= 1,
                &"L" => h_x -= 1,
                &"R" => h_x += 1,
                _ => (),
            }

            let (distance_x, distance_y) = (h_x - t_x, h_y - t_y);

            match (distance_x.abs(), distance_y.abs()) {
                (0, 2) => t_y += distance_y / 2,
                (2, 0) => t_x += distance_x / 2,
                (2, 1) => {
                    t_x += distance_x / 2;
                    t_y += distance_y;
                },
                (1, 2) => {
                    t_x += distance_x;
                    t_y += distance_y / 2;
                }
                _ => (),
            
            }

            println!("{} moves H to {:?} and T to {:?} and H and T are {:?} apart", line, (h_x, h_y), (t_x, t_y), (distance_x, distance_y));
    
            t_states.insert((t_x, t_y));


        }

        

    }

    println!("{}", t_states.len());

}

fn print_grid() {

}

fn get_distance() {

}

fn move_tail() {

}
