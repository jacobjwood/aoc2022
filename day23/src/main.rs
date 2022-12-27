use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut elf_set = parse_input(&contents);
    //let mut elf_states = elf_set.iter().map(|elf| (elf.to_owned(), 0)).collect::<HashMap<_, _>>();
    let mut elf_state = 0;

    print_grid(&elf_set);

    for _ in 0..10 {
        simulate_round(&mut elf_set, &mut elf_state);
        print_grid(&elf_set);
    }

    let empty_spaces = (elf_set.iter().map(|x| x.0).max().unwrap()
        - elf_set.iter().map(|x| x.0).min().unwrap()
        + 1)
        * (elf_set.iter().map(|x| x.1).max().unwrap() - elf_set.iter().map(|x| x.1).min().unwrap()
            + 1)
        - elf_set.len() as i32;
    println!("Part 1: {}", empty_spaces);

    let mut elf_set = parse_input(&contents);
    //let mut elf_states = elf_set.iter().map(|elf| (elf.to_owned(), 0)).collect::<HashMap<_, _>>();
    let mut elf_state = 0;
    let mut looping = true;
    let mut loop_count = 0;
    while looping {
        looping = simulate_round(&mut elf_set, &mut elf_state);
        loop_count += 1;
    }

    println!("Part 2: {}", loop_count);
}

fn parse_input(contents: &str) -> HashSet<(i32, i32)> {
    let mut elf_set: HashSet<(i32, i32)> = HashSet::new();
    for (r_idx, line) in contents.lines().enumerate() {
        //println!("{}", r_idx);
        for (c_idx, c) in line.chars().enumerate() {
            if c == '#' {
                elf_set.insert((r_idx as i32, c_idx as i32));
            }
        }
    }

    elf_set
}

fn simulate_round(elf_set: &mut HashSet<(i32, i32)>, elf_state: &mut usize) -> bool {
    let mut next_proposed: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut next_proposed_actual: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    'outer: for elf in elf_set.iter() {
        //println!("{:?}", elf);
        // north ne nw, s, se, sw, etc.
        let directions = vec![
            (elf.0 - 1, elf.1),
            (elf.0 - 1, elf.1 + 1),
            (elf.0 - 1, elf.1 - 1),
            (elf.0 + 1, elf.1),
            (elf.0 + 1, elf.1 - 1),
            (elf.0 + 1, elf.1 + 1),
            (elf.0, elf.1 - 1),
            (elf.0 - 1, elf.1 - 1),
            (elf.0 + 1, elf.1 - 1),
            (elf.0, elf.1 + 1),
            (elf.0 + 1, elf.1 + 1),
            (elf.0 - 1, elf.1 + 1),
        ];
        let move_bool = !directions
            .iter()
            .fold(true, |acc, item| acc && !elf_set.contains(&item));
        if !move_bool {
            continue 'outer;
        };
        //let elf_state = elf_states.get_mut(elf).unwrap();
        let mut round_es = elf_state.to_owned();

        for _ in 0..4 {
            //println!("{}", round_es);
            let choice = &directions[(3 * round_es)..(3 * round_es) + 3];
            let move_bool = !choice
                .iter()
                .fold(true, |acc, item| acc && !elf_set.contains(&item));
            if !move_bool {
                //println!("{:?} choice {:?}, mb {:?}", elf, choice[0], move_bool);
                next_proposed
                    .entry(choice[0])
                    .or_insert(Vec::new())
                    .push(elf.to_owned());
                break;
            }
            round_es += 1;
            round_es %= 4;
        }

        //println!("{:?}", first_choices);
    }

    next_proposed_actual = next_proposed
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(s, v)| (*s, v[0]))
        .collect::<HashMap<(i32, i32), (i32, i32)>>();

    if next_proposed_actual.len() == 0 {
        return false;
    }

    for (new, old) in next_proposed_actual.iter() {
        //println!("{}", elf_set.len());
        elf_set.remove(old);
    }

    for (new, old) in next_proposed_actual.iter() {
        elf_set.insert(*new);
        //let state = elf_states.remove(&old).unwrap();
        //println!("{:?}, {:?}", old, new);
        //elf_states.insert(*new, (state+1) % 4);
    }

    *elf_state += 1;
    *elf_state %= 4;

    //println!("{:?}", elf_states);
    true
}

fn print_grid(elf_set: &HashSet<(i32, i32)>) {
    let min_elf_col = elf_set.iter().map(|x| x.1).min().unwrap();
    let min_elf_row = elf_set.iter().map(|x| x.0).min().unwrap();
    let max_elf_col = elf_set.iter().map(|x| x.1).max().unwrap();
    let max_elf_row = elf_set.iter().map(|x| x.1).max().unwrap();

    let mut str_vec = String::new();

    for r_idx in min_elf_row..=max_elf_row {
        let mut row_vec = String::new();
        for c_idx in min_elf_col..=max_elf_col {
            if elf_set.contains(&(r_idx, c_idx)) {
                row_vec.push_str("#");
            } else {
                row_vec.push_str(".");
            }
        }
        row_vec.push_str("\n");
        str_vec.push_str(&row_vec);
    }

    println!("{}", str_vec);
}
