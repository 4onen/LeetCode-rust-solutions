// https://leetcode.com/problems/course-schedule/

pub struct Solution;

// Naive brute force
// impl Solution {
//     pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
//         let mut num_taken = 0;
//         let mut taken = vec![false; num_courses as usize];
//         let mut prereqs = vec![vec![]; num_courses as usize];
//         for p in prerequisites {
//             let a = p[0];
//             let b = p[1];
//             prereqs[a as usize].push(b);
//         }
//         loop {
//             let mut changed = false;
//             for c in 0..num_courses {
//                 if taken[c as usize] {
//                     continue;
//                 }
//                 if prereqs[c as usize].iter().all(|&p| taken[p as usize]) {
//                     taken[c as usize] = true;
//                     num_taken += 1;
//                     changed = true;
//                 }
//             }
//             if !changed || num_taken == num_courses {
//                 break;
//             }
//         }
//         num_taken == num_courses
//     }
// }

// Hash tables and a horrifying DFS hack.
// impl Solution {
//     pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
//         type Item = u16;
//         let num_courses = num_courses as u16;
//         let mut num_taken = num_courses;
//         let mut taken = vec![true; num_courses as usize];
//         let mut prereqs = std::collections::HashMap::new();
//         for p in prerequisites {
//             let a = p[0] as Item;
//             let b = p[1] as Item;
//             prereqs.entry(a as Item).or_insert(vec![]).push(b);
//             num_taken -= taken[a as usize] as u16;
//             taken[a as usize] = false;
//         }
//         let mut stack = std::vec::Vec::with_capacity(num_courses as usize);
//         for c in 0..num_courses {
//             if taken[c as usize] {
//                 continue;
//             }
//             stack.push(c as Item);
//             // If the stack hits a node twice, then we have a cycle.
//             // This is caught because when the stack unwinds, the
//             // node is counted in num_taken twice, causing the value
//             // to overshoot the number of courses at the end of the
//             // function.
//             while let Some(&c) = stack.last() {
//                 if prereqs.get(&c).map(std::vec::Vec::len).unwrap_or(0) > 0 {
//                     let p = prereqs.get_mut(&c).unwrap();
//                     while let Some(c) = p.pop() {
//                         if taken[c as usize] {
//                             continue;
//                         }
//                         stack.push(c as Item);
//                         break;
//                     }
//                 } else {
//                     taken[c as usize] = true;
//                     num_taken += 1;
//                     stack.pop();
//                 }
//             }
//         }
//         num_taken == num_courses
//     }
// }

// In-degree tracking with vecs (no more hashing)
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        type Item = u16;
        let num_courses = num_courses as Item;
        let mut num_taken = 0;
        let mut prereqs = vec![vec![]; num_courses as usize];
        let mut incoming = vec![0; num_courses as usize];
        for p in &prerequisites {
            let a = p[0] as Item;
            let b = p[1] as Item;
            prereqs[a as usize].push(b);
            incoming[b as usize] += 1;
        }
        let mut stack = (0..num_courses)
            .filter(|&c| incoming[c as usize] <= 0)
            .collect::<Vec<_>>();
        while let Some(c) = stack.pop() {
            num_taken += 1;
            for &n in &prereqs[c as usize] {
                incoming[n as usize] -= 1;
                if incoming[n as usize] == 0 {
                    stack.push(n);
                }
            }
        }
        num_taken == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num_courses: i32, prerequisites: &[[i32; 2]], expected: bool) {
        assert!(num_courses >= 1);
        assert!(num_courses <= 2000);
        assert!(prerequisites.len() <= 5000);
        let mut seen = std::collections::HashSet::new();
        for &p in prerequisites.iter() {
            assert!(p[0] >= 0);
            assert!(p[0] < num_courses);
            assert!(p[1] >= 0);
            assert!(p[1] < num_courses);
            assert!(seen.insert(p));
        }
        assert_eq!(
            Solution::can_finish(
                num_courses,
                prerequisites.iter().map(|&x| x.to_vec()).collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(2, &[[1, 0]], true)
    }

    #[test]
    fn ex1_3() {
        test(3, &[[1, 0]], true)
    }

    #[test]
    fn ex2() {
        test(2, &[[1, 0], [0, 1]], false)
    }

    #[test]
    fn discussion_case1() {
        test(5, &[[1, 4], [2, 4], [3, 1], [3, 2]], true)
    }

    #[test]
    fn discussion_case2() {
        test(
            6,
            &[[1, 0], [1, 2], [3, 1], [3, 2], [2, 4], [4, 5], [2, 5]],
            true,
        )
    }

    #[test]
    fn discussion_case3() {
        test(5, &[[1, 4], [2, 4], [3, 1], [3, 2]], true)
    }

    #[test]
    fn discussion_case4() {
        test(1, &[], true)
    }

    #[test]
    fn my_extreme_ex1() {
        // Generate 100 classes, have each class depend on all previous classes.
        let mut prerequisites = Vec::with_capacity(5000);
        for i in 0..100 {
            for j in 0..i {
                prerequisites.push([i as i32, j as i32]);
            }
        }
        test(100, &prerequisites, true)
    }

    #[test]
    fn my_extreme_ex1_1() {
        // Generate 100 classes, have each class depend on all previous classes.
        let mut prerequisites = Vec::with_capacity(5000);
        for i in 0..100 {
            for j in 0..i {
                prerequisites.push([i, j]);
            }
        }
        prerequisites.push([0, 100 - 1]);
        test(100, &prerequisites, false)
    }

    #[test]
    fn my_extreme_ex2() {
        // Generate 2000 classes, have each class depend on the two previous.
        let mut prerequisites = Vec::with_capacity(4000);
        for i in 2..2000 {
            prerequisites.push([i, i - 1]);
            prerequisites.push([i, i - 2]);
        }
        test(2000, &prerequisites, true)
    }

    #[test]
    fn my_extreme_ex2_2() {
        // Generate 2000 classes, have each class depend on the two previous.
        let mut prerequisites = Vec::with_capacity(4000);
        for i in 2..2000 {
            prerequisites.push([i, i - 1]);
            prerequisites.push([i, i - 2]);
        }
        prerequisites.push([0, 2000 - 1]);
        test(2000, &prerequisites, false)
    }
}
