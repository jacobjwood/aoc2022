// This doesn't actually do anything
// the stack was overflowing at the OS level but luckily
// I get away with the search space I have
#![recursion_limit = "65536"]

use std::collections::HashSet;
use std::fs;

pub mod search;

use crate::search::search_for_contact;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let cubes = parse_cubes(&contents);

    let mut exposed_sides = 0;
    let mut empty_spaces: HashSet<(i32, i32, i32)> = HashSet::new();

    for cube in &cubes {
        let x_left = (cube.0 - 1, cube.1, cube.2);
        let x_right = (cube.0 + 1, cube.1, cube.2);
        let x_up = (cube.0, cube.1 + 1, cube.2);
        let x_down = (cube.0, cube.1 - 1, cube.2);
        let x_forward = (cube.0, cube.1, cube.2 + 1);
        let x_back = (cube.0, cube.1, cube.2 - 1);

        let neighbor_vec = vec![x_left, x_right, x_up, x_down, x_forward, x_back];

        for n in &neighbor_vec {
            if !cubes.contains(n) {
                exposed_sides += 1;
                empty_spaces.insert(n.to_owned());
            }
        }
    }

    println!("Part 1: {}", exposed_sides);

    // maximum search bounds
    let max_x = cubes.iter().map(|r| r.0).max().unwrap() + 1;
    let max_y = cubes.iter().map(|r| r.1).max().unwrap() + 1;
    let max_z = cubes.iter().map(|r| r.2).max().unwrap() + 1;
    let max_cube_bounds = (max_x, max_y, max_z);

    // minimum search bounds
    let min_x = cubes.iter().map(|r| r.0).min().unwrap() - 1;
    let min_y = cubes.iter().map(|r| r.1).min().unwrap() - 1;
    let min_z = cubes.iter().map(|r| r.2).min().unwrap() - 1;
    let min_cube_bounds = (min_x, min_y, min_z);

    // explored empty spaces
    let mut explored_set = HashSet::new();

    // initialise the search from -1
    let init_cube = (-1, -1, -1);

    // track of the exposed sides with this new search
    let mut exposed_sides = 0;

    // search for contact
    search_for_contact(
        &init_cube,
        &max_cube_bounds,
        &min_cube_bounds,
        &cubes,
        &mut explored_set,
        &mut exposed_sides,
    );

    println!("Part 2: {}", exposed_sides);
}

fn parse_cubes(contents: &String) -> HashSet<(i32, i32, i32)> {
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in contents.lines() {
        let line_vec = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        cubes.insert((line_vec[0], line_vec[1], line_vec[2]));
    }

    cubes
}
