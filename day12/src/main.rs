use std::{fs, iter};
use std::collections::{VecDeque, HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("expected file to exist");

    println!("{}", contents);

    let (graph, s, e, array) = construct_graph(&contents);
    // println!("{}", graph.get(&(32, 163)).unwrap());
    println!("{:?}", s);
    println!("Part 1: {}", shortest_path(&graph, s, e, &array, false));
    // println!("CHILDREN OF ERROR {:?}", graph.get(&(33, 163)).unwrap());

    let mut dist_vec : Vec<usize> = Vec::new();
    let a_states = contents.lines().enumerate().flat_map(|(r, line)| line.chars().enumerate().filter(|(c, letter)| *letter == 'a').map(move |(c, letter)| (r, c))).collect::<Vec<(usize, usize)>>();
    for a in &a_states {
        dist_vec.push(shortest_path(&graph, *a, e, &array, false));
    }
    println!("Part 2: {:?}", dist_vec.iter().min().unwrap());

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

    children.into_iter().filter(|(r, c)| (mapping[&array[*r][*c]] - node_value) <= 1).collect()
}

fn construct_graph(contents: &str) -> (HashMap<(usize, usize), Vec<(usize, usize)>>, (usize, usize), (usize, usize), Vec<Vec<char>>) {

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
    
    (graph, s, e, array)
}


fn shortest_path(
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    goal: (usize, usize),
    array: &Vec<Vec<char>>,
    print_grid: bool,
) -> usize {
    let mut dist : HashMap<(usize, usize), usize> = HashMap::new(); //graph.to_owned().into_iter().map(|(node, _)| (node, usize::MAX)).collect();
    *dist.entry(start).or_insert(0) = 0;
    let mut priority_queue = dist.to_owned().into_iter().map(|(node, distance)| (distance, node)).collect::<Vec<(usize, (usize, usize))>>();
    priority_queue.sort_by(|a, b| a.0.cmp(&b.0));
    let mut priority_queue = VecDeque::from(priority_queue);
    let mut visited = HashSet::<(usize, usize)>::new();

    // println!("{:?}", priority_queue);
    
    while !priority_queue.is_empty() {
        let u = priority_queue.pop_front().unwrap();
        // println!("{}", u.0);
        visited.insert(u.1);
        // println!("PQ BEFORE {:?}", priority_queue);
        let u_dist = u.0;
        let u_index = u.1;

        // println!("{:?}", priority_queue.len());
        // println!("{} {:?}", u_dist, u_index);

        // println!("{}", u_dist);
        // println!("{}", graph.len());
        let children = graph.get(&u_index).unwrap();
        // println!("{}", graph.len());
        for child in children {

            if !visited.contains(child) {
                
                let entry = dist.entry(*child).or_insert(usize::MAX);
                let new_dist = u_dist + 1;
                if new_dist < *entry {
                    *entry = new_dist;   
                    //priority_queue.push_back((*entry, *child));
                
                }
            }

        }
        // Need some form of checking here and alter the value
        let mut tmp_priority_queue = dist.to_owned().into_iter().filter(|(node, distance)| !visited.contains(&node)).map(|(node, distance)| (distance, node)).collect::<Vec<(usize, (usize, usize))>>();
        tmp_priority_queue.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("{:?}", tmp_priority_queue);
        priority_queue = VecDeque::from(tmp_priority_queue);
        // println!("PQ AFTER {:?}", priority_queue);
        // println!("CHILDREN OF ERROR {:?}", graph.get(&u_index).unwrap());
        // println!("LEN PQ {:?}", priority_queue.len());
        // println!("DIST LEN {:?}", dist.len());
    }
    
    if print_grid {
        let max_visit_r = visited.iter().map(|(r, c)| r).max().unwrap();
        let max_visit_c = visited.iter().map(|(r, c)| c).max().unwrap();

        println!("{} {}", max_visit_r, max_visit_c);
        let mut grid = Vec::new();
        for _ in 0..=*max_visit_r {
            grid.push(
                (0..=*max_visit_c).map(|_| ".".to_string()).collect::<Vec<String>>()
            )
        }
        for visit in visited {
            let (r, c) = visit;
            grid[r][c] = array[r][c].to_string();
            if (r, c) == start {
                grid[r][c] = "S".to_string();
            } else if (r, c) == goal {
                grid[r][c] = "E".to_string();
            }
        }

        // grid[goal.0][goal.1] = "E".to_string();
        for line in grid {
            println!("{:?}", line.into_iter().collect::<String>());
        }
    }

    match dist.get(&goal) {
        Some(distance) => *distance,
        None => usize::MAX,
    }
    
}