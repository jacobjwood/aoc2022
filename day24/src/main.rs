use std::fs;
use std::collections::{HashSet, VecDeque, HashMap};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (mut blizz_vec, mut state_set, wall_set, max_i, max_j) = parse_input(&contents);

    println!("{:?} {}", max_i, max_j);
    print_blizz(&blizz_vec, &state_set, &wall_set, &max_i, &max_j);

    traverse_storm(blizz_vec, state_set, wall_set, max_i, max_j);
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

        if count > 20 && (pos.0 * pos.1 < count) {
            continue;
        }
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

        let mut future_vec = vec![up, down, left, right];

        future_vec = future_vec.into_iter().filter(|v| (*v == start || *v == end || ((v.0 >= 1 && v.0 <= max_i - 1) && (v.1 >= 1 && v.1 <= max_j - 1))) && !blizz_set.contains(v)).collect();

        if future_vec.len() == 0 {
            future_vec.push(pos);
        }

        for v in future_vec {
            traj.push_back((count+1, pos, v, bv.to_owned()));
        }

        let mut traj_vec = Vec::from(traj);
        //traj_vec.sort_by(|b, a| ((start.0 - a.1.0).pow(2) + (a.1.1 - start.1).pow(2) ).cmp(&((start.0 - b.1.0).pow(2) + (b.1.1 - start.1).pow(2))));
        traj_vec.sort_by(|b, a| (b.2.0 + b.2.1).cmp(&(a.2.0 + a.2.1)));
        traj = VecDeque::from(traj_vec);

        // loop over possibilities and add to the selection
        //print_blizz(&blizz_vec, &state_set, &wall_set, &max_i, &max_j);
    }
}