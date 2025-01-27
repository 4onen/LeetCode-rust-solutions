// https://leetcode.com/problems/course-schedule-iv/

pub struct Solution;

// Initial sol'n (forgets transitive prereqs can be added later)
// impl Solution {
//     pub fn check_if_prerequisite(
//         num_courses: i32,
//         prerequisites: Vec<Vec<i32>>,
//         queries: Vec<Vec<i32>>,
//     ) -> Vec<bool> {
//         let mut set = vec![0u128; num_courses as usize];
//         for prereq_spec in prerequisites {
//             let (a, b) = (prereq_spec[0], prereq_spec[1]);
//             set[b as usize] |= (1 << a) | set[a as usize];
//         }
//         queries
//             .into_iter()
//             .map(|x| {
//                 let (u, v) = (x[0], x[1]);
//                 set[v as usize] & (1 << u) > 0
//             })
//             .collect()
//     }
// }

// Lazy repetition sol'n
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut set = vec![0u128; num_courses as usize];
        loop {
            let mut updated = false;
            for prereq_spec in &prerequisites {
                let (a, b) = (prereq_spec[0], prereq_spec[1]);
                let old = set[b as usize];
                let new = old | (1 << a) | set[a as usize];
                updated = updated || old != new;
                set[b as usize] = new
            }
            if !updated {
                break;
            }
        }
        queries
            .into_iter()
            .map(|x| {
                let (u, v) = (x[0], x[1]);
                set[v as usize] & (1 << u) > 0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num_courses: u8, prerequisites: &[[u8; 2]], queries: &[[u8; 2]], expected: &[bool]) {
        assert!(num_courses >= 2);
        assert!(num_courses <= 100);
        assert!(prerequisites.len() <= (num_courses as usize * (num_courses as usize - 1)) / 2);
        let mut seen = std::collections::HashSet::new();
        for &prereq in prerequisites {
            for class in prereq {
                assert!(class <= num_courses);
            }
            assert_ne!(prereq[0], prereq[1]);
            assert!(seen.insert(prereq))
        }
        for &query in queries {
            for class in query {
                assert!(class <= num_courses);
            }
            assert_ne!(query[0], query[1]);
        }
        assert_eq!(
            Solution::check_if_prerequisite(
                num_courses as i32,
                prerequisites
                    .iter()
                    .map(|&x| x.map(|x| x as i32).to_vec())
                    .collect(),
                queries
                    .iter()
                    .map(|&x| x.map(|x| x as i32).to_vec())
                    .collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(2, &[[1, 0]], &[[0, 1], [1, 0]], &[false, true])
    }

    #[test]
    fn ex2() {
        test(2, &[], &[[1, 0], [0, 1]], &[false, false])
    }

    #[test]
    fn ex3() {
        test(
            3,
            &[[1, 2], [1, 0], [2, 0]],
            &[[1, 0], [1, 2]],
            &[true, true],
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            4,
            &[[2, 3], [2, 1], [0, 3], [0, 1]],
            &[
                [0, 1],
                [0, 2],
                [0, 3],
                [1, 0],
                [1, 2],
                [1, 3],
                [2, 0],
                [2, 1],
                [2, 3],
                [3, 0],
                [3, 1],
                [3, 2],
            ],
            &[
                true, false, true, false, false, false, false, true, true, false, false, false,
            ],
        )
    }

    #[test]
    fn failing_case1() {
        test(
            5,
            &[[3, 4], [2, 3], [1, 2], [0, 1]],
            &[[0, 4], [4, 0], [1, 3], [3, 0]],
            &[true, false, true, false],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let mut prereqs = vec![];
        for i in 0..99 {
            prereqs.push([i, i + 1]);
        }
        prereqs.reverse();
        test(100, &prereqs, &[[99, 0], [0, 99]], &[false, true])
    }
}
