use std::collections::HashMap;

pub fn execute() {
    let numbers = vec![0,5,4,1,10,14];
    let mut last_number = 7;
    let mut turn_history:HashMap<i32, i32> = HashMap::new();
    let mut current_turn = 1;

    for n in numbers {
        turn_history.insert(n, current_turn);
        current_turn += 1;
    }

    while current_turn < 30000000 {
        let previous_value = turn_history.insert(last_number, current_turn);
        let next = match previous_value {
            None => 0,
            Some(v) => current_turn - v
        };

        current_turn += 1;
        last_number = next;
    }

    println!("{}", last_number);
}
