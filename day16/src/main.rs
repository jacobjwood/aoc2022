use std::fs;
use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::iter::zip;

struct Valve {
    flow_rate: usize,
    neighbors: Vec<Valve>,
    distances: HashMap<Valve, usize>,
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let contents = contents.replace("valve ", "valves ");

    let valves = &contents
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>();

    let flow_rates = &contents
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<usize>().unwrap()) //[2].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("{}", flow_rates.iter().filter(|fr| **fr != 0).map(|fr| *fr).collect::<Vec<usize>>().len());

    let neighbors = &contents.lines()
        .map(|l| l.split("to valves ").collect::<Vec<&str>>()[1])
        .map(|l| l.split(", ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut graph : HashMap<&str, Vec<&str>> = HashMap::new();
    let mut valve_flow_rates : HashMap<&str, usize> = HashMap::new();

    for (valve, neighbors) in zip(valves, neighbors) {
        graph.insert(valve, neighbors.to_owned());
    }

    for (valve, flow_rate) in zip(valves, flow_rates) {
        valve_flow_rates.insert(valve, *flow_rate);
    }

    let mut valve_distances : HashMap<&str, HashMap<&str, usize>> = HashMap::new();

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
    traversal(&valve_flow_rates, &valve_distances, &"AA", 30);
}

fn traversal(vfr: &HashMap<&str, usize>, vd: &HashMap<&str, HashMap<&str, usize>>, start_state: &str, max_time: usize) {
    // history, current time, total flow rate, cumulative flow
    let mut traj_vec : Vec<(Vec<&str>, usize, usize, usize)> = Vec::new();
    traj_vec.push((vec![&start_state], 0, 0, 0));
    let mut scores : Vec<usize> = Vec::new();

    while let Some((mut traj, mut ct, mut tfr, mut cf)) = traj_vec.pop() {
        //println!("{}", ct);

        //println!("{:?}", traj);

        if false {
            println!("CURRENT TIME: {}, TOTAL FLOW RATE: {}, CUMULATIVE FLOW: {}", ct, tfr, cf);
            println!("  JEFF");
            println!("  JEFF TRAJ {:?}", traj);
            std::thread::sleep_ms(4000);
        }

        if ct == max_time {
            scores.push(cf+tfr); //+tfr);
            continue;
        }

        let traj_set : HashSet<&str> = HashSet::from_iter(traj.iter().cloned());
        let state = traj.last().unwrap();
        //println!("{} {}", max_time, ct);
        let mut poss_next : Vec<(&str, usize)> = vd
            .get(state)
            .unwrap()
            .to_owned()
            .into_iter()
            .filter(|(k, v)| !traj_set.contains(*k) && *vfr.get(k).unwrap() != 0 && (v + 1) < (max_time - ct))
            .collect();
        // poss_next.sort_by(|a, b| a.0.cmp(&b.0));

        if poss_next.len() == 0 {
            // println!("Current time {}", ct);
            // println!("{} {}", ct, max_time);
            for t in ct+1..=max_time {
                cf += tfr;
                ct += 1;
                if false {
                    println!("CURRENT TIME: {}, TOTAL FLOW RATE: {}, CUMULATIVE FLOW: {}, TRAJ: {:?}", ct, tfr, cf, traj); 
                }
            }
            scores.push(cf);
            continue;
        }

        for (k, v) in &poss_next {
            // println!("{:?} {:?}", k, v);
            let flow_rate = vfr.get(k).unwrap();
            let distance_to_and_turn_on = v + 1;
            let time_after_turning_on = distance_to_and_turn_on + ct;

            if time_after_turning_on <= max_time {

                //println!("{} {} {} {}", k, ct, distance_to_and_turn_on, flow_rate);
                let mut new_traj = traj.to_owned();
                new_traj.push(k);
                //println!("PUSHING");
                //println!("{}", flow_rate);
                let to_push = (new_traj, time_after_turning_on, tfr + flow_rate, cf + (tfr * (distance_to_and_turn_on)));
                // println!("{:?}", to_push);
                traj_vec.push(to_push);
            }
        }
        // println!("{:?}", poss_next);
        /*
        if traj_vec.len() > 1 {
            traj_vec.sort_by(|a, b| a.2.cmp(&b.2));
            traj_vec = vec![traj_vec.pop().unwrap()];
        }
        */
        
    }

    scores.sort_by(|a, b| b.cmp(&a));
    //let scores_print = scores[0..10].to_owned();
    println!("MAX SCORE {:?}", scores.iter().max());
    println!("{}", scores.len());
}

fn traversal2(vfr: &HashMap<&str, usize>, vd: &HashMap<&str, HashMap<&str, usize>>, max_time: usize) {

    // Have a HashMap of distances
    // start from initial state


}

fn get_best_valve(
    flow_rates: &HashMap<&str, usize>, 
    time_remaining: &usize, 
    current_valve: &str, 
    distances: &HashMap<&str, HashMap<&str, usize>>) {
    
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