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

fn traverse_storm(mut blizz_vec: Vec<(&str, (i32, i32))>, state_set: HashSet<(i32, i32)>, wall_set: HashSet<(i32, i32)>, max_i: i32, max_j: i32) {

    let start = (0, 1);
    let mut pos = start;
    let end = (max_i, max_j-1);
    let prev = (-1, -1);

    let mut traj = VecDeque::from([(0, prev, pos, blizz_vec.to_owned())]);

    loop {
        let (mut count, prev, mut pos, mut bv) = traj.pop_front().unwrap();

        // option to wait as well as move into the differing locations
        // but if blizzard catches me out, I will stop tracking

        println!("POS {:?} {}", pos, count);

        if pos == end {
            println!("Part 1: {}", count);
            break;
        }

        // up right down left and remain
        let up = (pos.0 as i32 - 1, pos.1 as i32);
        let down = (pos.0 as i32 + 1, pos.1 as i32);
        let right = (pos.0 as i32, pos.1 as i32 + 1);
        let left = (pos.0 as i32, pos.1 as i32 - 1);

        evolve_blizzard(&mut bv, &max_i, &max_j);
        let blizz_set = bv.iter().map(|x| x.1).collect::<HashSet<_>>();

        let mut future_vec = vec![pos, up, down, left, right];

        future_vec = future_vec.into_iter().filter(|v| (*v == start || *v == end || ((v.0 >= 1 && v.0 <= max_i - 1) && (v.1 >= 1 && v.1 <= max_j - 1))) && !blizz_set.contains(v)).collect();

        for v in future_vec {
            traj.push_back((count+1, pos, v, bv.to_owned()));
        }

        let mut traj_vec = Vec::from(traj);
        traj_vec.sort_by(|b, a| ((start.0 - a.2.0).pow(2) + (a.2.1 - start.1).pow(2) ).cmp(&((start.0 - b.2.0).pow(2) + (b.2.1 - start.1).pow(2))));
        //traj_vec.sort_by(|b, a| (b.2.0 + b.2.1).cmp(&(a.2.0 + a.2.1)));
        traj = VecDeque::from(traj_vec);

        // loop over possibilities and add to the selection
        //print_blizz(&blizz_vec, &state_set, &wall_set, &max_i, &max_j);
        
    }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    distance: usize,
    index: (i32, i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

// Needed this otherwise the ordering doesn't work
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    graph: &HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    start: (i32, i32, i32),
    goal: (i32, i32, i32),
) -> usize {
    let mut dist: HashMap<(i32, i32, i32), usize> = graph
        .to_owned()
        .into_iter()
        .map(|(node, _)| (node, usize::MAX))
        .collect();

    *dist.entry(start).or_insert(0) = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        distance: 1,
        index: start,
    });

    while let Some(State { distance, index }) = heap.pop() {
        if index.0 == goal.0 && index.1 == goal.1 {
            return distance;
        }

        if distance > *dist.get(&index).unwrap() {
            continue;
        }

        for child in graph.get(&index).unwrap() {
            let next = State {
                distance: distance + 1,
                index: *child,
            };

            if next.distance < *dist.get(&child).unwrap() {
                heap.push(next);
                *dist.entry(next.index).or_insert(usize::MAX) = next.distance;
            }
        }
    }

    // need this in case of loop not executing
    usize::MAX

}