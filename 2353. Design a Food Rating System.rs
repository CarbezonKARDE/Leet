use std::collections::{BTreeSet, HashMap};

#[derive(Clone, PartialEq, Eq)]
struct FoodItem {
    name: String,
    cuisine: String,
    rating: i32,
}
impl PartialOrd for FoodItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FoodItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rating.cmp(&other.rating) {
            std::cmp::Ordering::Equal => other.name.cmp(&self.name),
            ord => ord,
        }
    }
}
struct FoodRatings {
    highest_ratings: HashMap<String, BTreeSet<FoodItem>>,
    food: HashMap<String, FoodItem>,
}
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut highest_ratings: HashMap<_, BTreeSet<FoodItem>> = HashMap::new();
        let mut food = HashMap::new();
        for ((f, c), r) in foods.into_iter().zip(cuisines).zip(ratings) {
            let fi = FoodItem {
                name: f.clone(),
                cuisine: c.clone(),
                rating: r,
            };
            highest_ratings.entry(c).or_default().insert(fi.clone());
            food.insert(f, fi);
        }
        Self { highest_ratings, food }
    }
    fn change_rating(&mut self, food: String, new_rating: i32) {
        let fi = self.food.get_mut(&food).unwrap();
        let hr = self.highest_ratings.get_mut(&fi.cuisine).unwrap();
        hr.remove(fi);
        fi.rating = new_rating;
        hr.insert(fi.clone());
    }
    fn highest_rated(&self, cuisine: String) -> String {
        self.highest_ratings.get(&cuisine).unwrap().last().unwrap().name.clone()
    }
}
