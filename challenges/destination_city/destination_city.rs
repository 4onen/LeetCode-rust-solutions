// https://leetcode.com/problems/destination-city
pub struct Solution;

// Trivial hash set sol'n
// impl Solution {
//     // What person decided on this allocation-ful interface?! Ugh.
//     pub fn dest_city(paths: Vec<Vec<String>>) -> String {
//         let mut outgoing = std::collections::HashSet::new();
//         let mut incoming = std::collections::HashSet::new();

//         for path in paths {
//             for (set, city) in
//                 Iterator::zip([&mut outgoing, &mut incoming].iter_mut(), path.into_iter())
//             {
//                 set.insert(city);
//             }
//         }

//         incoming
//             .into_iter()
//             .find(|city| !outgoing.contains(city))
//             .unwrap()
//     }
// }

// More intelligent scan
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut incoming_guesses = std::collections::HashSet::new();
        let mut outgoing = std::collections::HashSet::new();

        for path in paths {
            let mut path_iter = path.into_iter();
            let outgoing_city = path_iter.next().unwrap();
            let incoming_city = path_iter.next().unwrap();

            incoming_guesses.remove(&outgoing_city);
            if !outgoing.contains(&incoming_city) {
                incoming_guesses.insert(incoming_city);
            }
            outgoing.insert(outgoing_city);
        }

        incoming_guesses.into_iter().next().unwrap() // Should only have the 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        const PATHS: &[&[&str]] = &[
            &["London", "New York"],
            &["New York", "Lima"],
            &["Lima", "Sao Paulo"],
        ];

        assert_eq!(
            Solution::dest_city(
                PATHS
                    .iter()
                    .map(|path| path.iter().map(|s| s.to_string()).collect())
                    .collect()
            ),
            "Sao Paulo"
        );
    }

    #[test]
    fn ex2() {
        const PATHS: &[&[&str]] = &[&["B", "C"], &["D", "B"], &["C", "A"]];

        assert_eq!(
            Solution::dest_city(
                PATHS
                    .iter()
                    .map(|path| path.iter().map(|s| s.to_string()).collect())
                    .collect()
            ),
            "A"
        );
    }

    #[test]
    fn ex3() {
        const PATHS: &[&[&str]] = &[&["A", "Z"]];

        assert_eq!(
            Solution::dest_city(
                PATHS
                    .iter()
                    .map(|path| path.iter().map(|s| s.to_string()).collect())
                    .collect()
            ),
            "Z"
        );
    }
}
