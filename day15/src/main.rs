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
    println!("Part 1: {}", no_beacon - beacon_in_line_count);

    let xy_max = 4000000;
    let mut row = 0;
    let mut col = 0;
    
    for i in 0..4000000 {
        if get_full_empty_beacon(&hm_out, i, 4000000) {
            row = i;
            break;
        }
    }

    println!("ROW {}", row);

    let x_set = pt2(&hm_out, row, 4000000);

    println!("x_set len {}", x_set.len());

    for j in 0..=4000000 {
        if !x_set.contains(&j) {
            col = j;
        }
    }
    
    println!("Part 2: {}", (col as u64)*4000000 + (row as u64));



}

fn parse_input(contents: &String) -> Vec<Vec<(i32, i32)>> {

    let mut sb_vec : Vec<Vec<(i32, i32)>> = Vec::new();

    for line in contents.lines() {
        let mut row_vec : Vec<(i32, i32)> = Vec::new();

        for sb in line.split(":") {
            //println!("{}", sb);

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

    //let mut x_set = x_set.iter().collect::<Vec<_>>();
    // x_set.sort();
    // println!("{:?}", x_set);
    x_set.len() 
}

fn pt2(md_info: &HashMap<(i32, i32), i32>, line: i32, max_no: i32) -> HashSet<i32> {
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
            //x_domains.push((x_min, x_max));
            x_domains.push((std::cmp::max(std::cmp::min(max_no, x_min), 0), std::cmp::min(max_no, x_max)));

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

    //let mut x_set = x_set.iter().collect::<Vec<_>>();
    // x_set.sort();
    // println!("{:?}", x_set);
    x_set
}

fn get_full_empty_beacon(md_info: &HashMap<(i32, i32), i32>, line: i32, max_no: i32) -> bool {
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
            x_domains.push((std::cmp::max(std::cmp::min(max_no, x_min), 0), std::cmp::min(max_no, x_max)));

            // println!("{:?} {} {}", source, x_min, x_max);
        }
    }

    //let domain_len = x_domains[1..].iter().fold((Vec::from([(0, 4000000)]), x_domains[0]), |(acc, prev_range), range| (overlap(&prev_range, range), (std::cmp::min(prev_range.0, range.0), std::cmp::max(prev_range.1, range.1))));
    //println!("DOMAIN LEN {}", domain_len.0);
    //println!("X DOMAIN LEN {}", x_domains.len());
    x_domains.sort_by(|a, b| a.0.cmp(&b.0));

    let min = x_domains[0].0;
    let mut max = x_domains[0].1;

    for idx in 1..x_domains.len() {
        
        let v0 = x_domains[idx].0;
        let v1 = x_domains[idx].1;

        //println!("MIN {} MAX {}", min, max);
        //println!("V0 {} V1 {}", v0, v1);
        if v0 <= max + 1 {
            max = std::cmp::max(max, v1);
        }
        else { return true };
    }

    false
    
    // println!("{:?}", x_domains);
    // let mut x_set : HashSet<i32> = HashSet::from_iter(x_domains);

    /*
    for x in x_domains {
        for i in x.0..=x.1 {
            x_set.insert(i);
        }
    }

    //let mut x_set = x_set.iter().collect::<Vec<_>>();
    // x_set.sort();
    // println!("{:?}", x_set);
    x_set.len()
    */
}

/*
x1------
x2    --------

x1       -------
x2 ----------

    x1----
x2 -------------

x1 -------------
      x2------

x1 ------
x2          ------
        -----
*/

fn overlap(x1: &(i32, i32), x2: &(i32, i32)) -> i32 {

    println!("{:?} {:?}", x1, x2);

    let overlap = if (x1.0..=x1.1).contains(&x2.0) && (x1.0..=x1.1).contains(&x2.1) {
            0 //1 + x2.1 - x2.0
        } else if (x2.0..=x2.1).contains(&x1.0) && (x2.0..=x2.1).contains(&x1.1) {
            0 //1 + x1.1 - x1.0
        } else if (x2.0..=x2.1).contains(&x1.0) && !(x2.0..=x2.1).contains(&x1.1) {
            0 //1 + x2.1 - x1.0
        } else if (x1.0..=x1.1).contains(&x2.0) && !(x1.0..=x1.1).contains(&x2.1) {
            0 //1 + x1.1 - x2.0 
        } else {
            println!("  LACK OF OVERLAP");
            let gap = std::cmp::max(x2.0, x1.0) - std::cmp::min(x1.1, x2.1) - 1;
            println!("  {}", gap);
            gap
        };

    // println!("  ol {}", overlap);

    //std::cmp


    0

}