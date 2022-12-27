use std::collections::{HashMap, VecDeque, HashSet};
use std::fs;

fn main() {
    let file_path = "input_test.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let bp_recipes = parse_input(&contents);

    println!("{:?}", bp_recipes);
    /*
    let mut running_qual = 0;
    for x in 1..=30 {
        let geode_count = find_best_geodes(bp_recipes.get(&x).unwrap(), &24);
        println!("{}", x * geode_count);
        running_qual += x * geode_count;
    }

    println!("Part 1: {}", running_qual);

    */

    let mut running_geo_prod = 1;
    for x in 1..=2 {
        let geode_count = find_best_geodes(bp_recipes.get(&x).unwrap(), &24);
        running_geo_prod *= geode_count;
        println!("Running prod: {}", running_geo_prod);
    }

    println!("Part 2: {}", running_geo_prod);
}

fn parse_input(contents: &String) -> HashMap<usize, HashMap<&str, (usize, usize, usize)>> {
    let mut bp_recipes: HashMap<usize, HashMap<&str, (usize, usize, usize)>> = HashMap::new();

    for line in contents.lines() {
        let mut recipes = HashMap::new();

        let blueprint_no = line.split(":").collect::<Vec<_>>()[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        // (ore, clay, obsidian)
        for (idx, recipe) in line.split(".").enumerate() {
            if !recipe.is_empty() {
                let mat = recipe
                    .split(" ")
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>();

                match idx {
                    0 => recipes.insert("ore", (mat[0], 0, 0)),
                    1 => recipes.insert("clay", (mat[0], 0, 0)),
                    2 => recipes.insert("obsidian", (mat[0], mat[1], 0)),
                    3 => recipes.insert("geode", (mat[0], 0, mat[1])),
                    _ => recipes.insert("_", (0, 0, 0)),
                };
            }
        }

        bp_recipes.insert(blueprint_no, recipes);
    }

    bp_recipes
}

fn bound(
    current_best: &usize,
    time: &usize,
    tl: &usize,
    item_count: &(usize, usize, usize, usize),
    robot_count: &(usize, usize, usize, usize),
    recipes: &HashMap<&str, (usize, usize, usize)>,
    min_time_to_geode_robot: &usize, 
) -> bool {

    let time_remaining = tl + 1 - time;
    let ore_req_ore = recipes.get(&"ore").unwrap().0;
    let ore_req_clay = recipes.get(&"clay").unwrap().0;
    let ore_req_obs = recipes.get(&"obsidian").unwrap().0;
    let ore_req_geo = recipes.get(&"geode").unwrap().0;
    let obs_req = recipes.get(&"geode").unwrap().2;
    let clay_req = recipes.get(&"obsidian").unwrap().1;
    let ore_req_max = recipes.iter().map(|(_, v)| v.0).max().unwrap();
    let (ore_bots, clay_bots, obs_bots, geo_bots) = robot_count;
    let (ore_count, clay_count, obs_count, geo_count) = item_count;

    // this bound estimate is the thing I need to get right
    // max 24 robots can be built
    // time to geo robot if every effort concerns getting to a geo robot

    // already saturated enough
    if clay_bots > &clay_req {
        return false;
    }

    if obs_bots > &obs_req {
        return false;
    }

    if ore_bots > &ore_req_max {
        return false;
    }



    // assume independence
    //let ttg = std::cmp::min((ore_req_geo - std::cmp::min(ore_count, ore_req_geo)) / ore_bots, obs_req - std::cmp::min((obs_req, obs_count)) / obs_bots);
    let potential_ore : usize = ore_count + ore_bots * time_remaining + (time_remaining * (time_remaining - 1) / (1 + ore_req_ore / ore_bots));
    let potential_clay : usize = clay_count + clay_bots * time_remaining + (time_remaining * (time_remaining - 1) / (1 + ore_req_clay / ore_bots));
    let potential_obs : usize = obs_count + obs_bots * time_remaining + (time_remaining * (time_remaining - 1) / 1 + std::cmp::min(clay_req / (clay_bots + 1), ore_req_obs / (ore_bots + 1)));
    let potential_geo : usize = geo_count + geo_bots * time_remaining + (time_remaining * (time_remaining - 1) / 1 + std::cmp::min(obs_req / (obs_bots + 1), ore_req_geo / (ore_bots + 1)));

    //println!("PG {}, BG {}, PO {}, TR {}", potential_geo, current_best, potential_ore, time_remaining);

    if potential_geo < *current_best {
        return false;
    }

    true

}

fn find_best_geodes(recipes: &HashMap<&str, (usize, usize, usize)>, tl: &usize) -> usize {
    //println!("{:?}", recipes);
    // ore, clay, obs, geode
    let mut item_count = (0, 0, 0, 0);
    let mut robot_count = (1, 0, 0, 0);

    let check_order = vec!["ore", "clay", "obsidian", "geode"];
    let mut time = 0;
    let mut best_geode = 0;
    let mut best_geode_prev = 0;
    let mut best_rob = 0;
    let mut min_time_to_geode_robot = 24;
    let mut repeat_count = 0;

    // time, robot count, geode count
    let mut visited_states : HashSet<(usize, (usize, usize, usize, usize), (usize, usize, usize, usize))> = HashSet::new();
    let mut min_time_to_state : HashMap<((usize, usize, usize, usize), (usize, usize, usize, usize), i32), usize> = HashMap::new();

    let mut traj_vec: Vec<(
        usize,
        (usize, usize, usize, usize),
        (usize, usize, usize, usize),
        i32,
    )> = Vec::new();
    traj_vec.push((time, item_count, robot_count, -1));

    'outer: while let Some((time, item_count, robot_count, choose_not)) = traj_vec.pop() {
        //println!("TIME {} BG {} RC {:?} IC {:?}", time, best_geode, robot_c   oun    t, item_count);
        let tr = tl - time;
        //let (cn_ore, cn_clay, cn_obs, cn_geo) = choose_not;
        //println!("mm {}", traj_vec.len());
        best_rob = std::cmp::max(robot_count.3, best_rob); 
        //println!("RC {:?} IC {:?} BG {:?}", robot_count, item_count, best_geode);

        if visited_states.contains(&(time, robot_count, item_count)) {
            //println!("ALREADY VISITED");
            continue;
        }

        let entry = min_time_to_state.get(&(item_count, robot_count, choose_not));

        best_geode = std::cmp::max(item_count.3 +(robot_count.3 * tr), best_geode);

        match entry {
            Some(t) => {
                if *t <= time {
                    continue;
                } else {
                    let entree = min_time_to_state.entry((item_count, robot_count, choose_not)).or_insert(time);
                    *entree = std::cmp::min(time, *entree);
                }
            }
            None => {min_time_to_state.insert((item_count, robot_count, choose_not), time);},
        }
        //println!("{:?}, {:?}, {} {}", item_count, robot_count, entry, time);
        //*entry = std::cmp::min(*entry, time);
        

        visited_states.insert((time, robot_count, item_count));

        if time == *tl {
            //println!("JEFF {}", item_count.3);
            if best_geode == best_geode_prev {
                repeat_count += 1;
            }

            /*
            if repeat_count == 50000 {
                break 'outer;
            }
            */

            best_geode_prev = best_geode;
            best_geode = std::cmp::max(item_count.3, best_geode);

            //println!("BG {}", best_geode);
            continue;
        }

        let resources_to_add = robot_count.to_owned();
        let mut best_could_build : i32 = -1;

        //println!("{:?} {}", check_order, bcb);

        'inner: for (idx, c) in check_order.iter().rev().enumerate() {
            // println!("{:?}", recipes.get(c).unwrap());
            let (ore_req, clay_req, obs_req) = recipes.get(c).unwrap();

            // can synthesise
            if item_count.0 >= *ore_req && item_count.1 >= *clay_req && item_count.2 >= *obs_req {


                best_could_build = std::cmp::max(best_could_build, (3 - idx) as i32) as i32;
                //println!("BCB {}", best_could_build); 
                let mut new_item_count = item_count.to_owned();
                let mut new_robot_count = robot_count.to_owned();

                //println!("NIC {:?}", new_item_count);
                new_item_count.0 -= ore_req;
                new_item_count.1 -= clay_req;
                new_item_count.2 -= obs_req;
                //println!("NIC2 {:?}", new_item_count);
                //println!("C {:?} {}", idx, c);

                new_item_count.0 += resources_to_add.0;
                new_item_count.1 += resources_to_add.1;
                new_item_count.2 += resources_to_add.2;
                new_item_count.3 += resources_to_add.3;

                //println!("ROBOT BEFORE {:?}", new_robot_count);

                match idx {
                    0 => new_robot_count.3 += 1,
                    1 => new_robot_count.2 += 1,
                    2 => new_robot_count.1 += 1,
                    3 => new_robot_count.0 += 1,
                    _ => (),
                }

                //println!("ROBOT AFTER {:?}", new_robot_count);
                if robot_count.3 >= 1 {
                    min_time_to_geode_robot = std::cmp::min(min_time_to_geode_robot, time);
                    //println!("Min time {}", min_time_to_geode_robot);
                }

                if true && bound(&best_geode, &time, &tl, &new_item_count, &new_robot_count, &recipes, &min_time_to_geode_robot) {

                    //let mut entry = min_time_to_state.entry((new_item_count, new_robot_count, -1)).or_insert(time+1);
                    //*entry = std::cmp::min(*entry, time+1);
                    traj_vec.push((time + 1, new_item_count, new_robot_count, -1));
                    //traj_vec.sort_by(|a, b| std::cmp::max(b.1.0, b.1.1).cmp(&a.1.1));
                    //break;
                }
            
            }
        }

        //println!("HI");
        let mut new_item_count = item_count.to_owned();

        new_item_count.0 += resources_to_add.0;
        new_item_count.1 += resources_to_add.1;
        new_item_count.2 += resources_to_add.2;
        new_item_count.3 += resources_to_add.3;

        if true && bound(&best_geode, &time, &tl, &new_item_count, &robot_count, &recipes, &min_time_to_geode_robot) {
            
            //let mut entry = min_time_to_state.entry((new_item_count, robot_count, best_could_build)).or_insert(time+1);
            traj_vec.push((time + 1, new_item_count, robot_count, best_could_build as i32));
            //traj_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
            //break;
        }
        
    }
    println!("Best_ROB {}", best_rob);
    best_geode
}
