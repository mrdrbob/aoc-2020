
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
    let is_missing_keys = REQ_KEYS.iter().any(|req_key| { 
        !key_value_pairs.into_iter().any(|(key, _)| { key == req_key.clone() })
    });
    if is_missing_keys {
        false
    } else {
        key_value_pairs.into_iter().map(|(key, value)| {
            match key.as_str() {
                "byr" => validate_yr(value, 1920, 2002),
                "iyr" => validate_yr(value, 2010, 2020),
                "eyr" => validate_yr(value, 2020, 2030),
                "hgt" => validate_height(value),
                "hcl" => validate_hex(value),
                "ecl" => validate_eye_color(value),
                "pid" => validate_id(value),
                "cid" => true,
                _ => { println!("{}", key); panic!() }
            }
        }).all(|x| { x })
    }
}

fn validate_yr(value:&String, at_least:i32, at_most:i32) -> bool {
    match value.parse::<i32>() {
        Err(_) => false,
        Ok(as_int) => as_int >= at_least && as_int <= at_most
    }
}

fn validate_height(value:&String) -> bool {
    if value.len() < 3 {
        false
    } else {
        let len = value.chars().count();
        let unit:String = value.chars().rev().take(2).collect();
        let number:String = value.chars().take(len - 2).collect();

        match unit.as_str() {
            "mc" => validate_yr(&number, 150, 193),
            "ni" => validate_yr(&number, 59, 76),
            _ => false
        }
    }
}

fn validate_hex(value:&String) -> bool {
    if value.chars().count() != 7 {
        false
    } else if value.chars().next().unwrap() != '#' {
        false
    } else {
        value.chars().skip(1).all(|c| { 
            "0123456789abcdef".contains(c.clone())
        })
    }
}

fn validate_eye_color(color:&String) -> bool {
    match color.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn validate_id(value:&String) -> bool {
    if value.len() != 9 {
        false
    } else {
        value.chars().all(|c| { 
            "0123456789".contains(c)
        })
    }
}