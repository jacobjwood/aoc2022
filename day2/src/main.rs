use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents = contents.trim();
    let contents_split = contents.split("\n");

    let mut round_scores_pt1 = 0;
    let mut round_scores_pt2 = 0;

    for round in contents_split {
        let round_string = String::from(round);

        let round_score = game_round_pt1(&round_string);
        round_scores_pt1 += round_score;

        let round_score_actual = game_round_pt2(&round_string);
        round_scores_pt2 += round_score_actual;
    }

    println!("Part 1: {}", round_scores_pt1);
    println!("Part 2: {}", round_scores_pt2);
}

fn game_round_pt1(round_string: &String) -> i32 {
    let game_values = HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);

    let hand_values = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let our_move = &round_string[2..];

    game_values.get(&round_string as &str).unwrap() + hand_values.get(&our_move as &str).unwrap()
}

fn game_round_pt2(round_string: &String) -> i32 {
    let opponent = &round_string[0..1];
    let outcome = &round_string[2..];

    let win = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let draw = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let lose = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);

    let pt1 = match outcome {
        "X" => lose.get(&opponent as &str).unwrap(),
        "Y" => draw.get(&opponent as &str).unwrap(),
        "Z" => win.get(&opponent as &str).unwrap(),
        _ => "",
    };

    let pt1 = String::from(pt1);

    let pt1_string = opponent.to_owned() + " " + &pt1;

    game_round_pt1(&pt1_string)
}
