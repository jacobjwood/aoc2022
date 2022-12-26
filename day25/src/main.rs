use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total : i64 = 0;
    for line in contents.lines() {
        total += parse_snafu(&line);
    }

    println!("Decimal total: {}", total);

    println!("{}", recode_snafu(&mut total));

}

fn parse_snafu(line: &str) -> i64 {

    let mut snafu : i64 = 0;
    let multiples = HashMap::from([('-', -1), ('=', -2), ('1', 1), ('2', 2), ('0', 0)]);

    for (power, x) in line.chars().rev().enumerate() {
        let multiple = multiples.get(&x).unwrap();
        snafu += (5 as i64).pow(power.try_into().unwrap()) * multiple;
    }

    snafu
}

fn recode_snafu(decimal: &i64) -> String {

    let mut dec = decimal.to_owned();
    let str_dec = format!("{}", decimal);

    let lookups = HashMap::from([(-1, '-'), (-2, '='), (0, '0'), (1, '1'), (2, '2')]);

    
    let mut snafu = String::new();

    let mut snafu_vec : Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut power : usize = 0;

    loop {
        let pow_five = 5_i64.pow(power.try_into().unwrap());
        let next_pow_five = 5_i64.pow((power+1).try_into().unwrap());
        if decimal / pow_five == 0 {break;}

        // 29 is 25 + 4
        // next pow five is 5, current power is 0,
        // 
        let into = (dec % (next_pow_five)) / pow_five;


        println!("into {}", into);

        if into / pow_five >= 3 {
            // 4 = 5 -1
            // add 1 to the right, add -1 to the left
            snafu_vec[power+1] += 1;
            snafu_vec[power] += into - 5;
        } else {
            snafu_vec[power] += into;
        }
        dec -= into * pow_five;


        power += 1;
    }

    println!("{:?}", snafu_vec);

    for j in 0..snafu_vec.len() {
        let snfu = snafu_vec[j];
        if snfu >= 3 {
            snafu_vec[j + 1] += 1;
            snafu_vec[j] = snfu - 5;
        }
    }

    println!("{:?}", snafu_vec);

    for k in snafu_vec.iter().rev() {
        snafu.push(*lookups.get(k).unwrap());
    }
    //println!("{}", j);
    //
    snafu

}
