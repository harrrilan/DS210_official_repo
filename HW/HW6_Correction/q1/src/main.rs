use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn load_category_ingredients(file_path: &str) -> HashMap<String, HashSet<String>> {
    let mut category_to_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    let lines = read_file(file_path).expect("Failed to read file");

    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let category = parts[0].trim().to_string();
            let ingredients_str = parts[1].trim();
            let ingredients: HashSet<String> = ingredients_str
                .split(',')
                .map(|ingredient| ingredient.trim().to_string())
                .collect();
            category_to_ingredients.insert(category, ingredients);
        }
    }
    category_to_ingredients
}

fn load_people_categories(file_path: &str) -> HashMap<String, HashSet<String>> {
    let mut person_to_categories: HashMap<String, HashSet<String>> = HashMap::new();
    let lines = read_file(file_path).expect("Failed to read file");

    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let person_name = parts[0].trim().to_string();
            let categories_str = parts[1].trim();
            let categories: HashSet<String> = categories_str
                .split(',')
                .map(|category| category.trim().to_string())
                .collect();
            person_to_categories.insert(person_name, categories);
        }
    }
    person_to_categories
}

fn load_recipes(file_path: &str) -> HashMap<String, Vec<String>> {
    let mut recipe_to_ingredients: HashMap<String, Vec<String>> = HashMap::new();
    let lines = read_file(file_path).expect("Failed to read file");

    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let recipe_name = parts[0].trim().to_string();
            let ingredients_str = parts[1].trim();
            let ingredients: Vec<String> = ingredients_str
                .split(',')
                .map(|ingredient| ingredient.trim().to_string())
                .collect();
            recipe_to_ingredients.insert(recipe_name, ingredients);
        }
    }
    recipe_to_ingredients
}

fn does_person_like_recipe(
    person_name: &str,
    recipe_name: &str,
    category_to_ingredients: &HashMap<String, HashSet<String>>,
    person_to_categories: &HashMap<String, HashSet<String>>,
    recipe_to_ingredients: &HashMap<String, Vec<String>>,
) -> bool {
    if let Some(person_categories) = person_to_categories.get(person_name) {
        if let Some(recipe_ingredients) = recipe_to_ingredients.get(recipe_name) {
            let total_ingredients = recipe_ingredients.len();
            let mut count_liked_ingredients = 0;

            for ingredient in recipe_ingredients {
                for category in person_categories {
                    if let Some(ingredients) = category_to_ingredients.get(category) {
                        if ingredients.contains(ingredient) {
                            count_liked_ingredients += 1;
                            break;
                        }
                    }
                }
            }

            let required_percentage = (total_ingredients as f32 * 0.6).ceil() as usize;
            return count_liked_ingredients >= required_percentage;
        }
    }
    false
}

fn get_recipes_person_likes(
    person_name: &str,
    category_to_ingredients: &HashMap<String, HashSet<String>>,
    person_to_categories: &HashMap<String, HashSet<String>>,
    recipe_to_ingredients: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut liked_recipes = Vec::new();
    for recipe_name in recipe_to_ingredients.keys() {
        if does_person_like_recipe(
            person_name,
            recipe_name,
            category_to_ingredients,
            person_to_categories,
            recipe_to_ingredients,
        ) {
            liked_recipes.push(recipe_name.clone());
        }
    }
    liked_recipes
}



// Tie breaking should have been done in reverse lexographic order
// where is test case???
fn main() {

    // fix file path. DO NOT HARD CODE
    let category_to_ingredients = load_category_ingredients("/data/categories_ingredients.txt");
    let person_to_categories = load_people_categories("/data/people_categories.txt");
    let recipe_to_ingredients = load_recipes("/data/recipes.txt");

    // take input from terminal
    // (what operation we are doing, and what are the person and recipe names)
    let person_name = "Halle Vaughn";
    let recipe_name = "Recipe0";

    if does_person_like_recipe(
        person_name,
        recipe_name,
        &category_to_ingredients,
        &person_to_categories,
        &recipe_to_ingredients,
    ) {
        println!("{} likes the recipe: {}", person_name, recipe_name);
    } else {
        println!("{} does not like the recipe: {}", person_name, recipe_name);
    }

    let liked_recipes = get_recipes_person_likes(
        person_name,
        &category_to_ingredients,
        &person_to_categories,
        &recipe_to_ingredients,
    );
    println!("Recipes that {} likes: {:?}", person_name, liked_recipes);
}
