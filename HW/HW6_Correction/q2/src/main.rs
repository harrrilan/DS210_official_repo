use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

fn data_file(name: &str) -> PathBuf {
    PathBuf::from("../data").join(name)      // ../data/<name>
}

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

// fix this perhaps
fn popular_recipes(
    category_to_ingredients: &HashMap<String, HashSet<String>>,
    person_to_categories: &HashMap<String, HashSet<String>>,
    recipe_to_ingredients: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    // 1. count how many people like each recipe
    let mut recipe_popularity: HashMap<String, usize> = HashMap::new();

    for person in person_to_categories.keys() {
        for recipe in recipe_to_ingredients.keys() {
            if does_person_like_recipe(
                person,
                recipe,
                category_to_ingredients,
                person_to_categories,
                recipe_to_ingredients,
            ) {
                *recipe_popularity.entry(recipe.clone()).or_insert(0) += 1;
            }
        }
    }

    // 2. rank by (likes DESC, name DESC) and return top 3
    let mut ranked: Vec<(String, usize)> = recipe_popularity.into_iter().collect();
    ranked.sort_by(|(r1, c1), (r2, c2)| c2.cmp(c1).then_with(|| r2.cmp(r1)));

    ranked
        .into_iter()
        .take(3)
        .map(|(name, _)| name)
        .collect()
}


fn main() {
    let category_to_ingredients =
        load_category_ingredients(data_file("categories_ingredients.txt").to_str().unwrap());
    let person_to_categories =
        load_people_categories(data_file("people_categories.txt").to_str().unwrap());
    let recipe_to_ingredients =
        load_recipes(data_file("recipes.txt").to_str().unwrap());

    let popular_recipes_list = popular_recipes(
        &category_to_ingredients,
        &person_to_categories,
        &recipe_to_ingredients,
    );

    println!("The 3 most popular recipes are: {:?}", popular_recipes_list);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    /// quick helper to build a HashSet<String>
    fn hs(items: &[&str]) -> HashSet<String> {
        HashSet::from_iter(items.iter().map(|s| s.to_string()))
    }

    #[test]
    fn unique_winner() {
        // Person Alice likes only R1; Bob likes none → R1 wins.
        let cti = HashMap::from([("fruit".into(), hs(&["apple"]))]);
        let p2c = HashMap::from([
            ("Alice".into(), hs(&["fruit"])),
            ("Bob".into(),   hs(&[])),
        ]);
        let r2i = HashMap::from([
            ("R1".into(), vec!["apple".into()]),
            ("R2".into(), vec!["kiwi".into()]),
        ]);

        let out = popular_recipes(&cti, &p2c, &r2i);
        assert_eq!(out, vec!["R1"]);
    }

    #[test]
    fn tie_break_descending_name() {
        // Both recipes liked by Alice → tie on popularity.
        // Reverse-lexicographic tie-break: “ZucchiniCake” before “ApplePie”.
        let cti = HashMap::from([("veg".into(), hs(&["carrot"]))]);
        let p2c = HashMap::from([("Alice".into(), hs(&["veg"]))]);
        let r2i = HashMap::from([
            ("ApplePie".into(),     vec!["carrot".into()]),
            ("ZucchiniCake".into(), vec!["carrot".into()]),
        ]);

        let out = popular_recipes(&cti, &p2c, &r2i);
        assert_eq!(out, vec!["ZucchiniCake", "ApplePie"]);
    }

    #[test]
    fn sixty_percent_rule() {
        // Recipe has 5 ingredients.
        // Alice likes 3 (≥60 %), Bob likes 2 (<60 %) → only Alice counts.
        let cti = HashMap::from([("fav".into(), hs(&["a", "b", "c"]))]);
        let p2c = HashMap::from([
            ("Alice".into(), hs(&["fav"])),
            ("Bob".into(),   hs(&["fav"])),
        ]);
        let r2i = HashMap::from([(
            "R".into(),
            vec!["a", "b", "c", "x", "y"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        )]);

        let out = popular_recipes(&cti, &p2c, &r2i);
        assert_eq!(out, vec!["R"]);
    }

    #[test]
    fn zero_ingredient_recipe_not_liked() {
        // Edge case: recipe with no ingredients should not be liked by anyone.
        let out = popular_recipes(
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::from([("Empty".into(), Vec::<String>::new())]),
        );
        assert!(out.is_empty());
    }
}
