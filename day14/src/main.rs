use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let print_caves = false;

    if print_caves {
        print!("{esc}c", esc = 27 as char);
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (positions, x_domain, y_domain) = parse_input(&contents);

    let (mut caves, _) = create_caves(&positions, &x_domain, &y_domain);

    let mut count_pt1 = 0;

    let visual = match file_path {
        "input_test.txt" => true,
        _ => false,
    };

    while !pour_sand_visual(&mut caves, &x_domain, &y_domain, visual) {
        if caves[0][500 - x_domain.0] == 'o' {
            break;
        }
        count_pt1 += 1;
    }

    println!("Part 1: {}", count_pt1);

    let (positions, x_domain, y_domain) = parse_input(&contents);

    let (_, mut cave_map) = create_caves(&positions, &x_domain, &y_domain);

    let mut pt2_count = 0;

    while !pour_sand_floor(&mut cave_map, &y_domain) {
        pt2_count += 1;
    }

    println!("Part 2: {}", pt2_count);
}

fn parse_input(contents: &String) -> (Vec<Vec<(usize, usize)>>, (usize, usize), (usize, usize)) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = usize::MAX;
    let mut min_y = 0;
    let mut positions: Vec<Vec<(usize, usize)>> = Vec::new();

    for path in contents.lines() {
        let mut path_vec: Vec<(usize, usize)> = Vec::new();

        for segment in path.trim().split("->") {
            let push = segment
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let x = push[0];
            let y = push[1];

            max_x = std::cmp::max(max_x, x);
            max_y = std::cmp::max(max_y, y);
            min_x = std::cmp::min(min_x, x);
            min_y = std::cmp::min(min_y, y);

            path_vec.push((x, y));
        }

        positions.push(path_vec);
    }

    (positions, (min_x, max_x), (min_y, max_y))
}

fn add_line_indices<'a>(
    coverage: &mut HashSet<(usize, usize)>,
    pos: &(usize, usize),
    next: &'a (usize, usize),
) -> &'a (usize, usize) {
    let (pos_x, pos_y) = pos;
    let (next_x, next_y) = next;

    let max_x = std::cmp::max(pos_x, next_x);
    let min_x = std::cmp::min(pos_x, next_x);
    let max_y = std::cmp::max(pos_y, next_y);
    let min_y = std::cmp::min(pos_y, next_y);

    for x in *min_x..=*max_x {
        for y in *min_y..=*max_y {
            coverage.insert((x, y));
        }
    }

    next
}

fn cave_print(cave: &Vec<Vec<char>>) {
    print!("{esc}c", esc = 27 as char);
    let cave_printable: Vec<String> = cave
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>();
    println!("{:#?}", cave_printable);
    std::thread::sleep(std::time::Duration::from_millis(30));
}

fn create_caves(
    positions: &Vec<Vec<(usize, usize)>>,
    x_domain: &(usize, usize),
    y_domain: &(usize, usize),
) -> (Vec<Vec<char>>, HashSet<(usize, usize)>) {
    let mut cave: Vec<Vec<char>> = Vec::new();
    let mut coverage: HashSet<(usize, usize)> = HashSet::new();

    for path in positions {
        path.iter()
            .reduce(|pos, next| add_line_indices(&mut coverage, pos, next));
    }

    for y in y_domain.0..=y_domain.1 {
        let mut cave_row: Vec<char> = Vec::new();

        for x in x_domain.0..=x_domain.1 {
            if coverage.contains(&(x, y)) {
                cave_row.push('#');
            } else {
                cave_row.push('.');
            }
        }

        cave.push(cave_row);
    }

    (cave, coverage)
}

fn is_abyss(x: &usize, y: &usize, x_domain: &(usize, usize), y_domain: &(usize, usize)) -> bool {
    if *x == x_domain.0 - 1 {
        true
    } else {
        !(x_domain.0..=x_domain.1).contains(&(x - x_domain.0))
            && !(y_domain.0..=y_domain.1).contains(y)
    }
}

fn pour_sand_visual(
    caves: &mut Vec<Vec<char>>,
    x_domain: &(usize, usize),
    y_domain: &(usize, usize),
    visual: bool,
) -> bool {
    let (mut sand_x, mut sand_y) = (500, 0);

    loop {
        let (below_x, below_y) = (sand_x, sand_y + 1);
        let (below_left_x, below_left_y) = (sand_x - 1, sand_y + 1);
        let (below_right_x, below_right_y) = (sand_x + 1, sand_y + 1);

        caves[sand_y][sand_x - x_domain.0] = '+';

        if visual {
            cave_print(&caves)
        };

        if !is_abyss(&below_x, &below_y, x_domain, y_domain) {
            let below = caves[below_y][below_x - x_domain.0];

            if below == '.' {
                caves[sand_y][sand_x - x_domain.0] = '.';
                sand_y += 1;
                continue;
            } else {
                if !is_abyss(&below_left_x, &below_left_y, x_domain, y_domain) {
                    let below_left = caves[below_left_y][below_left_x - x_domain.0];
                    if below_left == '.' {
                        caves[sand_y][sand_x - x_domain.0] = '.';
                        sand_x = below_left_x;
                        sand_y = below_left_y;
                        continue;
                    } else if below_left == '#' || below_left == 'o' {
                        if !is_abyss(&below_right_x, &below_right_y, x_domain, y_domain) {
                            let below_right = caves[below_right_y][below_right_x - x_domain.0];
                            if below_right == '.' {
                                caves[sand_y][sand_x - x_domain.0] = '.';
                                sand_x = below_right_x;
                                sand_y = below_right_y;
                                continue;
                            } else {
                                caves[sand_y][sand_x - x_domain.0] = 'o';
                                return false;
                            }
                        }
                    } else {
                        if !is_abyss(&below_right_x, &below_right_y, x_domain, y_domain) {
                            let below_right = caves[below_right_y][below_right_x - x_domain.0];
                            if below_right == '.' {
                                caves[sand_y][sand_x - x_domain.0] = '.';
                                sand_x = below_right_x;
                                sand_y = below_right_y;
                                continue;
                            }
                        } else {
                            caves[sand_y][sand_x - x_domain.0] = 'o';
                            if visual {
                                cave_print(&caves)
                            };
                            return true;
                        }
                    }
                } else {
                    caves[sand_y][sand_x - x_domain.0] = '.';
                    if visual {
                        cave_print(&caves)
                    };
                    return true;
                }
            }
        } else {
            return true;
        }
    }
}

fn pour_sand_floor(cave_map: &mut HashSet<(usize, usize)>, y_domain: &(usize, usize)) -> bool {
    // might need to go to isize because the x might become negative

    if cave_map.contains(&(500, 0)) {
        return true;
    }

    let (mut sand_x, mut sand_y) = (500, 0);
    let (_, mut max_y) = *y_domain;
    max_y += 2;

    loop {
        let (br_x, br_y) = (sand_x + 1, sand_y + 1);
        let (bl_x, bl_y) = (sand_x - 1, sand_y + 1);
        let (b_x, b_y) = (sand_x, sand_y + 1);
        // if floor directly below
        if b_y == max_y {
            cave_map.insert((sand_x, sand_y));
            return false;
        } else if cave_map.contains(&(b_x, b_y))
            && cave_map.contains(&(bl_x, bl_y))
            && cave_map.contains(&(br_x, br_y))
        {
            cave_map.insert((sand_x, sand_y));
            return false;
        } else if !cave_map.contains(&(b_x, b_y)) {
            sand_y += 1;
            continue;
        } else if cave_map.contains(&(b_x, b_y)) && !cave_map.contains(&(bl_x, bl_y)) {
            sand_y = bl_y;
            sand_x = bl_x;
            continue;
        } else if cave_map.contains(&(b_x, b_y))
            && cave_map.contains(&(bl_x, bl_y))
            && !cave_map.contains(&(br_x, br_y))
        {
            sand_y = br_y;
            sand_x = br_x;
            continue;
        }
    }
}
