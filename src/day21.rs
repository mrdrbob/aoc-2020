
use std::collections::HashSet;
use std::collections::HashMap;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\21.txt").unwrap();

    let all_recipes:Vec<Recipe> = file.lines().map(|line| {
        let split:Vec<&str> = line.split(" (contains ").collect();
        let ingredients:HashSet<&str> = split[0].split(" ").collect();
        let allergens:HashSet<&str> = split[1].strip_suffix(")").unwrap().split(", ").collect();

        Recipe {
            allergens: allergens,
            ingredients: ingredients
        }
    }).collect();

    // Allergen - possible ingredients
    let mut possible_matches:HashMap<&str,HashSet<&str>> = HashMap::new();
    let all_ingredients:HashSet<&str> = all_recipes.iter().flat_map(|recipe| { &recipe.ingredients }).map(|&x|{ x }).collect();

    for recipe in all_recipes.iter() {
        for allergen in recipe.allergens.iter() {
            let new_possible_values:HashSet<&str> = match possible_matches.remove(allergen) {
                None => recipe.ingredients.clone(),
                Some(current) => current.intersection(&recipe.ingredients).map(|&x| { x }).collect()
            };
            possible_matches.insert(allergen, new_possible_values);
        }
    }

    let ingredients_with_possible_allergens:HashSet<&str> = possible_matches.iter().flat_map(|(_, v)| { v }).map(|&x| { x }).collect();
    let safe_ingredients:HashSet<&str> = all_ingredients.difference(&ingredients_with_possible_allergens).map(|&x| { x }).collect();
    let total_references_to_safe_ingredients:usize = all_recipes.iter().map(|recipe| {
        recipe.ingredients.iter().filter(|i| { safe_ingredients.contains(*i) }).count()
    }).sum();
    println!("{}", all_ingredients.len());
    println!("{}", safe_ingredients.len());
    println!("{}", total_references_to_safe_ingredients);

    let mut solver = IngredientsSolver { 
        possible_matches: possible_matches,
        solved_matches: HashMap::new()
    };

    solver.solve();
    solver.print_solved_matches();

}

struct IngredientsSolver<'a> {
    possible_matches:HashMap<&'a str, HashSet<&'a str>>,
    solved_matches:HashMap<&'a str, &'a str>
}

impl IngredientsSolver<'_> {
    fn solve(&mut self) {
        while self.possible_matches.iter().len() > 0 {
            println!("{:?}", self.possible_matches);

            let (&key, value) = self.possible_matches.iter().filter(|(_,v)| { v.len() == 1 }).next().unwrap();
            let found_value = *value.iter().next().unwrap();
            self.solved_matches.insert(key, found_value);
            self.possible_matches.remove(key);

            for (_, v) in self.possible_matches.iter_mut() {
                v.retain(|&x| { x != found_value })
            }
        }
    }

    fn print_solved_matches(&self) {
        let mut allergens:Vec<&str> = self.solved_matches.iter().map(|(&k, _)| { k }).collect();
        allergens.sort();
        println!("{:?}", allergens);

        for &al in allergens.iter() {
            let ingredient = self.solved_matches.get(al).unwrap();
            print!("{},", ingredient);
        }
        println!();
    }
}

struct Recipe<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>
}

