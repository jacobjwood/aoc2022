use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let bp_recipes = parse_input(&contents);

    let mut running_qual = 0;
    for x in 1..=30 {
        let geode_count = find_best_geodes(bp_recipes.get(&x).unwrap(), &24);
        running_qual += x * geode_count;
    }

    println!("Part 1: {}", running_qual);

    let mut running_geo_prod = 1;
    for x in 1..=3 {
        let geode_count = find_best_geodes(bp_recipes.get(&x).unwrap(), &32);
        running_geo_prod *= geode_count;
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
    robot_count: &(usize, usize, usize, usize),
    recipes: &HashMap<&str, (usize, usize, usize)>,
) -> bool {
    let obs_req = recipes.get(&"geode").unwrap().2;
    let clay_req = recipes.get(&"obsidian").unwrap().1;
    let ore_req_max = recipes.iter().map(|(_, v)| v.0).max().unwrap();
    let (ore_bots, clay_bots, obs_bots, _) = robot_count;

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

    true
}

fn find_best_geodes(recipes: &HashMap<&str, (usize, usize, usize)>, tl: &usize) -> usize {
    // ore, clay, obs, geode
    let item_count = (0, 0, 0, 0);
    let robot_count = (1, 0, 0, 0);

    let check_order = vec!["ore", "clay", "obsidian", "geode"];
    let time = 0;
    let mut best_geode = 0;

    // time, robot count, geode count
    let mut visited_states: HashSet<(
        usize,
        (usize, usize, usize, usize),
        (usize, usize, usize, usize),
    )> = HashSet::new();
    let mut min_time_to_state: HashMap<
        (
            (usize, usize, usize, usize),
            (usize, usize, usize, usize),
            i32,
        ),
        usize,
    > = HashMap::new();

    let mut traj_vec: Vec<(
        usize,
        (usize, usize, usize, usize),
        (usize, usize, usize, usize),
        i32,
    )> = Vec::new();
    traj_vec.push((time, item_count, robot_count, -1));

    while let Some((time, item_count, robot_count, choose_not)) = traj_vec.pop() {
        let tr = tl - time;

        if visited_states.contains(&(time, robot_count, item_count)) {
            continue;
        }

        let entry = min_time_to_state.get(&(item_count, robot_count, choose_not));

        best_geode = std::cmp::max(item_count.3 + (robot_count.3 * tr), best_geode);

        match entry {
            Some(t) => {
                if *t <= time {
                    continue;
                } else {
                    let entree = min_time_to_state
                        .entry((item_count, robot_count, choose_not))
                        .or_insert(time);
                    *entree = std::cmp::min(time, *entree);
                }
            }
            None => {
                min_time_to_state.insert((item_count, robot_count, choose_not), time);
            }
        }

        visited_states.insert((time, robot_count, item_count));

        if time == *tl {
            continue;
        }

        let resources_to_add = robot_count.to_owned();
        let mut best_could_build: i32 = -1;

        'inner: for (idx, c) in check_order.iter().rev().enumerate() {
            let (ore_req, clay_req, obs_req) = recipes.get(c).unwrap();

            if item_count.0 >= *ore_req && item_count.1 >= *clay_req && item_count.2 >= *obs_req {
                best_could_build = std::cmp::max(best_could_build, (3 - idx) as i32) as i32;
                let mut new_item_count = item_count.to_owned();
                let mut new_robot_count = robot_count.to_owned();

                new_item_count.0 -= ore_req;
                new_item_count.1 -= clay_req;
                new_item_count.2 -= obs_req;

                new_item_count.0 += resources_to_add.0;
                new_item_count.1 += resources_to_add.1;
                new_item_count.2 += resources_to_add.2;
                new_item_count.3 += resources_to_add.3;

                match idx {
                    0 => new_robot_count.3 += 1,
                    1 => new_robot_count.2 += 1,
                    2 => new_robot_count.1 += 1,
                    3 => new_robot_count.0 += 1,
                    _ => (),
                }

                if bound(&new_robot_count, &recipes) {
                    traj_vec.push((time + 1, new_item_count, new_robot_count, -1));
                }
            } else {
                let mut new_item_count = item_count.to_owned();
                let mut new_robot_count = robot_count.to_owned();

                match idx {
                    1 => {
                        if robot_count.1 == 0 {
                            continue 'inner;
                        }
                    }
                    0 => {
                        if robot_count.2 == 0 {
                            continue 'inner;
                        }
                    }
                    _ => (),
                }

                let ore_remaining = ore_req - std::cmp::min(item_count.0, *ore_req);
                let obs_remaining = obs_req - std::cmp::min(item_count.2, *obs_req);
                let clay_remaining = clay_req - std::cmp::min(item_count.1, *clay_req);

                let ttore =
                    ore_remaining / robot_count.0 + (ore_remaining % robot_count.0 != 0) as usize;
                let ttobs = if idx == 0 {
                    obs_remaining / robot_count.2 + (obs_remaining % robot_count.2 != 0) as usize
                } else {
                    0
                };
                let ttclay = if idx == 1 {
                    clay_remaining / robot_count.1 + (clay_remaining % robot_count.1 != 0) as usize
                } else {
                    0
                };
                let time_to = vec![ttore, ttobs, ttclay].into_iter().max().unwrap();

                new_item_count.0 += robot_count.0 * (time_to + 1);
                new_item_count.1 += robot_count.1 * (time_to + 1);
                new_item_count.2 += robot_count.2 * (time_to + 1);
                new_item_count.3 += robot_count.3 * (time_to + 1);

                new_item_count.0 -= ore_req;
                new_item_count.1 -= clay_req;
                new_item_count.2 -= obs_req;

                match idx {
                    0 => new_robot_count.3 += 1,
                    1 => new_robot_count.2 += 1,
                    2 => new_robot_count.1 += 1,
                    3 => new_robot_count.0 += 1,
                    _ => (),
                }

                if time + time_to < *tl {
                    traj_vec.push((time + time_to + 1, new_item_count, new_robot_count, -1));
                }
            }
        }
    }
    best_geode
}
