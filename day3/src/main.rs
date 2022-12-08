use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let backpacks = contents.split("\n").filter(|x| !x.is_empty());

    let mut running_total = 0;
    let a_Z: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let priorities: Vec<i32> = (1..=52).into_iter().collect();
    let mapping: HashMap<char, i32> = iter::zip(a_Z, priorities).collect::<HashMap<char, i32>>();

    for backpack in backpacks {
        let bpack = String::from(backpack);
        let backpack_size = &bpack.len();
        let compartment_1 = &bpack[0..backpack_size / 2];
        let compartment_2 = &bpack[backpack_size / 2..];
        let set1: HashSet<char> = compartment_1.chars().collect();
        let set2: HashSet<char> = compartment_2.chars().collect();
        let intersection = set1
            .intersection(&set2)
            .into_iter()
            .collect::<String>()
            .chars()
            .collect::<Vec<char>>()[0];
        running_total += mapping.get(&intersection).unwrap();
    }
    println!("Part 1: {}", running_total);

    // Part 2 - rough
    let backpacks = contents.split("\n").filter(|x| !x.is_empty());
    let backpack_vec = backpacks.clone().collect::<Vec<&str>>();
    let mut running_total = 0;

    for backpack_3 in backpack_vec.chunks(3) {
        if let [pack1, pack2, pack3] = backpack_3[..] {
            let pack1_set: HashSet<char> = pack1.chars().collect();
            let pack2_set: HashSet<char> = pack2.chars().collect();
            let pack3_set: HashSet<char> = pack3.chars().collect();

            let intersection = pack1_set.intersection(&pack2_set).collect::<String>();
            let intersection: HashSet<char> = intersection.chars().collect();
            let intersection = intersection
                .intersection(&pack3_set)
                .collect::<String>()
                .chars()
                .collect::<Vec<char>>()[0];
            running_total += mapping.get(&intersection).unwrap();
        }
    }

    println!("Part 2: {}", running_total);
}
