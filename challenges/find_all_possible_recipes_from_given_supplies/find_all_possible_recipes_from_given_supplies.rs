// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/

pub struct Solution;

// Naive brute-force sol'n
impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut created = std::collections::HashSet::new();
        let mut supply_set = std::collections::HashSet::new();
        supply_set.extend(supplies.into_iter());
        loop {
            let mut made_something = false;
            for (recipe, ingredients) in std::iter::zip(recipes.iter(), ingredients.iter()) {
                if created.contains(recipe) {
                    continue;
                }
                if ingredients
                    .iter()
                    .all(|ingredient| supply_set.contains(ingredient))
                {
                    made_something = true;
                    supply_set.insert(recipe.to_owned());
                    created.insert(recipe);
                }
            }
            if !made_something {
                break;
            }
        }
        created.into_iter().cloned().collect()
    }
}

// Optimized representation brute force sol'n
// impl Solution {
//     pub fn find_all_recipes(
//         mut recipes: Vec<String>,
//         ingredients: Vec<Vec<String>>,
//         supplies: Vec<String>,
//     ) -> Vec<String> {
//         let mut created = 0u128;
//         let mut index_map: std::collections::HashMap<&String, u8> = recipes
//             .iter()
//             .enumerate()
//             .map(|(a, b)| (b, a as u8))
//             .collect();
//         let mut graph = vec![0u128; recipes.len()];
//         for (i, (recipe, ingredients)) in std::iter::zip(recipes.iter(), ingredients).enumerate() {
//             for ingredient in ingredients {
//                 if let Some(&index) = index_map.get(&ingredient) {
//                     graph[i as usize] |= 1 << index;
//                     continue;
//                 } else if supplies.contains(&ingredient) {
//                     continue;
//                 } else {
//                     index_map.remove(recipe);
//                     graph[i as usize] = u128::MAX; // Mark recipe as impossible
//                 }
//             }
//         }
//         // Brute-force recipe solving
//         loop {
//             let mut made_something = false;
//             for i in 0..recipes.len() {
//                 if created & 1 << i == 0 && graph[i as usize] & (!created) == 0 {
//                     made_something = true;
//                     created |= 1 << i;
//                 }
//             }
//             if !made_something {
//                 break;
//             }
//         }
//         (0..recipes.len())
//             .filter_map(|i| {
//                 if created & 1 << i > 0 {
//                     Some(std::mem::take(&mut recipes[i]))
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(recipes: &[&str], ingredients: &[&[&str]], supplies: &[&str], expected: &[&str]) {
        fn check_food(food: &str, kind: &'static str) {
            assert!(food.len() >= 1, "{} empty: {:?}", kind, food);
            assert!(food.len() <= 10, "{} too long: {:?}", kind, food);
            for &b in food.as_bytes() {
                assert!(b >= b'a', "{} unexpected char: {:?}", kind, food);
                assert!(b <= b'z', "{} unexpected char: {:?}", kind, food);
            }
        }
        fn check_list(list: &[&str], kind: &'static str) {
            assert!(list.len() >= 1, "{} list empty", kind);
            assert!(list.len() <= 100, "{} list too long: {:?}", kind, list);
            let mut seen = std::collections::HashSet::new();
            for &el in list {
                check_food(el, kind);
                assert!(seen.insert(el));
            }
        }
        assert!(recipes.len() >= 1);
        assert!(recipes.len() <= 100);
        let mut seen = std::collections::HashSet::new();
        for &result in recipes {
            check_food(result, "recipe result");
            assert!(seen.insert(result));
        }
        assert_eq!(recipes.len(), ingredients.len());
        for &list in ingredients {
            check_list(list, "ingredient");
        }
        assert!(supplies.len() >= 1);
        assert!(supplies.len() <= 100);
        for &supply in supplies {
            check_food(supply, "supply");
            assert!(seen.insert(supply));
        }
        let result_set: std::collections::HashSet<String> = Solution::find_all_recipes(
            recipes.iter().map(|&x| x.to_owned()).collect(),
            ingredients
                .iter()
                .map(|&x| x.iter().map(|&x| x.to_owned()).collect())
                .collect(),
            supplies.iter().map(|&x| x.to_owned()).collect(),
        )
        .into_iter()
        .collect();
        let expected_set: std::collections::HashSet<String> =
            expected.iter().map(|&x| x.to_owned()).collect();
        assert_eq!(result_set, expected_set);
    }

    #[test]
    fn ex1() {
        test(
            &["bread"],
            &[&["yeast", "flour"]],
            &["yeast", "flour", "corn"],
            &["bread"],
        )
    }

    #[test]
    fn ex2() {
        test(
            &["bread", "sandwich"],
            &[&["yeast", "flour"], &["bread", "meat"]],
            &["yeast", "flour", "meat"],
            &["bread", "sandwich"],
        )
    }

    #[test]
    fn ex3() {
        test(
            &["bread", "sandwich", "burger"],
            &[
                &["yeast", "flour"],
                &["bread", "meat"],
                &["sandwich", "meat", "bread"],
            ],
            &["yeast", "flour", "meat"],
            &["bread", "sandwich", "burger"],
        )
    }

    #[test]
    fn myex1() {
        // Can't produce something
        // with ingredients we don't have
        test(
            &["pastastew"],
            &[&["meat", "pasta", "vegetables", "tomatoes"]],
            &["meat", "tomatoes"],
            &[],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        // Generate a test case up to all the limits
        // 100 recipes, 100 ingredients per recipe, 100 supplies
        // The first recipe depends on the second as its first ingredient,
        // the second on the third, the third on the fourth, etc.

        // Recipes are generated with the format "r##" where ## are lowercase letters
        // Supplies are generated with the format "s##" where ## are lowercase letters
        // All recipes depend on (have ingredients of) the next recipe and the last 99 supplies
        // except the last, which depends on all 100 supplies
        let mut recipes = std::vec::Vec::with_capacity(100);
        let mut supplies = std::vec::Vec::with_capacity(100);
        for i in 0..100u8 {
            let recipe = format!("r{}{}", (b'a' + i / 26) as char, (b'a' + i % 26) as char);
            let supply = format!("s{}{}", (b'a' + i / 26) as char, (b'a' + i % 26) as char);
            recipes.push(recipe);
            supplies.push(supply);
        }
        let mut ingredients = std::vec::Vec::with_capacity(100);
        for i in 0..100u8 {
            let mut list: Vec<&str> = supplies.iter().map(String::as_str).collect();
            if i < 99 {
                list[0] = &recipes[i as usize + 1];
            }
            ingredients.push(list);
        }
        let recipes_refs: Vec<&_> = recipes.iter().map(String::as_str).collect();
        let ingredients_refs: Vec<&[&str]> = ingredients.iter().map(Vec::as_slice).collect();
        let supplies_refs: Vec<&_> = supplies.iter().map(String::as_str).collect();
        test(
            &recipes_refs,
            &ingredients_refs,
            &supplies_refs,
            &recipes_refs,
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &["ju", "fzjnm", "x", "e", "zpmcz", "h", "q"],
            &[
                &["d"],
                &["hveml", "f", "cpivl"],
                &["cpivl", "zpmcz", "h", "e", "fzjnm", "ju"],
                &["cpivl", "hveml", "zpmcz", "ju", "h"],
                &["h", "fzjnm", "e", "q", "x"],
                &["d", "hveml", "cpivl", "q", "zpmcz", "ju", "e", "x"],
                &["f", "hveml", "cpivl"],
            ],
            &["f", "hveml", "cpivl", "d"],
            &["ju", "q", "fzjnm"],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                "xevvq", "izcad", "p", "we", "bxgnm", "vpio", "i", "hjvu", "igi", "anp", "tokfq",
                "z", "kwdmb", "g", "qb", "q", "b", "hthy",
            ],
            &[
                &["wbjr"],
                &["otr", "fzr", "g"],
                &["fzr", "wi", "otr", "xgp", "wbjr", "igi", "b"],
                &[
                    "fzr", "xgp", "wi", "otr", "tokfq", "izcad", "igi", "xevvq", "i", "anp",
                ],
                &["wi", "xgp", "wbjr"],
                &["wbjr", "bxgnm", "i", "b", "hjvu", "izcad", "igi", "z", "g"],
                &["xgp", "otr", "wbjr"],
                &["wbjr", "otr"],
                &[
                    "wbjr", "otr", "fzr", "wi", "xgp", "hjvu", "tokfq", "z", "kwdmb",
                ],
                &["xgp", "wi", "wbjr", "bxgnm", "izcad", "p", "xevvq"],
                &["bxgnm"],
                &["wi", "fzr", "otr", "wbjr"],
                &["wbjr", "wi", "fzr", "xgp", "otr", "g", "b", "p"],
                &["otr", "fzr", "xgp", "wbjr"],
                &["xgp", "wbjr", "q", "vpio", "tokfq", "we"],
                &["wbjr", "wi", "xgp", "we"],
                &["wbjr"],
                &["wi"],
            ],
            &["wi", "otr", "wbjr", "fzr", "xgp"],
            &[
                "z", "xevvq", "i", "izcad", "g", "tokfq", "bxgnm", "hthy", "hjvu", "b",
            ],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &["smoothie", "juice", "salad", "fruitbowl"],
            &[
                &["juice", "banana"],
                &["orange", "water"],
                &["lettuce", "tomato"],
                &["juice", "salad"],
            ],
            &["orange", "water", "banana", "lettuce", "tomato"],
            &["smoothie", "salad", "fruitbowl", "juice"],
        )
    }
}
