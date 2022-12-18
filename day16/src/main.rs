use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::iter::zip;

struct Valve {
    flow_rate: usize,
    neighbors: Vec<Valve>,
    distances: HashMap<Valve, usize>,
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents = contents.replace("valve ", "valves ");

    let valves = &contents
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>();

    let flow_rates = &contents
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        }) //[2].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!(
        "{}",
        flow_rates
            .iter()
            .filter(|fr| **fr != 0)
            .map(|fr| *fr)
            .collect::<Vec<usize>>()
            .len()
    );

    let neighbors = &contents
        .lines()
        .map(|l| l.split("to valves ").collect::<Vec<&str>>()[1])
        .map(|l| l.split(", ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut valve_flow_rates: HashMap<&str, usize> = HashMap::new();

    for (valve, neighbors) in zip(valves, neighbors) {
        graph.insert(valve, neighbors.to_owned());
    }

    for (valve, flow_rate) in zip(valves, flow_rates) {
        valve_flow_rates.insert(valve, *flow_rate);
    }

    let mut valve_distances: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for valve in valves.iter() {
        let distances = shortest_path(&graph, valve);

        // println!("{} : {:?}", valve, distances);
        valve_distances.insert(valve, distances);
    }

    let mut current_val = valves[0];
    let mut pressure_released = 0;
    let mut current_flow_rate = 0;
    let mut time = 0;

    for i in 1..=30 {
        let time_remaining = 30 - i;

        // get best destination
    }

    get_best_valve(&valve_flow_rates, &30, &"AA", &valve_distances);

    // DD BB JJ HH EE CC
    //traversal(&valve_flow_rates, &valve_distances, &"AA", 30);
    // 20 minutes => score_count = 2041569;
    // 21 minutes => score_count = 3994918;
    // 22 minutes => score_count = ;
    traversal_w_e(&valve_flow_rates, &valve_distances, &"AA", 26);
}

fn traversal(
    vfr: &HashMap<&str, usize>,
    vd: &HashMap<&str, HashMap<&str, usize>>,
    start_state: &str,
    max_time: usize,
) {
    // history, current time, total flow rate, cumulative flow
    let mut traj_vec: Vec<(Vec<&str>, usize, usize, usize)> = Vec::new();
    traj_vec.push((vec![&start_state], 0, 0, 0));
    let mut scores: Vec<usize> = Vec::new();

    while let Some((mut traj, mut ct, mut tfr, mut cf)) = traj_vec.pop() {
        if ct == max_time {
            scores.push(cf + tfr);
            continue;
        }

        let traj_set: HashSet<&str> = HashSet::from_iter(traj.iter().cloned());
        let state = traj.last().unwrap();
        let mut poss_next: Vec<(&str, usize)> = vd
            .get(state)
            .unwrap()
            .to_owned()
            .into_iter()
            .filter(|(k, v)| {
                !traj_set.contains(*k) && *vfr.get(k).unwrap() != 0 && (v + 1) < (max_time - ct)
            })
            .collect();

        if poss_next.len() == 0 {
            for t in ct + 1..=max_time {
                cf += tfr;
                ct += 1;
                if false {
                    println!(
                        "CURRENT TIME: {}, TOTAL FLOW RATE: {}, CUMULATIVE FLOW: {}, TRAJ: {:?}",
                        ct, tfr, cf, traj
                    );
                }
            }
            scores.push(cf);
            continue;
        }

        for (k, v) in &poss_next {
            let flow_rate = vfr.get(k).unwrap();
            let distance_to_and_turn_on = v + 1;
            let time_after_turning_on = distance_to_and_turn_on + ct;

            if time_after_turning_on <= max_time {
                let mut new_traj = traj.to_owned();
                new_traj.push(k);
                let to_push = (
                    new_traj,
                    time_after_turning_on,
                    tfr + flow_rate,
                    cf + (tfr * (distance_to_and_turn_on)),
                );
                traj_vec.push(to_push);
            }
        }
    }

    scores.sort_by(|a, b| b.cmp(&a));
    println!("MAX SCORE {:?}", scores.iter().max());
    println!("{}", scores.len());
}

#[derive(Clone)]
struct MeOrE<'a> {
    trajectory: Vec<&'a str>,
    round_opened: Vec<usize>,
    flow_rate_after_round: Vec<usize>,
    time: usize,
    flow_rate: usize,
    total_flow: usize,
}

fn traversal_w_e(
    vfr: &HashMap<&str, usize>,
    vd: &HashMap<&str, HashMap<&str, usize>>,
    start_state: &str,
    max_time: usize,
) {
    // me, elephant, cumulative flow
    let mut traj_vec: Vec<(MeOrE, MeOrE)> = Vec::new();

    let mut me = MeOrE {
        trajectory: vec![&"AA"],
        round_opened: vec![0],
        flow_rate_after_round: Vec::new(),
        time: 0,
        flow_rate: 0,
        total_flow: 0,
    };

    let mut elephant = MeOrE {
        trajectory: vec![&"AA"],
        round_opened: vec![0],
        flow_rate_after_round: Vec::new(),
        time: 0,
        flow_rate: 0,
        total_flow: 0,
    };

    traj_vec.push((me, elephant));
    //let mut scores : Vec<usize> = Vec::new();
    let mut best_score = 0;
    let mut scores_count = 0;

    let best_flow_rate: usize = vfr.iter().map(|(k, v)| v).sum::<usize>();
    let mut min_flow_time: usize = max_time;
    println!("best_flow_rate {}", best_flow_rate);

    while let Some((mut me, mut elephant)) = traj_vec.pop() {
        if me.flow_rate + elephant.flow_rate == best_flow_rate {
            println!("BFR REACH");
            std::thread::sleep_ms(3000);
        }

        // first off generate the set of possible states left to explore for the free agents
        let traj_me_set: HashSet<&str> = HashSet::from_iter(me.trajectory.iter().cloned());
        let traj_e_set: HashSet<&str> = HashSet::from_iter(elephant.trajectory.iter().cloned());
        let traj_set: HashSet<&str> = traj_me_set.union(&traj_e_set).map(|s| *s).collect();

        // get our current states (although we may not be free yet)
        let my_state = me.trajectory.last().unwrap();
        let e_state = elephant.trajectory.last().unwrap();

        // get the current time. this is the minimum of the times we are in
        let ct = std::cmp::min(me.time, elephant.time);

        // keeps track of best flow rates for each ct
        let mut bfr_map: HashMap<usize, usize> = HashMap::new();
        bfr_map.insert(0, 0);

        // println!("Current time {}", ct);

        // if its my turn to choose only, then this becomes like part 1
        if ct == me.time && ct != elephant.time {
            // println!("It's my time");

            let state = me.trajectory.last().unwrap();

            let mut poss_next: Vec<(&str, usize)> = vd
                .get(state)
                .unwrap()
                .to_owned()
                .into_iter()
                .filter(|(k, v)| {
                    !traj_set.contains(*k) && *vfr.get(k).unwrap() != 0 && (v + 1) < (max_time - ct)
                })
                .collect();

            poss_next.sort_by(|a, b| b.1.cmp(&a.1));

            // if out of options, then we need to coerce the times
            if poss_next.len() == 0 {
                let mut new_me = me.to_owned();
                new_me.total_flow += new_me.flow_rate * (elephant.time - new_me.time);
                new_me.time = elephant.time;
                traj_vec.push((new_me, elephant.to_owned()));
            }

            // iterate over all possible next states
            for (k, v) in &poss_next {
                // println!("{:?} {:?}", k, v);
                let flow_rate = vfr.get(k).unwrap();
                let distance_to_and_turn_on = v + 1;
                let time_after_turning_on = distance_to_and_turn_on + ct;

                if time_after_turning_on <= max_time {
                    //println!("{} {} {} {}", k, ct, distance_to_and_turn_on, flow_rate);
                    let mut new_traj = me.trajectory.to_owned();
                    let mut new_ro = me.round_opened.to_owned();
                    new_ro.push(time_after_turning_on);
                    new_traj.push(k);
                    let mut frar = me.flow_rate_after_round.to_owned();
                    frar.push(me.flow_rate + flow_rate);
                    let new_me = MeOrE {
                        trajectory: new_traj,
                        round_opened: new_ro,
                        flow_rate_after_round: frar,
                        time: time_after_turning_on,
                        flow_rate: me.flow_rate + flow_rate,
                        total_flow: me.flow_rate * distance_to_and_turn_on,
                    };
                    let to_push = (new_me, elephant.to_owned());
                    traj_vec.push(to_push);
                }
            }
        } else if ct == elephant.time && ct != me.time {
            // println!("It's elephant time");

            let state = elephant.trajectory.last().unwrap();

            let mut poss_next: Vec<(&str, usize)> = vd
                .get(state)
                .unwrap()
                .to_owned()
                .into_iter()
                .filter(|(k, v)| {
                    !traj_set.contains(*k) && *vfr.get(k).unwrap() != 0 && (v + 1) < (max_time - ct)
                })
                .collect();

            poss_next.sort_by(|a, b| b.1.cmp(&a.1));

            if poss_next.len() == 0 {
                let mut new_elephant = elephant.to_owned();
                new_elephant.total_flow += new_elephant.flow_rate * (me.time - new_elephant.time);
                new_elephant.time = me.time;
                traj_vec.push((me.to_owned(), new_elephant));
            }

            // iterate over all possible next states
            for (k, v) in &poss_next {
                let flow_rate = vfr.get(k).unwrap();
                let distance_to_and_turn_on = v + 1;
                let time_after_turning_on = distance_to_and_turn_on + ct;

                if time_after_turning_on <= max_time {
                    let mut new_traj = elephant.trajectory.to_owned();
                    let mut new_ro = elephant.round_opened.to_owned();
                    new_traj.push(k);
                    new_ro.push(time_after_turning_on);
                    let mut frar = elephant.flow_rate_after_round.to_owned();
                    frar.push(elephant.flow_rate + flow_rate);
                    let new_elephant = MeOrE {
                        trajectory: new_traj,
                        round_opened: new_ro,
                        flow_rate_after_round: frar,
                        time: time_after_turning_on,
                        flow_rate: elephant.flow_rate + flow_rate,
                        total_flow: elephant.flow_rate * distance_to_and_turn_on,
                    };
                    let to_push = (me.to_owned(), new_elephant);
                    traj_vec.push(to_push);
                }
            }
        } else {
            let me_state = me.trajectory.last().unwrap();
            let e_state = elephant.trajectory.last().unwrap();

            let mut poss_next_me: Vec<(&str, usize)> = vd
                .get(me_state)
                .unwrap()
                .to_owned()
                .into_iter()
                .filter(|(k, v)| {
                    !traj_set.contains(*k) && *vfr.get(k).unwrap() != 0 && (v + 1) < (max_time - ct)
                })
                .collect();

            let mut poss_next_e: Vec<(&str, usize)> = vd
                .get(e_state)
                .unwrap()
                .to_owned()
                .into_iter()
                .filter(|(k, v)| {
                    !traj_set.contains(*k) && *vfr.get(k).unwrap() != 0 && (v + 1) < (max_time - ct)
                })
                .collect();

            poss_next_me.sort_by(|a, b| b.1.cmp(&a.1));
            poss_next_e.sort_by(|a, b| b.1.cmp(&a.1));

            // if space is exhausted, just tally up flow rates
            if poss_next_me.is_empty() && poss_next_e.is_empty() {
                me.flow_rate_after_round.push(0);
                me.round_opened.push(max_time);
                elephant.round_opened.push(max_time);
                me.flow_rate_after_round.sort();
                elephant.flow_rate_after_round.push(0);
                elephant.flow_rate_after_round.sort();

                let mut score1 = 0;

                for idx in 0..me.round_opened.len() - 1 {
                    score1 += (me.round_opened[idx + 1] - me.round_opened[idx])
                        * me.flow_rate_after_round[idx];
                }

                for idx in 0..elephant.round_opened.len() - 1 {
                    score1 += (elephant.round_opened[idx + 1] - elephant.round_opened[idx])
                        * elephant.flow_rate_after_round[idx];
                }

                me.total_flow += (max_time - ct) * me.flow_rate;
                elephant.total_flow += (max_time - ct) * elephant.flow_rate;
                best_score = std::cmp::max(best_score, score1);
                scores_count += 1;
            } else if poss_next_me.is_empty() && !poss_next_e.is_empty() {
                me.total_flow += (max_time - ct) * me.flow_rate;
                me.time = max_time;
                traj_vec.push((me.to_owned(), elephant.to_owned()));
            } else if !poss_next_me.is_empty() && poss_next_e.is_empty() {
                elephant.total_flow += (max_time - ct) * elephant.flow_rate;
                elephant.time = max_time;
                traj_vec.push((me.to_owned(), elephant.to_owned()));
            } else {
                // only need to run through one version of this
                let e_state = e_state;

                let mut poss_next = poss_next_e;

                if poss_next.len() == 0 {
                    println!("AM HERE");
                    let mut new_elephant = elephant.to_owned();
                    new_elephant.total_flow +=
                        new_elephant.flow_rate * (me.time - new_elephant.time);
                    new_elephant.time = me.time;
                    traj_vec.push((me.to_owned(), new_elephant));
                }

                // iterate over all possible next states
                for (k, v) in &poss_next {
                    // println!("{:?} {:?}", k, v);
                    let flow_rate = vfr.get(k).unwrap();
                    let distance_to_and_turn_on = v + 1;
                    let time_after_turning_on = distance_to_and_turn_on + ct;

                    if time_after_turning_on <= max_time {
                        //println!("{} {} {} {}", k, ct, distance_to_and_turn_on, flow_rate);
                        let mut new_traj = elephant.trajectory.to_owned();
                        let mut new_ro = elephant.round_opened.to_owned();
                        new_traj.push(k);
                        new_ro.push(time_after_turning_on);
                        let mut frar = elephant.flow_rate_after_round.to_owned();
                        frar.push(elephant.flow_rate + flow_rate);
                        let new_elephant = MeOrE {
                            trajectory: new_traj,
                            round_opened: new_ro,
                            flow_rate_after_round: frar,
                            time: time_after_turning_on,
                            flow_rate: elephant.flow_rate + flow_rate,
                            total_flow: elephant.flow_rate * distance_to_and_turn_on,
                        };
                        let to_push = (me.to_owned(), new_elephant);
                        traj_vec.push(to_push);
                    }
                }
            }
        }
    }

    println!("MAX SCORE {:?}", best_score);
    println!("{}", scores_count);
}

fn get_best_valve(
    flow_rates: &HashMap<&str, usize>,
    time_remaining: &usize,
    current_valve: &str,
    distances: &HashMap<&str, HashMap<&str, usize>>,
) {
    let mut best_potential_flow = 0;
    let mut best_valve = current_valve;

    for (valve, flow_rate) in flow_rates.iter().filter(|(v, fr)| *fr > &0) {
        let ttov = distances.get(current_valve).unwrap().get(valve).unwrap() + 1;
        let pf = (time_remaining - ttov) * flow_rate;
        println!("{} TTV {} PF {}", valve, ttov, pf);

        if pf > best_potential_flow {
            best_potential_flow = pf;
            best_valve = valve;
        }
    }

    println!("Best valve {}", best_valve);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    distance: usize,
    index: &'a str,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

// Needed this otherwise the ordering doesn't work
impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
) -> HashMap<&'a str, usize> {
    let mut dist: HashMap<&str, usize> = graph
        .to_owned()
        .into_iter()
        .map(|(node, _)| (node, usize::MAX))
        .collect();
    *dist.entry(start).or_insert(0) = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        distance: 0,
        index: start,
    });

    while let Some(State { distance, index }) = heap.pop() {
        if distance > *dist.get(index).unwrap() {
            continue;
        }

        for child in graph.get(index).unwrap() {
            let next = State {
                distance: distance + 1,
                index: *child,
            };

            if next.distance < *dist.get(child).unwrap() {
                heap.push(next);
                *dist.entry(next.index).or_insert(usize::MAX) = next.distance;
            }
        }
    }

    dist
}
