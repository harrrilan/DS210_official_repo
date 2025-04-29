use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

fn read_file(file_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn load_category_ingredients(path: &Path) -> io::Result<HashMap<String, HashSet<String>>> {
    let mut map = HashMap::new();
    for line in read_file(path)? {
        let (cat, list) = split_once(&line)?;
        map.insert(cat, list.into_iter().collect());
    }
    Ok(map)
}

fn load_people_categories(path: &Path) -> io::Result<HashMap<String, HashSet<String>>> {
    let mut map = HashMap::new();
    for line in read_file(path)? {
        let (person, list) = split_once(&line)?;
        map.insert(person, list.into_iter().collect());
    }
    Ok(map)
}

fn load_recipes(path: &Path) -> io::Result<HashMap<String, Vec<String>>> {
    let mut map = HashMap::new();
    for line in read_file(path)? {
        let (recipe, list) = split_once(&line)?;
        map.insert(recipe, list);
    }
    Ok(map)
}

fn split_once(line: &str) -> io::Result<(String, Vec<String>)> {
    let (left, right) = line
        .split_once(':')
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing ':'"))?;
    let list = right
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok((left.trim().to_string(), list))
}

fn does_person_like_recipe(
    person: &str,
    recipe: &str,
    cat2ing: &HashMap<String, HashSet<String>>,
    person2cat: &HashMap<String, HashSet<String>>,
    recipe2ing: &HashMap<String, Vec<String>>,
) -> bool {
    let person_cats = match person2cat.get(person) {
        Some(c) => c,
        None => return false,
    };
    let rec_ings = match recipe2ing.get(recipe) {
        Some(r) => r,
        None => return false,
    };
    if rec_ings.is_empty() {
        return false;
    }
    let liked = rec_ings
        .iter()
        .filter(|ing| {
            person_cats.iter().any(|cat| {
                cat2ing
                    .get(cat)
                    .map(|set| set.contains(*ing))
                    .unwrap_or(false)
            })
        })
        .count();
    liked * 10 >= rec_ings.len() * 6 // ≥60 %
}

fn get_recipes_person_likes(
    person: &str,
    cat2ing: &HashMap<String, HashSet<String>>,
    person2cat: &HashMap<String, HashSet<String>>,
    recipe2ing: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    recipe2ing
        .keys()
        .filter(|r| does_person_like_recipe(person, r, cat2ing, person2cat, recipe2ing))
        .cloned()
        .collect()
}

/// top-3 recipes by number of fans; ties broken by **reverse** lexicographic order
fn popular_recipes(
    cat2ing: &HashMap<String, HashSet<String>>,
    person2cat: &HashMap<String, HashSet<String>>,
    recipe2ing: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut pop: HashMap<String, usize> = HashMap::new();
    for person in person2cat.keys() {
        for recipe in recipe2ing.keys() {
            if does_person_like_recipe(person, recipe, cat2ing, person2cat, recipe2ing) {
                *pop.entry(recipe.clone()).or_insert(0) += 1;
            }
        }
    }
    let mut ranked: Vec<(String, usize)> = pop.into_iter().collect();
    ranked.sort_by(|(r1, c1), (r2, c2)| c2.cmp(c1).then_with(|| r2.cmp(r1))); // desc likes, desc name
    ranked.into_iter().take(3).map(|(r, _)| r).collect()
}


fn main() -> Result<(), Box<dyn Error>> {
let mut args = env::args().skip(1);          // drop argv[0]
let mut data_dir = PathBuf::from("../data"); // default if --data not given

// pull the first positional token, if any
let first = args
    .next()
    .ok_or("usage: <program> [--data DIR] <command> …")?;

let cmd = if first == "--data" {
    // pattern: --data <DIR> <command> …
    data_dir = PathBuf::from(
        args.next()
            .ok_or("`--data <DIR>`: directory is missing")?,
    );
    args.next()
        .ok_or("command missing after `--data <DIR>`")?
} else {
    // pattern: <command> …
    first
};
    let cat2ing = load_category_ingredients(&data_dir.join("categories_ingredients.txt"))?;
    let person2cat = load_people_categories(&data_dir.join("people_categories.txt"))?;
    let recipe2ing = load_recipes(&data_dir.join("recipes.txt"))?;

    match cmd.as_str() {
        "likes" => {
            let person = args.next().ok_or("likes <PERSON> <RECIPE>")?;
            let recipe = args.next().ok_or("likes <PERSON> <RECIPE>")?;
            let verdict = does_person_like_recipe(
                &person, &recipe, &cat2ing, &person2cat, &recipe2ing,
            );
            println!(
                "{} {} the recipe {}",
                person,
                if verdict { "likes" } else { "does not like" },
                recipe
            );
        }
        "list" => {
            let person = args.next().ok_or("list <PERSON>")?;
            let list = get_recipes_person_likes(&person, &cat2ing, &person2cat, &recipe2ing);
            println!("Recipes {} likes: {:?}", person, list);
        }
        "popular" => {
            let top = popular_recipes(&cat2ing, &person2cat, &recipe2ing);
            println!("Top-3 popular recipes: {:?}", top);
        }
        _ => return Err("unknown command".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    fn hs(items: &[&str]) -> HashSet<String> {
        HashSet::from_iter(items.iter().map(|s| s.to_string()))
    }

    /// small fixture with one person who likes "fruit"
    fn fixture() -> (
        HashMap<String, HashSet<String>>,
        HashMap<String, HashSet<String>>,
        HashMap<String, Vec<String>>,
    ) {
        let cat2ing = HashMap::from([("fruit".into(), hs(&["apple", "banana"]))]);
        let person2cat = HashMap::from([("Alice".into(), hs(&["fruit"]))]);
        let recipe2ing = HashMap::from([
            ("ApplePie".into(), vec!["apple".into()]),
            ("BananaBread".into(), vec!["banana".into()]),
        ]);
        (cat2ing, person2cat, recipe2ing)
    }

    #[test]
    fn popularity_and_tie_break() {
        let (c, p, r) = fixture();

        // both recipes liked by Alice → tie on popularity (count = 1 each)
        // reverse-lexicographic ⇒ BananaBread (B) before ApplePie (A)
        let top = popular_recipes(&c, &p, &r);
        assert_eq!(top, vec!["BananaBread", "ApplePie"]);
    }

    #[test]
    fn percent_rule_below_sixty_is_false() {
        // recipe has 5 ingredients, Alice likes only 2 (40 %) → should be false
        let (mut c, mut p, mut r) = fixture();
        c.insert("veg".into(), hs(&["x", "y"]));
        p.insert("Bob".into(), hs(&["veg"])); // not used, just to vary data

        r.insert(
            "BigSalad".into(),
            vec!["apple", "x", "y", "z", "w"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        );

        assert!(
            !does_person_like_recipe("Alice", "BigSalad", &c, &p, &r),
            "liking only 40 % of the ingredients should fail the 60 % test"
        );
    }

    #[test]
    fn zero_ingredient_recipe_rejected() {
        let c = HashMap::new();
        let p = HashMap::from([("A".into(), HashSet::new())]);
        let r = HashMap::from([("Empty".into(), Vec::<String>::new())]);

        assert!(!does_person_like_recipe("A", "Empty", &c, &p, &r));
    }
}
