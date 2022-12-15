use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    /*
    let input_vec = contents
        .split(":")
        .map(|s| s.split(",")
                    .map(|s| s.chars().filter(|c| *c == '-' || c.is_numeric()).collect::<String>().trim().parse::<i32>().unwrap()
                    ).collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();
    */
    let sb_vec = parse_input(&contents);

    //println!("{:?}", sb_vec);

    let hm_out = get_md_info(&sb_vec);

    // println!("{:?}", hm_out);
    let line = 2000000;
    let no_beacon = get_empty_beacon(&hm_out, line);
    let beacon_in_line_count = sb_vec.iter().filter(|v| v[1].1 == line).map(|v| v[1]).collect::<HashSet<_>>().len();
    println!("{:?}", beacon_in_line_count);
    println!("Pt 1: {}", no_beacon - beacon_in_line_count);


}

fn parse_input(contents: &String) -> Vec<Vec<(i32, i32)>> {

    let mut sb_vec : Vec<Vec<(i32, i32)>> = Vec::new();

    for line in contents.lines() {
        let mut row_vec : Vec<(i32, i32)> = Vec::new();

        for sb in line.split(":") {
            println!("{}", sb);

            let s_or_b = sb.split(",").map(|s| s.chars().filter(|c| c.is_numeric() || *c == '-').collect::<String>().parse::<i32>().unwrap()).collect::<Vec<i32>>();

            row_vec.push((s_or_b[0], s_or_b[1]));
        }
        sb_vec.push(row_vec);
    }

    sb_vec
}

fn get_md_info(sb_vec: &Vec<Vec<(i32, i32)>>) -> HashMap<(i32, i32), i32> {

    let mut empty_states : HashMap<(i32, i32), i32> = HashMap::new();

    for sb in sb_vec {

        // println!("{:?}", sb);
        let s = sb[0];
        let b = sb[1];
        // println!("{:?}", s);

        let x_span = (s.0 - b.0).abs();
        let y_span = (s.1 - b.1).abs();
        let md = x_span + y_span;

        empty_states.insert(s, md);

        
    }

    empty_states
}

fn get_empty_beacon(md_info: &HashMap<(i32, i32), i32>, line: i32) -> usize {
    let mut x_domains : Vec<(i32, i32)> = Vec::new();

    for (source, md) in md_info {
        // println!("{:?}", source);
        let y_max = source.1 + md;
        let y_min = source.1 - md;
        let y_domain = y_min..=y_max;
        let dist_from_source = (source.1 - line).abs();
        // println!("{}", dist_from_source);
        if y_domain.contains(&line) {
            let x_min = source.0 - (md - dist_from_source);
            let x_max = source.0 + (md - dist_from_source);
            x_domains.push((x_min, x_max));

            // println!("{:?} {} {}", source, x_min, x_max);
        }
    }

    // println!("{:?}", x_domains);

    let mut x_set : HashSet<i32> = HashSet::new();

    for x in x_domains {
        for i in x.0..=x.1 {
            x_set.insert(i);
        }
    }

    let mut x_set = x_set.iter().collect::<Vec<_>>();
    x_set.sort();
    // println!("{:?}", x_set);
    x_set.len() 
}