use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let blow_stream = contents.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

    let mut rock_count = 0;
    let mut cycle_count = 0;

    // initialise the floor
    let mut settled_set = HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6)]);

    while rock_count < 2022 {

        // initialise the rock with top left coordinate at 0, 0
        let mut rock = rock_types(&rock_count);
        // left starts 2 away from the wall
        rock = rock.iter().map(|r| (r.0, r.1 + 2)).collect();
        // get max depth of rock segment and push everything settled "further down" so that it is 3 away
        let rock_height = rock.iter().map(|r| r.0).max().unwrap();
        let settled_height = settled_set.iter().map(|r| r.0).min().unwrap();
        settled_set = settled_set.iter().map(|r| (r.0 + rock_height + 4 - settled_height, r.1)).collect();

        // check collision
        blow_rock(rock, &mut settled_set, &blow_stream, &mut cycle_count);

        
        //println!("LEN SS {}", settled_set.len());
        rock_count += 1;

       
    }

    // print_grid(&settled_set);
    println!("Part 1: {:?}", settled_set.iter().map(|r| r.0).max().unwrap() - settled_set.iter().map(|r| r.0).min().unwrap());

    let mut rock_count = 0;
    let mut cycle_count = 0;
    let mut height_count = 0;

    let mut settled_set = HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6)]);
    let mut floor_set : Vec<HashSet<Vec<(i32, i32)>>> = Vec::new();

    for _ in 0..=4 {
        floor_set.push(HashSet::new());
    }

    let mut found_match = false;
    let mut first_rc_break = 0;
    let mut first_hc_break = 0;
    let mut first_cc_break = 0;
    let mut first_repeat : Vec<(i32, i32)> = Vec::new();
    let first_track;
    let remainder;

    // some arbitrarily large target such that we find a repeat in the set
    'outer: loop {

        // initialise the rock with top left coordinate at 0, 0
        let mut rock = rock_types(&rock_count);
        // left starts 2 away from the wall
        rock = rock.iter().map(|r| (r.0, r.1 + 2)).collect();
        // get max depth of rock segment and push everything settled "further down" so that it is 3 away
        let rock_height = rock.iter().map(|r| r.0).max().unwrap();
        let settled_height = settled_set.iter().map(|r| r.0).min().unwrap();
        settled_set = settled_set.iter().map(|r| (r.0 + rock_height + 4 - settled_height, r.1)).collect();

        // check collision
        blow_rock(rock, &mut settled_set, &blow_stream, &mut cycle_count);

        
        rock_count += 1;

        let depth = settled_set.iter().map(|r| r.0).max().unwrap();
        let min_depth = settled_set.iter().map(|r| r.0).min().unwrap();

        // initialise new set to check if all horizontal states are covered
        let mut breadth_set : HashSet<i32> = HashSet::new();

        for k in min_depth..=depth {

            // see if all horizontal states have been covered
            let bs = settled_set.iter().filter(|r| r.0 == k).map(|r| r.1).collect::<HashSet<_>>();

            breadth_set.extend(bs);

            // check all floor states have been covered (not actually sufficient, the next hack fixes it)
            if breadth_set.len() == 7 {
                
                // this is a bit hacky, but basically just add enough k that there is a definite floor (had to tune this)
                // choose a sufficiently large k that there is definitely a floor
                settled_set = settled_set.into_iter().filter(|r| r.0 <= k + 200).collect();
                let new_depth = settled_set.iter().map(|r| r.0).max().unwrap();
                
                // if we're shifting the set, need to keep track of the shift so we can 
                // add this shift to the height count
                if new_depth - min_depth != depth - min_depth {
                    height_count += (depth - min_depth) - (new_depth - min_depth);
                }

                // generate the current settled set into a sorted vector of positions
                let mut ss = settled_set.iter().cloned().collect::<Vec<(i32, i32)>>();
                ss.sort();

                // check if we've seen this configuration before FOR THE SAME ROCK
                if floor_set[rock_count % 5].contains(&ss) {

                    // if already found a match, then we can just multiply this
                    // saves us iterating 1e12 times
                    if found_match {
                        // if we've seen this configuration before for the same rock that just fell
                        if ss == first_repeat && rock_count % 5 == first_rc_break % 5 {
                            settled_set = HashSet::from_iter(ss.to_owned().into_iter());

                            // the differences between the first and second time we saw the same config
                            // we need these to multiply / tally up
                            let rc_diff = rock_count - first_rc_break;
                            let hc_diff = height_count + settled_set.iter().map(|r| r.0).max().unwrap() - settled_set.iter().map(|r| r.0).min().unwrap() - first_hc_break;
                            let cc_diff = cycle_count - first_cc_break;

                            // the remaining repetitions needed
                            let n_repetitions = (1000000000000 - (rock_count) as i64) as i64 / rc_diff as i64;
                            // height before the remainder is accounted for
                            first_track = first_hc_break as i64 + (hc_diff as i64 * (n_repetitions + 1));
                            // find out the remaining number of rocks we need to simulate
                            remainder = ((1000000000000 - (rock_count + rc_diff) as i64) as i64 % rc_diff as i64) as usize;

                            // increase the rock and cycle counts accordingly
                            // need to +1 to n_repetitions to account for the fact we've already seen 1 repetition before this (the first one we match that triggers this condition)
                            rock_count = ((rock_count as i64 + ((n_repetitions + 1) * rc_diff as i64)) % 5) as usize;
                            cycle_count = ((cycle_count as i64 + ((n_repetitions + 1) * cc_diff as i64) as i64) % blow_stream.len() as i64) as usize; 
                            break 'outer;
                        }
                    }

                    // If first time finding match, keep track
                    if !found_match {
                        first_rc_break = rock_count;
                        first_hc_break = height_count + settled_set.iter().map(|r| r.0).max().unwrap() - settled_set.iter().map(|r| r.0).min().unwrap();
                        first_cc_break = cycle_count;
                        first_repeat = ss.to_owned();
                        settled_set = HashSet::from_iter(ss.to_owned().into_iter());
                        found_match = true;
                    }

                } else {
                    // if the floor_set doesn't contain this configuration,
                    // then need to insert it into the set
                    floor_set[rock_count % 5].insert(ss);
                }   
                break;
            }
        }
    };

    // keep track of the initial height so we can get the extra bit
    let ss_1 = settled_set.iter().map(|r| r.0).max().unwrap() - settled_set.iter().map(|r| r.0).min().unwrap();
    // how many rounds between rock_count and the extra bit we need to iterate over
    let rmder = remainder as usize + rock_count as usize;

    // loop over the extra bit that isn't in the cycle
    while rock_count < rmder {

        // initialise the rock with top left coordinate at 0, 0
        let mut rock = rock_types(&rock_count);
        // left starts 2 away from the wall
        rock = rock.iter().map(|r| (r.0, r.1 + 2)).collect();
        // get max depth of rock segment and push everything settled "further down" so that it is 3 away
        let rock_height = rock.iter().map(|r| r.0).max().unwrap();
        let settled_height = settled_set.iter().map(|r| r.0).min().unwrap();
        settled_set = settled_set.iter().map(|r| (r.0 + rock_height + 4 - settled_height, r.1)).collect();

        // check collision
        blow_rock(rock, &mut settled_set, &blow_stream, &mut cycle_count);

        
        //println!("LEN SS {}", settled_set.len());
        rock_count += 1;

       
    }

    println!("Part 2: {:?}", first_track + settled_set.iter().map(|r| r.0).max().unwrap() as i64 - settled_set.iter().map(|r| r.0).min().unwrap() as i64 - ss_1 as i64);


}

fn print_grid(settled_set: &HashSet<(i32, i32)>) {
    let max_y = settled_set.iter().map(|r| r.0).max().unwrap();
    let max_x = settled_set.iter().map(|r| r.1).max().unwrap();

    // println!("{} {}", max_x, max_y);

    let mut vec_grid : Vec<String> = Vec::new();

    for y in 0..=max_y {
        let mut row_vec : Vec<char> = Vec::new();
        for x in 0..=max_x {
            if settled_set.contains(&(y, x)) {
                row_vec.push('#');
            } else {
                row_vec.push('.');
            }
        }

        vec_grid.push(row_vec.iter().collect::<String>());

    }

    println!("{:#?}", vec_grid);

}

fn rock_types(rock_number: &usize) -> Vec<(i32, i32)> {
    // Start index is top left
    let h_line_rock = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let plus_rock = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let l_rock = vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)];
    let v_line_rock = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let square_rock = vec![(0, 0), (0, 1), (1, 0), (1, 1)];

    let generator_vec = vec![h_line_rock, plus_rock, l_rock, v_line_rock, square_rock];

    let yield_rock = &generator_vec[*rock_number % 5];

    yield_rock.to_owned()

}

fn blow_rock(mut rock: Vec<(i32, i32)>, 
             settled_set: &mut HashSet<(i32, i32)>, 
             blow_stream: &Vec<char>, 
             cycle_count: &mut usize) {
    // first let's not bother with the thing
    let mut collision = false;

    'outer: loop {

        let blow_dir : char = blow_stream[*cycle_count % blow_stream.len()];

        let dir = match blow_dir {
            '>' => 1,
            '<' => -1,
            _ => 0,
        };

        // blow left or right (if possible)
        let next_pos : Vec<(i32, i32)> = rock.iter().map(|r| (r.0, r.1 + dir)).collect();

        let left_most_idx = next_pos.iter().map(|r| r.1).min().unwrap();
        let right_most_idx = next_pos.iter().map(|r| r.1).max().unwrap();

        let mut proceed = true;

        if left_most_idx == -1 || right_most_idx == 7 {
            proceed = false;
        } else {
            for pos in next_pos.iter() {
                if settled_set.contains(pos) {
                    proceed = false;
                    break;
                }
            }
        }

        //println!("Blowing {} {}", blow_dir, proceed);

        if proceed { rock = next_pos };

        // check for collision below
        let next_pos : Vec<(i32, i32)> = rock.iter().map(|r| (r.0 + 1, r.1)).collect();

        for pos in next_pos.iter() {
            if settled_set.contains(pos) {
                collision = true;
            }
        }

        // if not collision, advance by 1
        if collision {

            for pos in &rock {
                settled_set.insert(*pos);
            }

            *cycle_count += 1;
            break 'outer;
        }

        rock = next_pos;
        *cycle_count += 1;
    }
    
}
