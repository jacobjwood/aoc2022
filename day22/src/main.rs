use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (state_set, wall_set, mut instructions) = parse_input(&contents);

    println!("{:?}", instructions);

    navigate(&instructions, &state_set, &wall_set);
    navigate_2(&instructions, &state_set, &wall_set);

    
}

fn parse_input(contents: &str) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>, Vec<String>) {

    let contents_vec = contents.split("\n\n").collect::<Vec<&str>>();
    let map_string = contents_vec[0];
    let instructions = contents_vec[1].chars().filter(|c| *c != '\n').collect::<String>();
    let instr_numbers = instructions.split(['L', 'R', 'U', 'D']).map(|s| s.to_owned()).collect::<Vec<String>>();
    let instr_dirs = instructions.chars().filter(|c| !c.is_numeric()).map(|c| c.to_string()).collect::<Vec<String>>();

    let mut instructions_vec : Vec<String> = Vec::new();

    for idx in 0..instr_numbers.len() {
        instructions_vec.push(instr_numbers[idx].to_owned());
        if idx < instr_numbers.len() -1 {
            instructions_vec.push(instr_dirs[idx].to_owned());
        }
    }
    //println!("{}", instructions);

    let mut state_set : HashSet<(i32, i32)> = HashSet::new();
    let mut wall_set : HashSet<(i32, i32)> = HashSet::new();

    for (r_idx, line) in map_string.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {

            match c {
                '#' => {
                    wall_set.insert((r_idx as i32, c_idx as i32));
                    state_set.insert((r_idx as i32, c_idx as i32));
                },
                '.' => {state_set.insert((r_idx as i32, c_idx as i32));},
                _ => (),
            }
        }
    }

    (state_set, wall_set, instructions_vec)
}

fn navigate(instructions: &Vec<String>, state_set: &HashSet<(i32, i32)>, wall_set: &HashSet<(i32, i32)>) {

    let mut instructions = instructions.into_iter().rev().map(|s| s.to_owned()).collect::<Vec<String>>();
    let mut position = state_set.iter().min().unwrap().to_owned();
    
    let mut direction = (0, 1); // initially right

    while let Some(instruction) = instructions.pop()  {

        //println!("INSTRUCTION: {}", instruction);

        if !instruction.chars().fold(true, |acc, c| acc && c.is_numeric()) {
            match instruction.as_str() {
                "L" => direction = (-direction.1, direction.0),
                "R" => direction = (direction.1, -direction.0),
                _ => ()
            }
            continue;
        }

        let moves = instruction.parse::<i32>().unwrap();

        'inner: for _ in 0..moves {
            let mut next = (position.0 + direction.0, position.1 + direction.1);

            if !state_set.contains(&next) {

                if direction.0 == -1 {
                    // up direction so filter for same column
                    next = *state_set.iter().filter(|x| x.1 == position.1).max().unwrap();
                } else if direction.0 == 1 {
                    next = *state_set.iter().filter(|x| x.1 == position.1).min().unwrap();
                } else if direction.1 == -1 {
                    // left
                    next = *state_set.iter().filter(|x| x.0 == position.0).max().unwrap();
                } else {
                    // right
                    next = *state_set.iter().filter(|x| x.0 == position.0).min().unwrap();
                }
                
            }

            if wall_set.contains(&next) {
                break 'inner;
            } else {
                position = next;
            }

        }

    }

    let facing = match direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => 100,
    };

    let final_sum = (1000 * (position.0 + 1)) + (4 * (position.1 + 1)) + facing;
    println!("Part 1: {}", final_sum);

    
}

fn navigate_2(instructions: &Vec<String>, state_set: &HashSet<(i32, i32)>, wall_set: &HashSet<(i32, i32)>) {
    // the downward right diagonal encodes face information
    let cube_1_diagonal = ((0, 50), (49, 99));
    let cube_2_diagonal = ((0, 100), (49, 199));
    let cube_3_diagonal = ((50, 50), (99, 99));
    let cube_4_diagonal = ((100, 50), (149, 99));
    let cube_5_diagonal = ((100, 0), (149, 49));
    let cube_6_diagonal = ((150, 0), (199, 49));

    // right down left up // edge of entry
    let cube_1_neighbors = HashMap::from([("right", (2, "left")), ("down", (3, "top")), ("left", (5, "left")), ("up", (6, "left"))]);
    let cube_2_neighbors = HashMap::from([("right", (4, "right")), ("down", (3, "right")), ("left", (1, "right")), ("up", (6, "bottom"))]);
    let cube_3_neighbors = HashMap::from([("right", (2, "bottom")), ("down", (4, "top")), ("left", (5, "top")), ("up", (1, "bottom"))]);
    let cube_4_neighbors = HashMap::from([("right", (2, "right")), ("down", (6, "right")), ("left", (5, "right")), ("up", (3, "bottom"))]);
    let cube_5_neighbors = HashMap::from([("right", (4, "left")), ("down", (6, "top")), ("left", (1, "left")), ("up", (3, "left"))]);
    let cube_6_neighbors = HashMap::from([("right", (4, "bottom")), ("down", (2, "top")), ("left", (1, "top")), ("up", (5, "bottom"))]);

    let cubes = [cube_1_neighbors, cube_2_neighbors, cube_3_neighbors, cube_4_neighbors, cube_5_neighbors, cube_6_neighbors];

    let mut instructions = instructions.into_iter().rev().map(|s| s.to_owned()).collect::<Vec<String>>();
    let mut position = state_set.iter().min().unwrap().to_owned();
    
    let mut direction = (0, 1); // initially right
    let mut cubic_shifts = 0;

    'outer: while let Some(instruction) = instructions.pop()  {

        println!("INSTRUCTION: {}", instruction);

        if !instruction.chars().fold(true, |acc, c| acc && c.is_numeric()) {
            match instruction.as_str() {
                "L" => direction = (-direction.1, direction.0),
                "R" => direction = (direction.1, -direction.0),
                _ => ()
            }
            continue;
        }

        let moves = instruction.parse::<i32>().unwrap();

        'inner: for i in 0..moves {
            let mut next = (position.0 + direction.0, position.1 + direction.1);

            println!("POS {:?}", position);

            // logic to change here
            if !state_set.contains(&next) {

                println!("CUBIC SHIFT");
                cubic_shifts += 1;

                if cubic_shifts > 6 {
                    panic!();
                }

                let diagonals = [cube_1_diagonal, cube_2_diagonal, cube_3_diagonal, cube_4_diagonal, cube_5_diagonal, cube_6_diagonal];
                let mut current_cube = 0;
                // find cube
                for (idx, c) in diagonals.iter().enumerate() {
                    let (top, left) = c.0;
                    let (bottom, right) = c.1;

                    if (left..=right).contains(&position.1) && (top..=bottom).contains(&position.0) {
                        current_cube = idx + 1;
                    }
                }

                //println!("Cube {}", current_cube);

                let current_neighbors = &cubes[current_cube-1];

                let (current_dir, neighbor_cube) = match direction {
                    (0, 1) => ("right", current_neighbors.get(&"right").unwrap()),
                    (0, -1) => ("left", current_neighbors.get(&"left").unwrap()),
                    (1, 0) => ("down", current_neighbors.get(&"down").unwrap()),
                    (-1, 0) => ("up", current_neighbors.get(&"up").unwrap()),
                    _ => ("up", current_neighbors.get(&"up").unwrap()), // arm shouldn't ever be triggered so this is hacky
                };

                let (next_cube, entry_edge) = neighbor_cube.to_owned();
                let cc_diagonal = diagonals[current_cube-1];
                let nc_diagonal = diagonals[next_cube-1];
                let next_edge_idx = match entry_edge {
                    "top" => nc_diagonal.0.0,
                    "bottom" => nc_diagonal.1.0,
                    "left" => nc_diagonal.0.1,
                    "right" => nc_diagonal.1.1,
                    _ => 100, // again shouldn't ever be met
                };

                let (relative_row, relative_column) = (position.0 - cc_diagonal.0.0, position.1 - cc_diagonal.0.1);


                //println!("next_edge_idx {}", next_edge_idx);
                next = match (current_dir, entry_edge) {
                    ("right", "right") => (nc_diagonal.1.0 - relative_row, next_edge_idx),
                    ("right", "bottom") => (next_edge_idx, nc_diagonal.0.1 + relative_row),
                    ("right", "left") => (nc_diagonal.0.0 + relative_row, next_edge_idx),
                    ("right", "top") => (next_edge_idx, nc_diagonal.1.1 - relative_row),
                    ("down", "right") => (nc_diagonal.0.0 + relative_column, next_edge_idx), 
                    ("down", "bottom") => (next_edge_idx, nc_diagonal.1.1 - relative_column),
                    ("down", "left") => (nc_diagonal.1.0 - relative_column, next_edge_idx),
                    ("down", "top") => (next_edge_idx, nc_diagonal.0.1 + relative_column),
                    ("left", "right") => (nc_diagonal.0.0 + relative_row, next_edge_idx),
                    ("left", "bottom") => (next_edge_idx, nc_diagonal.1.1 - relative_row),
                    ("left", "left") => (nc_diagonal.1.0 - relative_row, next_edge_idx),
                    ("left", "top") => (next_edge_idx, nc_diagonal.0.1 + relative_row),
                    ("up", "right") => (nc_diagonal.1.0 - relative_column, next_edge_idx),
                    ("up", "bottom") => (next_edge_idx, nc_diagonal.0.1 + relative_column),
                    ("up", "left") => (nc_diagonal.0.0 + relative_column, next_edge_idx),
                    ("up", "top") => (next_edge_idx, nc_diagonal.1.1 - relative_column),

                    _ => (1, 1), // should never be triggered
                };


                direction = match entry_edge {
                    "left" => (0, 1),
                    "right" => (0, -1),
                    "top" => (1, 0),
                    "bottom" => (-1, 0),
                    _ => (1, 1), // shouldn't ever trigger
                };

                println!("CUBIC SHIFT: POS {:?}, NEXT {:?}", position, next);

            

                
            }

            if wall_set.contains(&next) {
                break 'inner;
            } else {
                position = next;
            }

        }

    }

    let facing = match direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => 100,
    };

    let final_sum = (1000 * (position.0 + 1)) + (4 * (position.1 + 1)) + facing;
    println!("Part 2: {}", final_sum);
}