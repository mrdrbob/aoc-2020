
pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\04.txt").unwrap();

    let total_valid_items = file.split("\n\n").filter(|x|{ x.len() > 0 }).filter(|l| {
        let t:Vec<(String, String)> = l.split("\n").filter(|x|{ x.len() > 0 }).map(|l2| {
            l2.split(" ").map(|kvp| {
                let split:Vec<&str> = kvp.split(":").collect();
                (split[0].to_owned(), split[1].to_owned())
            })
        }).flatten().collect();
        validate(&t)
    }).count();

    println!("{}", total_valid_items);
}

const REQ_KEYS: [&'static str; 7] = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

fn validate(key_value_pairs:&Vec<(String, String)>) -> bool {
    let is_missing_keys = REQ_KEYS.iter().find(|req_key| { key_value_pairs.into_iter().find(|(key, _)| { key == req_key.clone() }).is_none()  }).is_some();
    !is_missing_keys
}


