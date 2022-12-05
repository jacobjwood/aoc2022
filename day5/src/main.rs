use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let contents_vec = contents.split("\n\n").collect::<Vec<&str>>();
    let crates = contents_vec[0];
    let instructions = contents_vec[1];

    println!("{}", crates);
    println!("{}", instructions);
    let mut crate_map = parse_crates(&crates);
    println!("{:#?}", crate_map);
    let instructions_vec = parse_instructions(&instructions);
    println!("{:?}", instructions_vec);
    let mut crate_map_pt2 = crate_map.clone();

    move_crates(true, &mut crate_map, &instructions_vec);
    move_crates(false, &mut crate_map_pt2, &instructions_vec);

    let mut pt1 = crate_map.iter().collect::<Vec<_>>();
    let mut pt2 = crate_map_pt2.iter().collect::<Vec<_>>();
    pt1.sort_by(|x, y| x.0.cmp(&y.0));
    pt2.sort_by(|x, y| x.0.cmp(&y.0));
    let mut pt1 = pt1.iter().map(|(a, b)| b.last().copied().unwrap()).collect::<String>().replace(|c: char| !c.is_alphanumeric(), "");
    let mut pt2 = pt2.iter().map(|(a, b)| b.last().copied().unwrap()).collect::<String>().replace(|c: char| !c.is_alphanumeric(), "");
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}

fn parse_crates(crate_string: &str) -> HashMap<usize, Vec<&str>> {

    for line in crate_string.lines().rev() {
	println!("{}", line);
    }
    let crate_string = crate_string.lines().rev().collect::<Vec<&str>>();
    println!("{:#?}", crate_string);
    
    let mut crate_map = HashMap::<usize, Vec<&str>>::new();

    for line in crate_string[1..].iter() {
	println!("New line: {}", line.len());
	for (idx, nbr) in (0..line.len()).step_by(4).enumerate() {
	    println!("{} {}", idx+1, nbr);
	    let slice = &line[nbr..nbr+3];
	    println!("{}", slice);
	    if !slice.trim().is_empty() { 
	    	let crate_vec = crate_map.entry(idx+1).or_insert(Vec::<&str>::new());
	    	crate_vec.push(slice);
	    } 
	}
    }
    crate_map.to_owned()
}

fn parse_instructions(instructions_string: &str) -> Vec<Vec<usize>> {
    let mut instructions_vec = Vec::<Vec<usize>>::new();
    for line in instructions_string.lines() {
        let line_vec = line.split(" ").enumerate().filter(|(idx, string)| idx % 2 != 0).map(|(idx, string)| string.parse::<usize>().unwrap()).collect::<Vec<usize>>();
	instructions_vec.push(line_vec);
    }
    instructions_vec
}

fn move_crates(part1: bool, crate_map: &mut HashMap<usize, Vec<&str>>, instructions_vec: &Vec<Vec<usize>>) {
    
    for instruction in instructions_vec.iter() {
	println!("Take {:?} from {} into {}", instruction[0], instruction[1], instruction[2]);
	let take_vec = crate_map.get_mut(&instruction[1]).unwrap();
        
        println!("{} : {:?}", instruction[1], take_vec);	
	let new_len = take_vec.len().saturating_sub(instruction[0]);
        let mut take = take_vec.split_off(new_len);
	
	if part1 { take.reverse(); }
	
	println!("Taken {:?}", take);
	println!("After {} : {:?}", instruction[1], take_vec);
	
	let give_vec = crate_map.get_mut(&instruction[2]).unwrap();
	println!("{} : {:?}", instruction[2], give_vec);
        give_vec.append(&mut take);
	println!("After {} : {:?}", instruction[2], give_vec);        
    }
}
