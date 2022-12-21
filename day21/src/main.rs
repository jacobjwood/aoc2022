use std::fs;
use std::collections::{BTreeMap, HashMap};
use fasteval;
use fasteval::{Evaler, Compiler};

fn main() -> Result<(), fasteval::Error>  {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //assert_eq!(val, 6.0);
    let mut monkey_mapping : HashMap<String, String> = HashMap::new();
    let mut map : BTreeMap<String, f64> = BTreeMap::new();
    let mut root : String = String::new();

    for c in contents.lines() {
        let k_v = c.split(":").map(|s| s.trim()).collect::<Vec<&str>>();
        //println!("{:?}", k_v);

        if k_v[1].chars().fold(true, |_, c| c.is_numeric()) {
            map.insert(k_v[0].to_owned(), k_v[1].parse::<f64>().unwrap());
            //monkey_mapping.insert(k_v[0].to_owned(), k_v[1])
        } else {
            monkey_mapping.insert(k_v[0].to_owned(), k_v[1].to_owned());
        };

        if k_v[0] == "root" {
            root = k_v[1].to_owned();
        }

        //monkey_mapping.insert()
    }

    println!("ROOT {}", root);
    let mut root2 = root.to_owned();

    loop {
        let mut monkeys_in_eq = root.split(['+', '-', '/', '*', '(', ')']).map(|s| s.trim().to_owned()).collect::<Vec<String>>();
        //println!("monkeys_in_eq {:?}", monkeys_in_eq);

        monkeys_in_eq = monkeys_in_eq.into_iter().filter(|s| monkey_mapping.contains_key(s)).collect();
        // check for substitution
        if monkeys_in_eq.len() == 0 {
            break;
        }
        
        for monkey in monkeys_in_eq {
            let mut replacement = String::from("(");
            replacement.push_str(monkey_mapping.get(&monkey).unwrap());
            replacement.push_str(")");
            //println!("{:?}", replacement);
            root = root.replace(&monkey, &replacement);
        }
            
    }

    //println!("ROOT {}", root);
    //println!("ROOT {}", root.len());
    let mut parser = fasteval::Parser::new();
    parser.expr_len_limit = 64000;
    parser.expr_depth_limit = 1000;

    let mut slab = fasteval::Slab::with_capacity(4000);

    let expr_ref = parser.parse(&root, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    let val = expr_ref.eval(&slab, &mut map);
    //let val = fasteval::ez_eval(&root, &mut map)?;

    println!("Part 1: {:?}", val.unwrap() as i64);
    root2 = root2.replace(&"+", &"=");
    //println!("ROOT 2 {}", root2);

    loop {
        let mut monkeys_in_eq = root2.split(['+', '-', '/', '*', '(', ')', '=']).map(|s| s.trim().to_owned()).collect::<Vec<String>>();
        //println!("monkeys_in_eq {:?}", monkeys_in_eq);

        monkeys_in_eq = monkeys_in_eq.into_iter().filter(|s| monkey_mapping.contains_key(s)).collect();
        // check for substitution
        if monkeys_in_eq.len() == 0 {
            break;
        }
        
        for monkey in monkeys_in_eq {
            let mut replacement = String::from("(");
            replacement.push_str(monkey_mapping.get(&monkey).unwrap());
            replacement.push_str(")");
            //println!("{:?}", replacement);
            root2 = root2.replace(&monkey, &replacement);
        }
            
    }
    
    //println!("ROOT 2 {}", root2); 
    
    // Find the part that corresponds to no humn
    let humn_part = root2.split("=").filter(|s| s.contains("humn")).collect::<String>();
    let eval_part = root2.split("=").filter(|s| !s.contains("humn")).collect::<String>();

    let mut parser = fasteval::Parser::new();
    parser.expr_len_limit = 64000;
    parser.expr_depth_limit = 1000;

    let mut slab = fasteval::Slab::with_capacity(4000);

    let humn_part_expr = parser.parse(&humn_part, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    let eval_part_expr = parser.parse(&eval_part, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    

    //let mut step = 100;
    map.insert(String::from("humn"), 0.0);
    
    let mut step_size : i64 = 10000000000000;
    let mut too_low_prev = false;

    loop {
        let eval_part_val = eval_part_expr.eval(&slab, &mut map);
        let humn_part_val = humn_part_expr.eval(&slab, &mut map);
        let delta = humn_part_val.to_owned().unwrap() - eval_part_val.to_owned().unwrap();
        let too_low = delta < 0.0;

        if delta.abs() < step_size as f64 && step_size != 1 || (too_low != too_low_prev) {
            step_size /= 10;
        }

        //println!("{:?}", eval_part_val.to_owned().unwrap() - humn_part_val.to_owned().unwrap());
        //println!("{}", map.get("humn").unwrap());
        //println!("{}", delta);

        if eval_part_val.unwrap() == humn_part_val.unwrap() {
            println!("Part 2: {}", map.get("humn").unwrap());
            break;
        } else {
            if too_low {
                    *map.get_mut("humn").unwrap() -= step_size as f64;
            } else {
                *map.get_mut("humn").unwrap() += step_size as f64;
            }
        }

        too_low_prev = too_low;
    }
    //println!("{}", humn_part_val.unwrap());
    //println!("{}", eval_part_val.unwrap());

    Ok(())
}
