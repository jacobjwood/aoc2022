use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let sb_vec = parse_input(&contents);

    let hm_out = get_md_info(&sb_vec);
    let line = 2000000;
    let no_beacon = get_empty_beacon(&hm_out, line);
    let beacon_in_line_count = sb_vec
        .iter()
        .filter(|v| v[1].1 == line)
        .map(|v| v[1])
        .collect::<HashSet<_>>()
        .len();
    println!("Part 1: {}", no_beacon - beacon_in_line_count);

    let xy_max = 4000000;
    let mut row = 0;
    let mut col = 0;

    for i in 0..xy_max {
        if check_for_gaps(&hm_out, i, xy_max) {
            row = i;
            break;
        }
    }

    let x_set = get_empty_beacon_pt2(&hm_out, row, xy_max);

    for j in 0..=xy_max {
        if !x_set.contains(&j) {
            col = j;
        }
    }

    println!("Part 2: {}", (col as u64) * 4000000 + (row as u64));
}

fn parse_input(contents: &String) -> Vec<Vec<(i32, i32)>> {
    let mut sb_vec: Vec<Vec<(i32, i32)>> = Vec::new();

    for line in contents.lines() {
        let mut row_vec: Vec<(i32, i32)> = Vec::new();

        for sb in line.split(":") {
            //println!("{}", sb);

            let s_or_b = sb
                .split(",")
                .map(|s| {
                    s.chars()
                        .filter(|c| c.is_numeric() || *c == '-')
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap()
                })
                .collect::<Vec<i32>>();

            row_vec.push((s_or_b[0], s_or_b[1]));
        }
        sb_vec.push(row_vec);
    }

    sb_vec
}

fn get_md_info(sb_vec: &Vec<Vec<(i32, i32)>>) -> HashMap<(i32, i32), i32> {
    let mut empty_states: HashMap<(i32, i32), i32> = HashMap::new();

    for sb in sb_vec {
        let s = sb[0];
        let b = sb[1];

        let x_span = (s.0 - b.0).abs();
        let y_span = (s.1 - b.1).abs();
        let md = x_span + y_span;

        empty_states.insert(s, md);
    }

    empty_states
}

fn get_empty_beacon(md_info: &HashMap<(i32, i32), i32>, line: i32) -> usize {
    let mut x_domains: Vec<(i32, i32)> = Vec::new();

    for (source, md) in md_info {
        let y_max = source.1 + md;
        let y_min = source.1 - md;
        let y_domain = y_min..=y_max;
        let dist_from_source = (source.1 - line).abs();
        if y_domain.contains(&line) {
            let x_min = source.0 - (md - dist_from_source);
            let x_max = source.0 + (md - dist_from_source);
            x_domains.push((x_min, x_max));
        }
    }

    let mut x_set: HashSet<i32> = HashSet::new();

    for x in x_domains {
        for i in x.0..=x.1 {
            x_set.insert(i);
        }
    }

    x_set.len()
}

fn get_empty_beacon_pt2(
    md_info: &HashMap<(i32, i32), i32>,
    line: i32,
    max_no: i32,
) -> HashSet<i32> {
    let mut x_domains: Vec<(i32, i32)> = Vec::new();

    for (source, md) in md_info {
        let y_max = source.1 + md;
        let y_min = source.1 - md;
        let y_domain = y_min..=y_max;
        let dist_from_source = (source.1 - line).abs();
        if y_domain.contains(&line) {
            let x_min = source.0 - (md - dist_from_source);
            let x_max = source.0 + (md - dist_from_source);
            x_domains.push((
                std::cmp::max(std::cmp::min(max_no, x_min), 0),
                std::cmp::min(max_no, x_max),
            ));
        }
    }

    let mut x_set: HashSet<i32> = HashSet::new();

    for x in x_domains {
        for i in x.0..=x.1 {
            x_set.insert(i);
        }
    }

    x_set
}

fn check_for_gaps(md_info: &HashMap<(i32, i32), i32>, line: i32, max_no: i32) -> bool {
    let mut x_domains: Vec<(i32, i32)> = Vec::new();

    for (source, md) in md_info {
        let y_max = source.1 + md;
        let y_min = source.1 - md;
        let y_domain = y_min..=y_max;
        let dist_from_source = (source.1 - line).abs();
        if y_domain.contains(&line) {
            let x_min = source.0 - (md - dist_from_source);
            let x_max = source.0 + (md - dist_from_source);
            x_domains.push((
                std::cmp::max(std::cmp::min(max_no, x_min), 0),
                std::cmp::min(max_no, x_max),
            ));
        }
    }

    x_domains.sort_by(|a, b| a.0.cmp(&b.0));

    let mut max = x_domains[0].1;

    for idx in 1..x_domains.len() {
        let v0 = x_domains[idx].0;
        let v1 = x_domains[idx].1;

        if v0 <= max + 1 {
            max = std::cmp::max(max, v1);
        } else {
            return true;
        };
    }

    false
}
