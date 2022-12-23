use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let file_path = "input_test.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let bp_recipes = parse_input(&contents);

    println!("{:?}", bp_recipes);

    for x in 1..=2 {
        let geode_count = find_best_geodes(bp_recipes.get(&x).unwrap());
        println!("{}", x * geode_count);
    }
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
    item_count: &(usize, usize, usize, usize),
    robot_count: &(usize, usize, usize, usize),
    recipes: &HashMap<&str, (usize, usize, usize)>,
) -> bool {

    let time_remaining = 25 - time;
    let obs_req = recipes.get(&"geode").unwrap().2;
    let clay_req = recipes.get(&"obsidian").unwrap().1;
    let estimand = 6;

    // this bound estimate is the thing I need to get right
    // max 24 robots can be built
    // time to geo robot if every effort concerns getting to a geo robot
    if robot_count.3 > 0 {
        true
    } else if robot_count.2 > 0 {
        
    }
    
    //estimate >= *current_best
}

fn find_best_geodes(recipes: &HashMap<&str, (usize, usize, usize)>) -> usize {
    //println!("{:?}", recipes);
    // ore, clay, obs, geode
    let mut item_count = (0, 0, 0, 0);
    let mut robot_count = (1, 0, 0, 0);
    let check_order = vec!["geode", "obsidian", "clay", "ore"];
    let mut time = 0;
    let mut best_geode = 0;

    let mut traj_vec: Vec<(
        usize,
        (usize, usize, usize, usize),
        (usize, usize, usize, usize),
    )> = Vec::new();
    traj_vec.push((time, item_count, robot_count));

    'outer: while let Some((time, item_count, robot_count)) = traj_vec.pop() {
        //println!("TIME {} BG {} RC {:?} IC {:?}", time, best_geode, robot_count, item_count);

        if time == 24 {
            //println!("JEFF {}", item_count.3);
            best_geode = std::cmp::max(item_count.3, best_geode);
            continue;
        }

        let resources_to_add = robot_count.to_owned();

        'inner: for (idx, c) in check_order.iter().enumerate() {
            // println!("{:?}", recipes.get(c).unwrap());
            let (ore_req, clay_req, obs_req) = recipes.get(c).unwrap();

            // can synthesise
            if item_count.0 >= *ore_req && item_count.1 >= *clay_req && item_count.2 >= *obs_req {
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

                if bound(&best_geode, &time, &new_item_count, &new_robot_count, &recipes) {
                    traj_vec.push((time + 1, new_item_count, new_robot_count));
                    traj_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
                }
                
            }
        }

        let mut new_item_count = item_count.to_owned();

        new_item_count.0 += resources_to_add.0;
        new_item_count.1 += resources_to_add.1;
        new_item_count.2 += resources_to_add.2;
        new_item_count.3 += resources_to_add.3;

        if bound(&best_geode, &time, &new_item_count, &robot_count, &recipes) {
            traj_vec.push((time + 1, new_item_count, robot_count));
            traj_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        }
        
    }

    best_geode
}

fn collect_ore() {}
