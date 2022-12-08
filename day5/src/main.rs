use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_vec = contents.split("\n\n").collect::<Vec<&str>>();
    let crates = contents_vec[0];
    let instructions = contents_vec[1];

    let mut crate_map = parse_crates(&crates);
    let instructions_vec = parse_instructions(&instructions);
    let mut crate_map_pt2 = crate_map.clone();

    move_crates(true, &mut crate_map, &instructions_vec);
    move_crates(false, &mut crate_map_pt2, &instructions_vec);

    let mut pt1 = crate_map.iter().collect::<Vec<_>>();
    let mut pt2 = crate_map_pt2.iter().collect::<Vec<_>>();
    pt1.sort_by(|x, y| x.0.cmp(&y.0));
    pt2.sort_by(|x, y| x.0.cmp(&y.0));
    let pt1 = pt1
        .iter()
        .map(|(_, b)| b.last().copied().unwrap())
        .collect::<String>()
        .replace(|c: char| !c.is_alphanumeric(), "");
    let pt2 = pt2
        .iter()
        .map(|(_, b)| b.last().copied().unwrap())
        .collect::<String>()
        .replace(|c: char| !c.is_alphanumeric(), "");
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}

fn parse_crates(crate_string: &str) -> HashMap<usize, Vec<&str>> {
    let crate_string = crate_string.lines().rev().collect::<Vec<&str>>();

    let mut crate_map = HashMap::<usize, Vec<&str>>::new();

    for line in crate_string[1..].iter() {
        for (idx, nbr) in (0..line.len()).step_by(4).enumerate() {
            let slice = &line[nbr..nbr + 3];
            if !slice.trim().is_empty() {
                let crate_vec = crate_map.entry(idx + 1).or_insert(Vec::<&str>::new());
                crate_vec.push(slice);
            }
        }
    }
    crate_map.to_owned()
}

fn parse_instructions(instructions_string: &str) -> Vec<Vec<usize>> {
    let mut instructions_vec = Vec::<Vec<usize>>::new();
    for line in instructions_string.lines() {
        let line_vec = line
            .split(" ")
            .enumerate()
            .filter(|(idx, _)| idx % 2 != 0)
            .map(|(_, string)| string.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        instructions_vec.push(line_vec);
    }
    instructions_vec
}

fn move_crates(
    part1: bool,
    crate_map: &mut HashMap<usize, Vec<&str>>,
    instructions_vec: &Vec<Vec<usize>>,
) {
    for instruction in instructions_vec.iter() {
        let take_vec = crate_map.get_mut(&instruction[1]).unwrap();

        let new_len = take_vec.len().saturating_sub(instruction[0]);
        let mut take = take_vec.split_off(new_len);

        if part1 {
            take.reverse();
        }

        let give_vec = crate_map.get_mut(&instruction[2]).unwrap();
        give_vec.append(&mut take);
    }
}
