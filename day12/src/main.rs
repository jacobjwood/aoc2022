use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::{fs, iter};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("expected file to exist");

    let (graph, s, e) = construct_graph(&contents);

    println!("Part 1: {}", shortest_path(&graph, s, e));

    let mut dist_vec: Vec<usize> = Vec::new();
    let a_states = contents
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, letter)| *letter == 'a')
                .map(move |(c, _)| (r, c))
        })
        .collect::<Vec<(usize, usize)>>();
    for a in &a_states {
        dist_vec.push(shortest_path(&graph, *a, e));
    }
    println!("Part 2: {:?}", dist_vec.iter().min().unwrap());
}

fn get_children(
    array: &Vec<Vec<char>>,
    index: &(usize, usize),
    size_down: &usize,
    size_across: &usize,
) -> Vec<(usize, usize)> {
    let a_z = "abcdefghijklmnopqrstuvwxyz".chars();
    let heights: Vec<i32> = (1..=26).into_iter().collect();
    let mut mapping: HashMap<char, i32> = iter::zip(a_z, heights).collect();
    mapping.insert('E', 26);
    mapping.insert('S', 1);

    let (r, c): (usize, usize) = *index;
    let node_value = mapping[&array[r][c]];

    let children = if r == 0 {
        if c == 0 {
            vec![(r, c + 1), (r + 1, c)]
        } else if c == *size_across - 1 {
            vec![(r, c - 1), (r + 1, c)]
        } else {
            vec![(r, c + 1), (r, c - 1), (r + 1, c)]
        }
    } else if r == *size_down - 1 {
        if c == 0 {
            vec![(r, c + 1), (r - 1, c)]
        } else if c == *size_across - 1 {
            vec![(r, c - 1), (r - 1, c)]
        } else {
            vec![(r, c + 1), (r, c - 1), (r - 1, c)]
        }
    } else {
        if c == 0 {
            vec![(r, c + 1), (r + 1, c), (r - 1, c)]
        } else if c == *size_across - 1 {
            vec![(r, c - 1), (r + 1, c), (r - 1, c)]
        } else {
            vec![(r, c + 1), (r, c - 1), (r + 1, c), (r - 1, c)]
        }
    };

    children
        .into_iter()
        .filter(|(r, c)| (mapping[&array[*r][*c]] - node_value) <= 1)
        .collect()
}

fn construct_graph(
    contents: &str,
) -> (
    HashMap<(usize, usize), Vec<(usize, usize)>>,
    (usize, usize),
    (usize, usize),
) {
    let array: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let size_down = array.len();
    let size_across = array[0].len();
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

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

    (graph, s, e)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    distance: usize,
    index: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

// Needed this otherwise the ordering doesn't work
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = graph
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
        if index == goal {
            return distance;
        }

        if distance > *dist.get(&index).unwrap() {
            continue;
        }

        for child in graph.get(&index).unwrap() {
            let next = State {
                distance: distance + 1,
                index: *child,
            };

            if next.distance < *dist.get(&child).unwrap() {
                heap.push(next);
                *dist.entry(next.index).or_insert(usize::MAX) = next.distance;
            }
        }
    }

    // need this in case of loop not executing
    usize::MAX

}
