use std::collections::{HashSet, VecDeque, HashMap};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{fs, iter};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (mut blizz_vec, mut state_set, wall_set, max_i, max_j) = parse_input(&contents);

    println!("{:?} {}", max_i, max_j);
    print_blizz(&blizz_vec, &state_set, &wall_set, &max_i, &max_j);


    println!("{} {}", max_i, max_j);

    let lf = max_i - 1;
    let hf = max_j - 1;
    let mut multiple = 1;

    let lcf = loop {
        if (lf * multiple) % hf == 0 {
            break lf * multiple;
        }
        multiple += 1;
    };

    let mut blizz_cube = HashMap::new();
    let mut bv = blizz_vec.to_owned();

    for round in 0..lcf {
        
        let blizz_set = bv.iter().map(|x| x.1).collect::<HashSet<_>>();
        blizz_cube.insert(round as i32, blizz_set);
        evolve_blizzard(&mut bv, &max_i, &max_j);

    }

    let graph1 = construct_graph(&blizz_cube, &max_i, &max_j, 500, &lcf, &0, (0, 1, 0), (max_i, max_j-1, 0).to_owned());
    let state_1 = graph1.iter().flat_map(|(k, v)| v).filter(|v| v.0 == max_i && v.1 == max_j-1).map(|v| v.2).min().unwrap();
    let graph2 = construct_graph(&blizz_cube, &max_i, &max_j, 500, &lcf, &state_1, (max_i, max_j-1, 0).to_owned(), (0, 1, 0));
    let state_2 = graph2.iter().flat_map(|(k, v)| v).filter(|v| v.0 == 0 && v.1 == 1).map(|v| v.2).min().unwrap();
    let graph3 = construct_graph(&blizz_cube, &max_i, &max_j, 500, &lcf, &(state_2 + state_1), (0, 1, 0), (max_i, max_j-1, 0).to_owned());
    let state_3 = graph3.iter().flat_map(|(k, v)| v).filter(|v| v.0 == max_i && v.1 == max_j-1).map(|v| v.2).min().unwrap(); 
    
    println!("Part 1: {}", state_1);
    println!("Part 2a: {} {}", state_2, state_3);
    println!("Part 2: {}", state_3 + state_2 + state_1);

    //println!("{:?}", graph.iter().flat_map(|(k, v)| v).filter(|v| v.0 == max_i && v.1 == max_j-1).map(|v| v.2).min().unwrap());
    //println!("{}", shortest_path(&graph, s, e));

    //traverse_storm(blizz_vec, state_set, wall_set, max_i, max_j);
}

fn parse_input(contents: &str) -> (Vec<(&str, (i32, i32))>, HashSet<(i32, i32)>, HashSet<(i32, i32)>, i32, i32) {

    let mut blizz_vec = Vec::new();
    let mut max_i = 0;
    let mut max_j = 0;

    let mut wall_set = HashSet::new();
    let mut state_set = HashSet::new();

    for (r_idx, line) in contents.lines().enumerate() {

        max_j = line.len() - 1;

        for (c_idx, c) in line.chars().enumerate() {
            match c {
                '^' => blizz_vec.push(("up", (r_idx as i32, c_idx as i32))),
                'v' => blizz_vec.push(("down", (r_idx as i32, c_idx as i32))),
                '>' => blizz_vec.push(("right", (r_idx as i32, c_idx as i32))),
                '<' => blizz_vec.push(("left", (r_idx as i32, c_idx as i32))),
                '#' => {_ = wall_set.insert((r_idx as i32, c_idx as i32))},
                _ => ()
            }

            state_set.insert((r_idx as i32, c_idx as i32));
        }
    }

    max_i = contents.lines().collect::<Vec<_>>().len() - 1;

    (blizz_vec, state_set, wall_set, max_i as i32, max_j as i32)
}

fn evolve_blizzard(blizz_vec: &mut Vec<(&str, (i32, i32))>, max_i: &i32, max_j: &i32) {

    for x in 0..blizz_vec.len() {

        let blizz = &mut blizz_vec[x];
            
        // First deal with the negatively moving blizzards
        if blizz.0 == "up" && blizz.1.0 == 1 {
            blizz.1.0 = *max_i - 1;
            continue;
        }

        if blizz.0 == "right" && blizz.1.1 == *max_j-1 {
            blizz.1.1 = 1;
            continue;
        }

        if blizz.0 == "left" && blizz.1.1 == 1 {
            blizz.1.1 = *max_j - 1;
            continue;
        }

        if blizz.0 == "down" && blizz.1.0 == *max_i - 1 {
            blizz.1.0 = 1;
            continue;
        }

        match blizz.0 {
            "right" => blizz.1.1 += 1,
            "left" => blizz.1.1 -= 1,
            "up" => blizz.1.0 -= 1,
            _ => blizz.1.0 += 1,
        };



    }
}

fn print_blizz(blizz_vec: &Vec<(&str, (i32, i32))>, state_set: &HashSet<(i32, i32)>, wall_set: &HashSet<(i32, i32)>, max_i: &i32, max_j: &i32) {
    let mut blizz_str : Vec<String> = Vec::new();

    let mut blizz_map = HashMap::new();

    for blizz in blizz_vec.iter() {
        blizz_map.entry(blizz.1).or_insert(Vec::new()).push(
            match blizz.0 {
                "up" => '^',
                "down" => 'v',
                "left" => '<',
                "right" => '>',
                _ => 'X',
            }
        )
    }
    
    for i in 0..=*max_i {
        let mut row_vec : String = String::new();
        for j in 0..=*max_j {
            if wall_set.contains(&(i, j)) {
                row_vec.push('#');
            } else if blizz_map.contains_key(&(i, j)) {
                let blizzes = blizz_map.get(&(i, j)).unwrap();

                if blizzes.len() > 1 {
                    let len_char = match std::char::from_u32(blizzes.len().try_into().unwrap()) {
                        Some(c) => 'X',
                        None => '?'
                    };

                    row_vec.push(len_char);
                } else {
                    row_vec.push(blizzes[0]);
                }
            } else {
                row_vec.push('.');
            }
        }
        blizz_str.push(row_vec);
    }

    println!("{:#?}", blizz_str);

}

fn construct_graph(blizz_cube: &HashMap<i32, HashSet<(i32, i32)>>, max_i: &i32, max_j: &i32, length_of_cube: i32, lcf: &i32, init_state: &i32, s: (i32, i32, i32), e: (i32, i32, i32)) -> HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>> {
   
    let mut graph: HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();

    for time in 0..=length_of_cube {
        //println!("{}", time);
        let mut children = graph.iter().flat_map(|(_, children)| children.to_owned()).filter(|c| c.2 == time as i32).collect::<Vec<(i32, i32, i32)>>();

        //println!("graph {:?}", graph);
        if children.len() == 0 {
            children = vec![s];
        }

        //println!("{:?}", children.len());

        for pos in children {
            let up = (pos.0 as i32 - 1, pos.1 as i32, time as i32+1);
            let down = (pos.0 as i32 + 1, pos.1 as i32, time as i32+1);
            let right = (pos.0 as i32, pos.1 as i32 + 1, time as i32+1);
            let left = (pos.0 as i32, pos.1 as i32 - 1, time as i32+1);
            let stay = (pos.0 as i32, pos.1 as i32, time as i32 +1);
            let mut future_vec = vec![stay, up, down, left, right];

            // && !blizz_cube.get(&((time + 1) % lcf)).unwrap().contains(&(v.0, v.1)
            let blizz_set = blizz_cube.get(&((init_state+time+1) % lcf)).unwrap();
            //println!("{:?}", blizz_set.len());
            if time == 2 {

            }
            future_vec = future_vec.into_iter().filter(|v| (*v == (s.0, s.1, time as i32 + 1) || *v == (e.0, e.1, time as i32 +1) || ((v.0 >= 1 && v.0 <= max_i - 1) && (v.1 >= 1 && v.1 <= max_j - 1) && !blizz_set.contains(&(v.0, v.1))))).collect();

            //println!("{:?}", future_vec.len());
            graph.insert(pos, future_vec.to_owned());
        }
            
    }

    graph
}