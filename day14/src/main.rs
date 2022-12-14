use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "input_test.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (positions, x_domain, y_domain) = parse_input(contents);

    create_caves(&positions, &x_domain, &y_domain);

}


fn parse_input(contents: String) -> (Vec<Vec<(usize, usize)>>, (usize, usize), (usize, usize)) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = usize::MAX;
    let mut min_y = 0;
    let mut positions : Vec<Vec<(usize, usize)>> = Vec::new();

    for path in contents.lines() {

        let mut path_vec : Vec<(usize, usize)> = Vec::new();
        
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

fn add_line_indices<'a>(coverage: &mut HashSet<(usize, usize)>, pos: &(usize, usize), next: &'a (usize, usize)) -> &'a (usize, usize) {

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

fn create_caves(positions: &Vec<Vec<(usize, usize)>>, x_domain: &(usize, usize), y_domain: &(usize, usize)) {
    
    let mut cave : Vec<Vec<char>> = Vec::new();
    let mut coverage : HashSet<(usize, usize)> = HashSet::new();

    for path in positions {
        path.iter().reduce(|pos, next| add_line_indices(&mut coverage, pos, next));
    }

    println!("{:?}", coverage);

    for y in y_domain.0..=y_domain.1 {
        let mut cave_row : Vec<char> = Vec::new();

        for x in x_domain.0..=x_domain.1 {
            
            if coverage.contains(&(x, y)) {
                
                cave_row.push('#');
            } else {
                cave_row.push('.');
            }
        }

        cave.push(cave_row);
    }

    let cave_printable : Vec<String> = cave.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>();

    println!("{:#?}", cave_printable)

}