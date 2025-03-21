use std::collections::{HashMap, HashSet};
impl Solution {
    fn search<'a>(
        recipe: &'a str,
        recipe_map: &mut HashMap<&'a str, &'a Vec<String>>,
        supplies: &mut HashSet<&'a str>,
    ) {
        if supplies.contains(recipe) {
            return;
        }
        if let Some(ingredient_list) = recipe_map.remove(recipe) {
            for ingredient in ingredient_list {
                if supplies.contains(ingredient.as_str()) {
                    continue;
                }
                Self::search(ingredient, recipe_map, supplies);
                if !recipe_map.contains_key(ingredient.as_str()) {
                    return;
                }
            }
            recipe_map.insert(recipe, ingredient_list);
            supplies.insert(recipe);
        }
    }
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut supplies_map: HashSet<&str> = HashSet::new();
        for ingredient in &supplies {
            supplies_map.insert(ingredient);
        }
        let mut recipe_map: HashMap<&str, &Vec<String>> = HashMap::new();
        recipes
            .iter()
            .zip(ingredients.iter())
            .for_each(|(recipe, ingredient_list)| {
                recipe_map.insert(recipe, ingredient_list);
            });
        for recipe in &recipes {
            Self::search(recipe, &mut recipe_map, &mut supplies_map);
        }
        recipe_map.keys().map(|r| r.to_string()).collect()
    }
}
