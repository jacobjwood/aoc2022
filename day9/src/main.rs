use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Part 1: {}", solution(2));
    println!("Part 2: {}", solution(10));
}

fn solution(n_knots: i32) -> usize {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut knot_vec: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n_knots {
        knot_vec.push((0, 0));
    }

    let mut t_states: HashSet<(i32, i32)> = HashSet::new();

    t_states.insert(*knot_vec.last().unwrap());

    for line in contents.lines() {
        // println!("{}", line);
        let cmd_vec = line.split(" ").collect::<Vec<&str>>();

        for _ in 1..=cmd_vec[1].parse::<i32>().unwrap() {
            // knot_vec[0] points to the tuple on the stack
            // the Copy trait is available to a tuple, which is what happens here
            // https://doc.rust-lang.org/std/ops/trait.Index.html
            // "This allows nice things such as let value = v[index] if the type of value implements Copy."
            // so we need to replace the value in the vector by reassignment
            let (mut h_x, mut h_y) = knot_vec[0];

            match &cmd_vec[0] {
                &"U" => h_y += 1,
                &"D" => h_y -= 1,
                &"L" => h_x -= 1,
                &"R" => h_x += 1,
                _ => (),
            }

            knot_vec[0] = (h_x, h_y);

            // iterate over next knots
            for knot_idx in 0..knot_vec.len() - 1 {
                let (knot_1_x, knot_1_y) = knot_vec[knot_idx];
                let (mut knot_2_x, mut knot_2_y) = knot_vec[knot_idx + 1];

                let (distance_x, distance_y) = (knot_1_x - knot_2_x, knot_1_y - knot_2_y);

                match (distance_x.abs(), distance_y.abs()) {
                    (0, 2) => knot_2_y += distance_y / 2,
                    (2, 0) => knot_2_x += distance_x / 2,
                    (2, 1) => {
                        knot_2_x += distance_x / 2;
                        knot_2_y += distance_y;
                    }
                    (1, 2) => {
                        knot_2_x += distance_x;
                        knot_2_y += distance_y / 2;
                    }
                    (2, 2) => {
                        knot_2_x += distance_x / 2;
                        knot_2_y += distance_y / 2;
                    }
                    _ => (),
                }

                knot_vec[knot_idx + 1] = (knot_2_x, knot_2_y);
            }
            //println!("      {:?}", knot_vec);

            t_states.insert(*knot_vec.last().unwrap());
        }
    }
    t_states.len()
}
