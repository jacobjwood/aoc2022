use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut signal_vec = parse_input(&contents);

    //println!("{:?}", signal_vec);

    //println!("{:#?}", signal_vec);
    //mix_signal(signal_vec);
    //mix_signal(vec![-21, 3, 0, -3, -8, 2, 4, 40, 20, -10, -20]);

    let mut vec = parse_input(&contents);
    vec = vec.into_iter().map(|v| v * 811589153).collect();

    let mut original_vec: Vec<i64> = vec.to_owned();
    println!("OV {:?}", vec);
    println!(
        "{:?}",
        vec.iter()
            .map(|x| *x % vec.len() as i64)
            .collect::<Vec<i64>>()
    );
    let mut count = 0;
    let mut vec_numbers: HashSet<&i64> = HashSet::from_iter(vec.iter());
    println!("VL {}", vec_numbers.len());

    let mut configs = vec_numbers
        .iter()
        .map(|num| {
            (
                **num,
                vec.iter()
                    .enumerate()
                    .filter(|(_, e)| **e == **num)
                    .map(|(idx, _)| idx)
                    .collect::<VecDeque<usize>>(),
            )
        })
        .collect::<HashMap<i64, VecDeque<usize>>>();

    for i in 1..=10 {
        (vec, configs) = mix_signal_2(vec, &original_vec, configs);
        //println!("{:?}", original_vec);
        //println!("{:?}", vec[0..7].to_owned());
        //println!("{:?}", configs.get(&original_vec[0]).unwrap());
        if i == 1 {
            //assert_eq!(vec![0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153], vec);
            //assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], vec);
        }
        if i == 2 {
            //assert_eq!(vec![0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153], vec);
        }
        if i == 3 {
            println!("{}", i);
            //assert_eq!(vec![0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459], vec);
        }
        //println!("AFTER {} ROUND OF MIXING {:?}", i, vec);
        println!("\n\n");
        //println!("OV {:?}", original_vec);
    }

    let zero_index = vec
        .iter()
        .enumerate()
        .filter(|(_, e)| *e == &0)
        .map(|(idx, _)| idx)
        .next()
        .unwrap();
    println!("zi {}", zero_index);
    println!("1000th Number: {}", vec[(zero_index + 1000) % vec.len()]);
    println!("2000th Number: {}", vec[(zero_index + 2000) % vec.len()]);
    println!("3000th Number: {}", vec[(zero_index + 3000) % vec.len()]);
    let sum = vec[(zero_index + 1000) % vec.len()]
        + vec[(zero_index + 2000) % vec.len()]
        + vec[(zero_index + 3000) % vec.len()];
    println!("SUM : {}", sum);
}

fn parse_input(contents: &String) -> Vec<i64> {
    let mut signal_vec = Vec::new();

    for line in contents.lines() {
        signal_vec.push(line.parse::<i64>().unwrap());
    }

    signal_vec
}

fn mix_signal(mut vec: Vec<i64>) {
    let mut vec_numbers: HashSet<&i64> = HashSet::from_iter(vec.iter());
    let mut visited: HashSet<usize> = HashSet::new();
    let mut original_vec: Vec<i64> = vec.to_owned().into_iter().rev().collect();
    println!("{:?}", vec);
    let mut count = 0;
    println!("VL {}", vec_numbers.len());

    let mut configs = vec_numbers
        .iter()
        .map(|num| {
            (
                **num,
                vec.iter()
                    .enumerate()
                    .filter(|(idx, e)| **e == **num && !visited.contains(idx))
                    .map(|(idx, _)| idx)
                    .rev()
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<HashMap<i64, Vec<usize>>>();
    println!("{:?}", configs);

    while let Some(next_element) = original_vec.pop() {
        //println!("VL {:?}", visited.len());
        if false && count % 10 == 0 && count > 0 {
            std::thread::sleep_ms(10000);
        }
        count += 1;

        println!("REMAINING {}", original_vec.len());
        let mut ptr = configs.get_mut(&next_element).unwrap().pop().unwrap(); //vec.iter().enumerate().filter(|(idx, e)| **e == next_element && !visited.contains(idx)).map(|(idx, _)| idx).next().unwrap();
        println!("POINTER AT {}", ptr);
        /*
        if visited.contains(&ptr) {
            ptr += 1;
            ptr %= vec.len();
            continue;
        }
        */

        let element = vec[ptr];

        if element < 0 {
            vec = vec.into_iter().rev().collect();
            ptr = vec.len() - 1 - ptr;
        }

        let abs_element = element.abs() as usize;

        vec.remove(ptr);
        let mut insertion_position = ((ptr + abs_element) % vec.len()); // ((element > 0) &&  as usize) +

        // println!("VEC {:?}", vec);

        // println!("PTR {}", ptr);
        // println!("IP {}", insertion_position);

        vec.insert(insertion_position, element);

        if element < 0 {
            vec = vec.into_iter().rev().collect();
            ptr = vec.len() - 1 - ptr;
            insertion_position = vec.len() - 1 - insertion_position;
        }

        let (ipl, ipr) = if insertion_position == vec.len() - 1 {
            (insertion_position - 1, 0)
        } else if insertion_position == 0 {
            (vec.len() - 1, insertion_position + 1)
        } else {
            (insertion_position - 1, insertion_position + 1)
        };
        println!("{} moves between {} and {}", element, vec[ipl], vec[ipr]);

        let ptr_prev = ptr;

        if insertion_position <= ptr {
            ptr += 1;
            ptr %= vec.len();
        }

        //println!("VEC AFTER {:?}", vec);

        // visited = visited.into_iter().map(|x| (x + (insertion_position < x) as usize - (x >= ptr_prev) as usize) % vec.len()).collect();
        configs = configs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|x| {
                            (x + (insertion_position < x) as usize - (x >= ptr_prev) as usize)
                                % vec.len()
                        })
                        .collect(),
                )
            })
            .collect();
        //visited.insert(insertion_position);
        //println!("INSERTED {:?}", insertion_position);
        //println!("VISITED {:?}", visited);
    }

    let zero_index = vec
        .iter()
        .enumerate()
        .filter(|(_, e)| *e == &0)
        .map(|(idx, _)| idx)
        .next()
        .unwrap();
    println!("zi {}", zero_index);
    println!("1000th Number: {}", vec[(zero_index + 1000) % vec.len()]);
    println!("2000th Number: {}", vec[(zero_index + 2000) % vec.len()]);
    println!("3000th Number: {}", vec[(zero_index + 3000) % vec.len()]);
    let sum = vec[(zero_index + 1000) % vec.len()]
        + vec[(zero_index + 2000) % vec.len()]
        + vec[(zero_index + 3000) % vec.len()];
    println!("SUM : {}", sum);
}

fn mix_signal_2(
    mut vec: Vec<i64>,
    original_vec: &Vec<i64>,
    mut configs: HashMap<i64, VecDeque<usize>>,
) -> (Vec<i64>, HashMap<i64, VecDeque<usize>>) {
    let mut vec_numbers: HashSet<&i64> = HashSet::from_iter(vec.iter());
    let mut visited: HashSet<usize> = HashSet::new();
    //println!("{:?}", configs);
    //let mut configs = vec_numbers.iter().map(|num| (**num, vec.iter().enumerate().filter(|(_, e)| **e == **num).map(|(idx, _)| idx).collect::<VecDeque<usize>>())).collect::<HashMap<i64, VecDeque<usize>>>();
    println!("CFGET {:?}", configs.get(&(5061 * 811589153)).unwrap());
    // need [0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459]
    // [2434767459, 3246356612, 1623178306, -1623178306, -2434767459, 0, 811589153]
    //  [2434767459, 3246356612, 1623178306, -1623178306, -2434767459, 0, 811589153]
    for next_element in original_vec {
        //println!("VL {:?}", visited.len());

        //println!("REMAINING {}", original_vec.len());
        //println!("LENGTH OF VEC BEFORE POPPING {}", configs.get(&next_element).unwrap().len());
        // println!("{:?}", configs);
        let mut ptr = configs.get_mut(&next_element).unwrap().pop_front().unwrap(); //vec.iter().enumerate().filter(|(idx, e)| **e == next_element && !visited.contains(idx)).map(|(idx, _)| idx).next().unwrap();
                                                                                    //println!("PTR {}", ptr);
                                                                                    //println!("LENGTH OF VEC AFTER POPPING {}", configs.get(&next_element).unwrap().len());
                                                                                    //println!("POINTER AT {}", ptr);
                                                                                    /*
                                                                                    if visited.contains(&ptr) {
                                                                                        ptr += 1;
                                                                                        ptr %= vec.len();
                                                                                        continue;
                                                                                    }
                                                                                    */

        let element = vec[ptr];
        let ptr_prev = ptr;

        if element < 0 {
            vec = vec.into_iter().rev().collect();
            ptr = vec.len() - 1 - ptr;
        }

        let abs_element = element.abs() as usize;

        vec.remove(ptr);

        // update the mappings
        configs = configs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter().map(|x| x - (x > ptr_prev) as usize).collect(),
                )
            })
            .collect();

        let mut insertion_position = ((ptr + abs_element) % vec.len()); // ((element > 0) &&  as usize) +

        // println!("VEC {:?}", vec);

        // println!("PTR {}", ptr);
        // println!("IP {}", insertion_position);

        vec.insert(insertion_position, element);

        if element < 0 {
            vec = vec.into_iter().rev().collect();
            ptr = vec.len() - 1 - ptr;
            insertion_position = vec.len() - 1 - insertion_position;
        }

        let (ipl, ipr) = if insertion_position == vec.len() - 1 {
            (insertion_position - 1, 0)
        } else if insertion_position == 0 {
            (vec.len() - 1, insertion_position + 1)
        } else {
            (insertion_position - 1, insertion_position + 1)
        };

        let ptr_prev = ptr;

        if insertion_position <= ptr {
            ptr += 1;
            ptr %= vec.len();
        }

        //println!("{} moves between {} and {}", element, vec[ipl], vec[ipr]);
        //println!("VEC AFTER {:?}", vec);

        // visited = visited.into_iter().map(|x| (x + (insertion_position < x) as usize - (x >= ptr_prev) as usize) % vec.len()).collect();
        configs = configs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|x| {
                            //
                            if false && insertion_position == x {
                                println!("X {} k {} PTR PREV {}", x, k, ptr_prev);
                                println!("IP {} NE {}", insertion_position, next_element);
                                println!("X {}", x >= ptr_prev);
                                println!("X {}", insertion_position < x);
                            }
                            // ptr prev < x and ip >

                            //(x + (insertion_position <= x) as usize - (x >= ptr_prev) as usize)
                            (x + (insertion_position <= x) as usize)
                            //
                            //(x  + ((insertion_position < x || (insertion_position <= x && ptr_prev == vec.len()-1))) as usize - (x >= ptr_prev) as usize) % vec.len()
                        })
                        .collect(),
                )
            })
            .collect();
        configs
            .get_mut(next_element)
            .unwrap()
            .push_back(insertion_position);

        //cfgs = cfgs.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| (x + (insertion_position < x) as usize - (x >= ptr_prev) as usize) % vec.len()).collect())).collect();
        //visited.insert(insertion_position);
        //println!("INSERTED {:?}", insertion_position);
        //println!("VISITED {:?}", visited);

        // if positive

        /*
        // remove element from vec and moves all remaining elements left
        //println!("PTR {} ELEMENT {}", ptr, vec[ptr]);
        let mut new_idx = match vec[ptr] < 0 {
            true => ((vec.len() as i64 + ((ptr as i64 + vec[ptr]) % vec.len() as i64)) % vec.len() as i64) as usize,
            false => ((ptr as i64 + vec[ptr]) % vec.len() as i64) as usize,
        };
        //println!("NEW IDX {}", new_idx);
        let element = vec.remove(ptr);

        if vec[ptr] > 0 {
            vec.insert(new_idx + 1, vec[ptr]);
        } else {
            vec.insert(new_idx, vec[ptr]);
        }


        if ptr >= new_idx {
            ptr += 1;
            //ptr %= vec.len() - 1;
        }

        println!("{} moves between {} and {}", element, vec[new_idx-1], vec[new_idx+1]);
        //println!("Pointer removed at {} so new_idx is now {}", ptr, new_idx);
        println!("{:?}", vec);
        let vl_before = visited.len();
        println!("{:?}", visited);
        visited = visited.into_iter().map(|x| x + (x >= new_idx) as usize).collect();
        visited = visited.into_iter().map(|x| x - (x >= ptr) as usize).collect();
        assert!(vl_before == visited.len());
        visited.insert(new_idx);

        ptr = (ptr + ((ptr > new_idx) as usize)) % vec.len();
        */
    }

    (vec, configs)
}
