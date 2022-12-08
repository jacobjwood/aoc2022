use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let buffer_stream = lines.collect::<String>();

    solution(&buffer_stream);
}

fn solution(buffer_stream: &str) {
    let mut start_message = false;

    for (rightpointer, _) in buffer_stream.chars().enumerate() {
        let leftpointer_1 = match rightpointer {
            0..=3 => 0,
            _ => rightpointer - 3,
        };
        let leftpointer_2 = match rightpointer {
            0..=13 => 0,
            _ => rightpointer - 13,
        };

        if start_message {
            let hs_message =
                HashSet::<char>::from_iter(buffer_stream[leftpointer_2..=rightpointer].chars());

            if hs_message.len() == 14 {
                println!("Part 2: {}", rightpointer + 1);
                break;
            }
        }

        let hs_begin =
            HashSet::<char>::from_iter(buffer_stream[leftpointer_1..=rightpointer].chars());

        if hs_begin.len() == 4 && !start_message {
            println!("Part 1: {}", rightpointer + 1);
            start_message = true;
        }
    }
}
