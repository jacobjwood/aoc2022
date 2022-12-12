use std::{fs, iter};
use std::collections::{VecDeque, HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("expected file to exist");

    println!("{}", contents);

    let (graph, s, e) = construct_graph(&contents);
    // println!("{}", graph.get(&(32, 163)).unwrap());
    println!("Part 1: {}", shortest_path(graph, e, s));

}

fn get_children(
    array: &Vec<Vec<char>>,
    index: &(usize, usize), 
    size_down: &usize, 
    size_across: &usize
) -> Vec<(usize, usize)> {


    let a_z = "abcdefghijklmnopqrstuvwxyz".chars();
    let heights : Vec<i32> = (1..=26).into_iter().collect();
    let mut mapping: HashMap<char, i32> = iter::zip(a_z, heights).collect();
    mapping.insert('E', 26);
    mapping.insert('S', 1);

    let (r, c) : (usize, usize) = *index;
    let node_value = mapping[&array[r][c]];

    let children = if r == 0 {
        if c == 0 {
            vec![(r, c+1), (r+1, c)]
        } else if c == *size_across - 1 {
            vec![(r, c-1), (r+1, c)]
        } else {
            vec![(r, c+1), (r, c-1), (r+1, c)]
        }
    } else if r == *size_down - 1 {
        if c == 0 {
            vec![(r, c+1), (r-1, c)]
        } else if c == *size_across - 1 {
            vec![(r, c-1), (r-1, c)]
        } else {
            vec![(r, c+1), (r, c-1), (r-1, c)]
        }
    } else {
        if c == 0 {
            vec![(r, c+1), (r+1, c), (r-1, c)]
        } else if c == *size_across - 1 {
            vec![(r, c-1), (r+1, c), (r-1, c)]
        } else {
            vec![(r, c+1), (r, c-1), (r+1, c), (r-1, c)]
        }
    };

    children.into_iter().filter(|(r, c)| (node_value - mapping[&array[*r][*c]]).abs() <= 1).collect()
}

fn construct_graph(contents: &str) -> (HashMap<(usize, usize), Vec<(usize, usize)>>, (usize, usize), (usize, usize)) {

    let array : Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let size_down = array.len();
    let size_across = array[0].len();
    let mut graph : HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let mut s = (0, 0);
    let mut e = (0, 0);

    for (r, line) in array.iter().enumerate() {
        for (c, letter) in line.iter().enumerate() {

            match letter {
                'S' => s = (r, c),
                'E' => e = (r, c),
                _ => (),
            }

            let children = get_children(&array, &(r, c), &size_down, &size_across);
            graph.insert((r, c), children);
        }
    }

    // println!("{:?}", graph);
    
    (graph, s, e)
}


fn shortest_path(
    graph: HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let mut dist : HashMap<(usize, usize), usize> = HashMap::new(); //graph.to_owned().into_iter().map(|(node, _)| (node, usize::MAX)).collect();
    *dist.entry(start).or_insert(0) = 0;
    let mut priority_queue = dist.to_owned().into_iter().map(|(node, distance)| (distance, node)).collect::<Vec<(usize, (usize, usize))>>();
    priority_queue.sort_by(|a, b| a.0.cmp(&b.0));
    let mut priority_queue = VecDeque::from(priority_queue);
    let mut visited = HashSet::<(usize, usize)>::new();

    println!("{:?}", priority_queue);
    
    while !priority_queue.is_empty() {
        let u = priority_queue.pop_front().unwrap();
        visited.insert(u.1);
        // println!("PQ BEFORE {:?}", priority_queue);
        let u_dist = u.0;
        let u_index = u.1;

        println!("{:?}", priority_queue.len());
        println!("{} {:?}", u_dist, u_index);

        // println!("{}", u_dist);
        println!("{}", graph.len());
        let children = graph.get(&u_index).unwrap();
        println!("{}", graph.len());
        for child in children {
            let entry = dist.entry(*child).or_insert(usize::MAX);
            let new_dist = u_dist + 1;
            if new_dist < *entry {
                *entry = new_dist;   
            }


        }
        // Need some form of checking here and alter the value
        let mut tmp_priority_queue = dist.to_owned().into_iter().filter(|(node, distance)| !visited.contains(&node)).map(|(node, distance)| (distance, node)).collect::<Vec<(usize, (usize, usize))>>();
        tmp_priority_queue.sort_by(|a, b| a.0.cmp(&b.0));
        priority_queue = VecDeque::from(tmp_priority_queue);
        // println!("PQ AFTER {:?}", priority_queue);
        //println!("{:?}", graph.get(&u_index).unwrap());
        println!("LEN PQ {:?}", priority_queue.len());
        println!("DIST LEN {:?}", dist.len());
    }
    
    *dist.get(&goal).unwrap()
}