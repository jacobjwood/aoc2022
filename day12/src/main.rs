use std::{fs, iter};
use std::collections::{VecDeque, HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: (usize, usize),
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<(usize, usize), _> = iter::zip(adj_list(0..adj_list.len()).map(|_| usize::MAX)).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    let graph = vec![
        // Node 0
        vec![Edge { node: (2, 2), cost: 10 },
             Edge { node: (1, 1), cost: 1 }],
        // Node 1
        vec![Edge { node: (3, 3), cost: 2 }],
        // Node 2
        vec![Edge { node: (1, 1), cost: 1 },
             Edge { node: (3, 3), cost: 3 },
             Edge { node: (4, 4), cost: 1 }],
        // Node 3
        vec![Edge { node: (0, 0), cost: 7 },
             Edge { node: (4, 4), cost: 2 }],
        // Node 4
        vec![]];

    assert_eq!(shortest_path(&graph, (0, 0), (1, 1)), Some(1));
    assert_eq!(shortest_path(&graph, (0, 0), (3, 3)), Some(3));
    assert_eq!(shortest_path(&graph, (3, 0), (0, 0)), Some(7));
    assert_eq!(shortest_path(&graph, (0, 0), (4, 4)), Some(5));
    assert_eq!(shortest_path(&graph, (4, 4), (0, 0)), None);
}

/*

#[derive(Debug)]
struct Node {
    curr_count: u32,
    letter: char,
    value: i32,
    parent: (usize, usize),
    index: (usize, usize),
    visited: HashSet<(usize, usize)>,
}

fn get_children(index: &(usize, usize), size_down: &usize, size_across: &usize, ) -> Vec<(usize, usize)> {

    let (mut r, mut c) = index;

    if r == 0 {
        if c == 0 {
            vec![(r+1, c), (r, c+1)]
        } else if c == size_across - 1 {
            vec![(r+1, c), (r, c-1)]
        } else {
            vec![(r+1, c), (r, c+1), (r, c-1)]
        }
    } else if r == size_down - 1 {
        if c == 0 {
            vec![(r-1, c), (r, c+1)]
        } else if c == size_across - 1 {
            vec![(r-1, c), (r, c-1)]
        } else {
            vec![(r-1, c), (r, c-1), (r, c+1)]
        }
    } else {
        if c == 0 {
            vec![(r+1, c), (r-1, c), (r, c+1)]
        } else if c == size_across - 1 {
            vec![(r+1, c), (r-1, c), (r, c-1)]
        } else {
            vec![(r+1, c), (r-1, c), (r, c+1), (r, c-1)]
        }
    }



}

fn find_e(height_array: &Vec<Vec<char>>, start_index: &(usize, usize), mapping: &HashMap<char, i32>) {

    let value = 1;
    let curr_count = 0;

    let size_down = height_array.len();
    let size_across = height_array[0].len();

    // Children
    let mut to_search : VecDeque<Node> = VecDeque::new();
    let children = get_children(start_index, &size_down, &size_across);

    //to_search.push_back(Node { curr_count : 0, letter : height_array[start_index.0][start_index.1], })
    

    println!("{:?}", children);

    for child in children {

        println!("{:?}", child);

        let child_letter = height_array[child.0][child.1];

        let child_value = mapping[&child_letter];

        if (value - child_value).abs() <= 1 {
        
            to_search.push_back(Node { curr_count : curr_count, letter : child_letter, value : child_value, parent : *start_index, index : child, visited : HashSet::from([*start_index])} )

        }
        println!("{}", child_value);
    }

    while !to_search.is_empty() {

        //println!("{:?}", to_search);
        let node = to_search.pop_front().unwrap();
        //println!("{:?}", to_search);
        

        let curr_count = node.curr_count + 1;
        let value = node.value;
        let parent = node.parent;
        let letter = node.letter;
        let index = node.index;
        let mut visited = node.visited;
        visited.insert(index);

        if letter == 'z' {
            println!("Part 1: {}", curr_count);
            break;
        }

        let children = get_children(&index, &size_down, &size_across);

        
        //to_search.push_back(Node { curr_count : 0, letter : height_array[start_index.0][start_index.1], })
        

        //println!("{:?}", children);

        for child in children {

            //println!("{:?}", child);

            let child_letter = height_array[child.0][child.1];

            let child_value = mapping[&child_letter];

            if (value - child_value).abs() <= 1 && child != parent && !visited.contains(&child) {
            
                to_search.push_back(Node { curr_count : curr_count, letter : child_letter, value : child_value, parent : index, index : child, visited: visited.clone()} )

            }
            //println!("{}", child_value);
        }
    }


}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let height_array = contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let a_z: Vec<char> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let heights: Vec<i32> = (1..=26).into_iter().collect();
    let mut mapping: HashMap<char, i32> = iter::zip(a_z, heights).collect::<HashMap<char, i32>>();
    mapping.insert('E', 26);
    mapping.insert('S', 1);

    let mut s_index = (0, 0);
    let mut e_index = (0, 0);

    for (r, line) in height_array.iter().enumerate() {
        for (c, letter) in line.iter().enumerate() {
            match letter {
                'S' => s_index = (r, c),
                'E' => e_index = (r, c),
                _ => (),
            }
        }
    }

    println!("{:?}, {:?}", s_index, e_index);

    find_e(&height_array, &s_index, &mapping);

}
*/