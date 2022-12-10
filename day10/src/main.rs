use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut x : i32 = 1;
    let mut cycle_count : i32 = 0;

    let mut cycle_tracker : HashMap<i32, i32> = HashMap::new();

    let mut final_string_vec = Vec::<&str>::new();

    for line in contents.lines() {

        if line == "noop" {

            update_string(&mut final_string_vec, &cycle_count, &x);

            cycle_count += 1;

            cycle_tracker.insert(cycle_count, x);
            
        } else {
            
            for string in line.split(" ") {

                update_string(&mut final_string_vec, &cycle_count, &x);

                cycle_count += 1;

                cycle_tracker.insert(cycle_count, x);

                match string.parse::<i32>() {
                    Ok(num) => x += num,
                    Err(_) => ()
                }
            }
        }


    }

    let pt1 : i32 = cycle_tracker
        .into_iter()
        .filter(|(k, _)| (k <= &220) & (k % 20 == 0))
        .filter(|(k, _)| ((k / 20) % 2) == 1)
        .map(|(k, v)| k*v)
        .sum::<i32>();

    println!("Part 1: {}", pt1);

    for i in 1..=6 {
        let line_string = final_string_vec[40*(i-1)..40*i]
            .iter()
            .map(|s| *s)
            .collect::<String>();

        println!("{:?}", line_string);
    }

}

fn update_string(final_string: &mut Vec<&str>, cycle: &i32, x: &i32) {
    
    let x_left = *x - 1 + (40 * (cycle / 40));
    let x_right = *x + 1 + (40 * (cycle / 40));

    if (x_left..=x_right).contains(cycle) {
        final_string.push(&"#");
    } else {
        final_string.push(&".");
    }
}