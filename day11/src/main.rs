use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
struct Monkey<'a> {
    items: VecDeque<u128>,
    items_repr: VecDeque<HashMap<u128, (u128, u128)>>,
    operation: &'a str,
    test_quotient: u128,
    throw_monkeys: [u32; 2],
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut monkey_map = init_monkeys(&contents);

    let mut divisors = monkey_map
        .iter()
        .map(|m| m.test_quotient)
        .collect::<Vec<u128>>();
    println!("Divisors {:?}", divisors);
    let mut monkey_item_count: Vec<u128> = Vec::new();

    for _ in 0..monkey_map.len() {
        monkey_item_count.push(0);
    }
    //println!("{:?}", monkey_item_count);

    println!("{:#?}", monkey_map);

    for _ in 0..20 {
        monkey_cycles1(&mut monkey_map, &mut monkey_item_count, true)
    }

    println!("\n MONKEY MUTATED \n");
    //println!("{:#?}", monkey_map);
    //println!("{:?}", monkey_item_count);
    //let mut monkey_map = init_monkeys(&contents);
    println!("{:?}", monkey_item_count);
    monkey_item_count.sort_by(|a, b| b.cmp(a));
    println!(
        "Part 1: {:?}",
        monkey_item_count[..2].iter().product::<u128>()
    );

    if true {
        let mut monkey_map = init_monkeys(&contents);
        //println!("{}", contents);
        let mut monkey_item_count: Vec<u128> = Vec::new();

        for _ in 0..monkey_map.len() {
            monkey_item_count.push(0);
        }

        for _ in 0..10000 {
            monkey_cycles1(&mut monkey_map, &mut monkey_item_count, false)
        }

        println!("\n MONKEY MUTATED \n");
        //println!("{:#?}", monkey_map);
        //println!("{:?}", monkey_item_count);
        //let mut monkey_map = init_monkeys(&contents);
        println!("{:?}", monkey_item_count);
        monkey_item_count.sort_by(|a, b| b.cmp(a));
        println!(
            "Part 2: {:?}",
            monkey_item_count[..2].iter().product::<u128>()
        );
    }

    // println!("\n MONKEY MUTATED \n");
    // println!("{:#?}", monkey_map);
    // println!("{:?}", monkey_item_count);
    //let mut monkey_map = init_monkeys(&contents);
    // monkey_item_count.sort_by(|a, b| b.cmp(a));
    // println!("Part 1: {:?}", monkey_item_count[..2].iter().product::<u128>());

    /*
    println!("{:?}", monkey_map);

    let mut current_monkey: u128;
    let mut current_test : &str;

    for line in contents.lines() {
        println!("{}", line);

        let current_monkey = match line.contains("Monkey") {
            true => {
                line.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
            }
            false => (),
        };




    }
    */
}

fn init_monkeys(contents: &str) -> Vec<Monkey> {
    let mut monkey_map: Vec<Monkey> = Vec::new();

    for monkey_info in contents.split("\n\n") {
        let mut items: VecDeque<u128> = VecDeque::new();
        let mut items_repr: VecDeque<HashMap<u128, (u128, u128)>> = VecDeque::new();
        let mut operation: &str = &"";
        let mut test_quotient: u128 = 1;
        let mut throw_monkeys: [u32; 2] = [0, 0];

        for line in monkey_info.split("\n") {
            let line_split = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>();
            let info = line_split[0];
            let content = line_split[1];

            match info {
                "Starting items" => {
                    items = content
                        .split(",")
                        .map(|s| s.trim().parse::<u128>().unwrap())
                        .collect::<VecDeque<u128>>();
                }
                "Operation" => {
                    operation = content.trim();
                }
                "Test" => {
                    test_quotient = content
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u128>()
                        .unwrap();
                }
                "If true" => {
                    throw_monkeys[0] = content
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                }
                "If false" => {
                    throw_monkeys[1] = content
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                }

                _ => (),
            }
        }

        let monkey = Monkey {
            items,
            items_repr,
            operation,
            test_quotient,
            throw_monkeys,
        };
        //println!("{:#?}", monkey);
        monkey_map.push(monkey);
    }

    //println!("{:#?}", monkey_map);
    let mut divisors = monkey_map
        .iter()
        .map(|m| m.test_quotient)
        .collect::<Vec<u128>>();
    for monkey_idx in 0..monkey_map.len() {
        let mut items_repr = monkey_map[monkey_idx]
            .items
            .iter()
            .map(|item| get_repr(&item, &divisors))
            .collect::<VecDeque<_>>();
        monkey_map[monkey_idx].items_repr = items_repr;
    }
    monkey_map
}

fn parse_operation(item: &mut u128, operation: &str, worry: bool) -> u128 {
    let op_vec = operation.split(" ").collect::<Vec<&str>>();
    // println!("{:?}", op_vec);

    let operand_1 = match op_vec[2] {
        "old" => *item,
        _ => op_vec[2].parse::<u128>().unwrap(),
    };

    let operand_2 = match op_vec[4] {
        "old" => *item,
        _ => op_vec[4].parse::<u128>().unwrap(),
    };

    let result = match op_vec[3] {
        "*" => operand_1 * operand_2,
        "/" => operand_1 / operand_2,
        "+" => operand_1 + operand_2,
        "-" => operand_1 - operand_2,
        _ => 0,
    };

    // println!("{} {} {} = {}", operand_1, op_vec[3], operand_2, result);

    match worry {
        true => result / 3,
        false => result,
    }
}

/*
fn monkey_cycles(monkey_map: &mut Vec<Monkey>, monkey_item_count: &mut Vec<u128>, worry: bool) {

    for monkey_idx in 0..monkey_map.len() {

        for item_idx in 0..monkey_map[monkey_idx].items.len() {

            monkey_item_count[monkey_idx] += 1;

            let mut item = monkey_map[monkey_idx].items[item_idx];
            let operation = monkey_map[monkey_idx].operation;
            item = parse_operation(&mut item, &operation, worry);

            let tq = monkey_map[monkey_idx].test_quotient;
            let tm0 = monkey_map[monkey_idx].throw_monkeys[0];
            let tm1 = monkey_map[monkey_idx].throw_monkeys[1];

            let mut non_overflow_item = 1;

            for div in divisors {
                match item % div {
                    0 => non_overflow_item *= div,
                    _ => (),
                }
            }

            if non_overflow_item == 1 { non_overflow_item = item }

            match non_overflow_item % tq {
                0 => monkey_map[tm0 as usize].items.push_back(non_overflow_item),
                _ => monkey_map[tm1 as usize].items.push_back(non_overflow_item),
            }

            // println!("{}", item);
        }
        monkey_map[monkey_idx].items = VecDeque::new();


    }
}
*/

fn monkey_cycles1(monkey_map: &mut Vec<Monkey>, monkey_item_count: &mut Vec<u128>, worry: bool) {
    for monkey_idx in 0..monkey_map.len() {
        //println!("MONKEY {}", monkey_idx);

        for item_idx in 0..monkey_map[monkey_idx].items_repr.len() {
            monkey_item_count[monkey_idx] += 1;

            let mut item = monkey_map[monkey_idx].items_repr[item_idx].to_owned();

            let printable_item = item
                .iter()
                .map(|(k, v)| (k * v.0) + v.1)
                .collect::<Vec<u128>>();

            //println!("ITEM {:?}", printable_item);

            //println!("{} {:?}", monkey_idx, item);
            let operation = monkey_map[monkey_idx].operation;
            update_repr(&mut item, &operation, worry);

            let tq = monkey_map[monkey_idx].test_quotient;
            let tm0 = monkey_map[monkey_idx].throw_monkeys[0];
            let tm1 = monkey_map[monkey_idx].throw_monkeys[1];

            let repr = item[&tq];
            //println!("tq {} Repr {} {}", tq, repr.0, repr.1);

            match repr.1 {
                0 => {
                    // println!("True, throwing to monkey {}", tm0);
                    monkey_map[tm0 as usize].items_repr.push_back(item);
                }
                _ => {
                    // println!("False, throwing to monkey {}", tm1);
                    monkey_map[tm1 as usize].items_repr.push_back(item)
                }
            }

            // println!("{}", item);
            // let printable_item = item.iter().map(|(k, v)| (k*v.0) + v.1).collect::<Vec<u128>>();
            // println!("ITEM {:?}", printable_item)
        }
        monkey_map[monkey_idx].items_repr = VecDeque::new();
    }
}

// Part 1: 55930

fn get_repr(item: &u128, divisors: &Vec<u128>) -> HashMap<u128, (u128, u128)> {
    let mut repr: HashMap<u128, (u128, u128)> = HashMap::new();

    for div in divisors {
        // println!("Item {}", item);
        repr.insert(*div, (*item / *div, *item % *div));
        // println!("Item {}", repr[div].1 + (repr[div].0 * *div));
    }

    repr
}

fn update_repr(repr: &mut HashMap<u128, (u128, u128)>, operation: &str, worry: bool) {
    let mut count = 0;

    // println!("  NEW ITEM ==============================================");
    // println!("OPERATION {}", operation);

    for (k, v) in repr {
        // println!("{:?} {:?}", k, v);

        let op_vec = operation.split(" ").collect::<Vec<&str>>();
        // println!("{:?}", op_vec);

        let operand_1 = match op_vec[2] {
            "old" => *v,
            _ => {
                let value = op_vec[2].parse::<u128>().unwrap();
                (value / k, value % k)
            }
        };

        let operand_2 = match op_vec[4] {
            "old" => *v,
            _ => {
                let value = op_vec[4].parse::<u128>().unwrap();
                (value / k, value % k)
            }
        };

        let mut result = match op_vec[3] {
            "*" => {
                let n = operand_1.0;
                let m = operand_2.0;
                let r = operand_1.1;
                let s = operand_2.1;

                // let operand_1_0_new = (n * m * k) + (r * m) + (s * n);
                let operand_1_1_new = (r * s);

                (0, operand_1_1_new % k)
            }
            "+" => {
                let operand_1_0_new = operand_1.0 + operand_2.0;
                let operand_1_1_new = operand_1.1 + operand_2.1;

                (0, ((operand_2.0 * k) + operand_2.1 + operand_1.1) % k)
            }
            _ => (0, 0),
        };

        if false {
            println!("  DIVISOR {}", k);
            println!("  OPERAND 1 {:?}", operand_1);
            println!("  OPERAND 2 {:?}", operand_2);
            println!(
                "  OPERATION {:?} {} {}",
                (k * operand_1.0) + operand_1.1,
                op_vec[3],
                operand_2.1
            );
            println!(
                "  BEFORE {:?} -> {:?} AFTER",
                (v.0 * k) + v.1,
                (result.0 * k) + result.1
            );
            count += 1;
        }

        // println!("{} {} {} = {}", operand_1, op_vec[3], operand_2, result);
        // println!("{:?}", result);

        if worry {
            //println!("  Worry decreasing");
            // divide by 3 with integer division
            let result_0_over_3 = result.0 / 3;
            let result_0_over_3_remainder = result.0 % 3;
            let result_1_over_3 = ((k * result_0_over_3_remainder) + result.1) / 3;

            result = (result_0_over_3 + result_1_over_3 / k, result_1_over_3 % k);
            //println!("  After worry / 3 : {:?}", (result.0 * k) + result.1);
        }

        *v = result;
    }
}
