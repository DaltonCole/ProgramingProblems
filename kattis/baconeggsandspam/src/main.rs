use std::collections::HashMap;

fn get_num() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_order() -> (String, Vec<String>) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<String> = line.trim()
        .split(" ")
        .map(|x| x.to_string())
        .collect();
    let name = v.swap_remove(0);
    (name, v)
}

fn get_all_orders(n: u32) -> HashMap<String, Vec<String>> {
    let mut orders = HashMap::<String, Vec<String>>::new();
    for _ in 0..n {
        let (name, order) = get_order();
        orders.insert(name, order);
    }
    orders
}

fn names_to_order(names: &mut HashMap<String, Vec<String>>) -> (Vec<String>, HashMap<String, Vec<String>>) {
    let mut orders_name = HashMap::<String, Vec<String>>::new();
    for (name, orders) in names {
        for order in orders {
            match orders_name.get_mut(order) {
                Some(v) => { v.push(name.to_string()); },
                None => { orders_name.insert((*order).clone(), vec![name.to_string()]); }
            }
        }
    }

    let mut foods = Vec::<String>::new();
    for key in orders_name.keys() {
        foods.push(key.clone());
    }
    foods.sort();

    for key in &foods {
        orders_name.get_mut(key).unwrap().sort();
    }

    (foods, orders_name)
}



fn main() {
    loop {
        let num = get_num();
        if num == 0 {
            break;
        }
        let mut orders = get_all_orders(num);
        let (foods, orders_name) = names_to_order(&mut orders);

        for food in foods {
            print!("{}", food);
            for name in orders_name.get(&food).unwrap().iter() {
                print!(" {}", name);
            }
            println!("");
        }
        println!("");
    }
}
