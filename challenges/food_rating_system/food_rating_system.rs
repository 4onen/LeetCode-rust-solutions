#![allow(dead_code)]

use std::collections::{BTreeSet, HashMap};

#[derive(PartialEq, Eq, Clone, Copy)]
struct FoodRating<'a> {
    rating: i32,
    food: &'a str,
}

impl PartialOrd for FoodRating<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FoodRating<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rating
            .cmp(&other.rating)
            .then_with(|| self.food.cmp(&other.food).reverse())
    }
}

struct SubCuisineFoodRating<'a> {
    ratings: HashMap<&'a str, i32>,
    rating_order: BTreeSet<FoodRating<'a>>,
}

impl<'a> SubCuisineFoodRating<'a> {
    fn new() -> Self {
        Self {
            ratings: HashMap::new(),
            rating_order: BTreeSet::new(),
        }
    }

    fn insert(&mut self, food: &'a str, rating: i32) {
        // insert must only be called if the food is not in the ratings
        assert!(self.ratings.insert(food, rating).is_none());
        self.rating_order.insert(FoodRating { rating, food });
    }

    fn update(&mut self, food: &'a str, new_rating: i32) {
        if let Some(old_rating) = self.ratings.get_mut(food) {
            self.rating_order.remove(&FoodRating {
                rating: *old_rating,
                food,
            });
            *old_rating = new_rating;
        } else {
            self.insert(food, new_rating);
        }

        self.rating_order.insert(FoodRating {
            rating: new_rating,
            food,
        });
    }

    fn highest_rated(&self) -> Option<&'a str> {
        self.rating_order
            .iter()
            .next_back()
            .map(|food_rating| food_rating.food)
    }
}

//// Self-referential struct!
pub struct FoodRatings {
    // Actually has a lifetime of 'cuisines
    // Declared first so that it is dropped before cuisines
    cuisine_to_food: HashMap<&'static str, SubCuisineFoodRating<'static>>,
    // Ditto to above
    food_to_cuisine: HashMap<&'static str, &'static str>,

    // Safety: Must not be moved out of, mutated, nor dropped before
    // cuisine_to_food and food_to_cuisine.
    cuisines: Vec<String>,
    foods: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        // Safety: These references will be contained to the lifetimes of
        // the struct being constructed.
        let food_refs = foods
            .iter()
            .map(|s| -> &'static str { unsafe { std::mem::transmute(s.as_str()) } });
        let cuisine_refs = cuisines
            .iter()
            .map(|s| -> &'static str { unsafe { std::mem::transmute(s.as_str()) } });

        let food_to_cuisine = Iterator::zip(food_refs.clone(), cuisine_refs.clone()).collect();

        let mut cuisine_to_food = HashMap::new();
        for ((cuisine, food), rating) in Iterator::zip(
            Iterator::zip(cuisine_refs.clone(), food_refs.clone()),
            ratings.iter(),
        ) {
            cuisine_to_food
                .entry(cuisine)
                .or_insert_with(SubCuisineFoodRating::new)
                .insert(food, *rating);
        }

        Self {
            cuisine_to_food,
            food_to_cuisine,
            cuisines,
            foods,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (food, cuisine) = self.food_to_cuisine.get_key_value(food.as_str()).unwrap();
        self.cuisine_to_food
            .get_mut(cuisine)
            .unwrap()
            .update(food, new_rating);
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        self.cuisine_to_food
            .get(&cuisine.as_str())
            .unwrap()
            .highest_rated()
            .unwrap()
            .to_owned()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut ratings = FoodRatings::new(
            ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            [
                "korean", "japanese", "japanese", "greek", "japanese", "korean",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
            vec![9, 12, 8, 15, 14, 7],
        );

        assert_eq!(ratings.highest_rated("korean".to_owned()), "kimchi");
        assert_eq!(ratings.highest_rated("japanese".to_owned()), "ramen");
        ratings.change_rating("sushi".to_owned(), 16);
        assert_eq!(ratings.highest_rated("japanese".to_owned()), "sushi");
        ratings.change_rating("ramen".to_owned(), 16);
        assert_eq!(ratings.highest_rated("japanese".to_owned()), "ramen");
    }
}
