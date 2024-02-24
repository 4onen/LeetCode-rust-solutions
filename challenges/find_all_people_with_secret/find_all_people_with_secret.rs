// https://leetcode.com/problems/find-all-people-with-secret/

// You are given an integer n indicating there are n people numbered from 0 to n - 1. You are also given a 0-indexed 2D integer array meetings where meetings[i] = [xi, yi, timei] indicates that person xi and person yi have a meeting at timei. A person may attend multiple meetings at the same time. Finally, you are given an integer firstPerson.
// Person 0 has a secret and initially shares the secret with a person firstPerson at time 0. This secret is then shared every time a meeting takes place with a person that has the secret. More formally, for every meeting, if a person xi has the secret at timei, then they will share the secret with person yi, and vice versa.
// The secrets are shared instantaneously. That is, a person may receive the secret and share it with people in other meetings within the same time frame.
// Return a list of all the people that have the secret after all the meetings have taken place. You may return the answer in any order.

// Example 1:

// Input: n = 6, meetings = [[1,2,5],[2,3,8],[1,5,10]], firstPerson = 1
// Output: [0,1,2,3,5]
// Explanation:
// At time 0, person 0 shares the secret with person 1.
// At time 5, person 1 shares the secret with person 2.
// At time 8, person 2 shares the secret with person 3.
// At time 10, person 1 shares the secret with person 5.​​​​
// Thus, people 0, 1, 2, 3, and 5 know the secret after all the meetings.

// Example 2:

// Input: n = 4, meetings = [[3,1,3],[1,2,2],[0,3,3]], firstPerson = 3
// Output: [0,1,3]
// Explanation:
// At time 0, person 0 shares the secret with person 3.
// At time 2, neither person 1 nor person 2 know the secret.
// At time 3, person 3 shares the secret with person 0 and person 1.
// Thus, people 0, 1, and 3 know the secret after all the meetings.

// Example 3:

// Input: n = 5, meetings = [[3,4,2],[1,2,1],[2,3,1]], firstPerson = 1
// Output: [0,1,2,3,4]
// Explanation:
// At time 0, person 0 shares the secret with person 1.
// At time 1, person 1 shares the secret with person 2, and person 2 shares the secret with person 3.
// Note that person 2 can share the secret at the same time as receiving it.
// At time 2, person 3 shares the secret with person 4.
// Thus, people 0, 1, 2, 3, and 4 know the secret after all the meetings.

// Constraints:
//     2 <= n <= 10^5
//     1 <= meetings.length <= 10^5
//     meetings[i].length == 3
//     0 <= xi, yi <= n - 1
//     xi != yi
//     1 <= timei <= 10^5
//     1 <= firstPerson <= n - 1

pub struct Solution;

// Time-incremental hash table sol'n (TLE)
// impl Solution {
//     pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
//         assert!(n >= 2);
//         let n = n as u32;
//         assert!(n <= 100_000);
//         assert!(meetings.len() > 0);
//         assert!(meetings.len() <= 100_000);
//         assert!(first_person >= 1);
//         assert!((first_person as u32) < n);
//         let mut meetings = meetings
//             .into_iter()
//             .map(|info| {
//                 let x_i = info[0];
//                 let y_i = info[1];
//                 let time_i = info[2] as u32;
//                 (time_i, x_i, y_i)
//             })
//             .collect::<Vec<_>>();
//         meetings.sort_unstable();
//         let mut know = vec![0, first_person];
//         let mut tstart = meetings[0].0;
//         let mut meetings = meetings.into_iter().peekable();
//         while tstart < u32::MAX {
//             let mut spreads_to: std::collections::HashMap<_, Vec<_>> =
//                 std::collections::HashMap::new();
//             tstart = loop {
//                 let Some(&(time_i, _, _)) = meetings.peek() else {
//                     break u32::MAX;
//                 };
//                 if tstart < time_i {
//                     break time_i;
//                 }
//                 let (_, x_i, y_i) = meetings.next().unwrap();
//                 spreads_to.entry(x_i).or_default().push(y_i);
//                 spreads_to.entry(y_i).or_default().push(x_i);
//             };
//             dbg!(tstart);
//             let mut spread_this_time = know.clone();
//             while let Some(person) = spread_this_time.pop() {
//                 for &p in spreads_to.get(&person).into_iter().flatten() {
//                     if !know.contains(&p) {
//                         know.push(p);
//                         spread_this_time.push(p);
//                     }
//                 }
//             }
//         }
//         know
//     }
// }

// Time-incremental hash table sol'n with sets and accelerated lookup (TLE, but local this time)
// impl Solution {
//     pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
//         assert!(n >= 2);
//         let n = n as u32;
//         assert!(n <= 100_000);
//         assert!(meetings.len() > 0);
//         assert!(meetings.len() <= 100_000);
//         assert!(first_person >= 1);
//         assert!((first_person as u32) < n);
//         let mut meetings = meetings
//             .into_iter()
//             .map(|info| {
//                 let x_i = info[0];
//                 let y_i = info[1];
//                 let time_i = info[2] as u32;
//                 let lesser_person = std::cmp::min(x_i, y_i);
//                 let greater_person = std::cmp::max(x_i, y_i);
//                 (time_i, lesser_person, greater_person)
//             })
//             .collect::<Vec<_>>();
//         meetings.sort_unstable();
//         let mut know = vec![false; n as usize];
//         know[0] = true;
//         know[first_person as usize] = true;
//         let mut know_set = vec![0, first_person];
//         let mut tstart = meetings[0].0;
//         let mut meetings = meetings.into_iter().peekable();
//         while tstart < u32::MAX {
//             let mut spreads_to: std::collections::HashMap<_, Vec<_>> =
//                 std::collections::HashMap::new();
//             tstart = loop {
//                 let Some(&(time_i, _, _)) = meetings.peek() else {
//                     break u32::MAX;
//                 };
//                 if tstart < time_i {
//                     break time_i;
//                 }
//                 let (_, lesser_i, greater_i) = meetings.next().unwrap();
//                 spreads_to.entry(lesser_i).or_default().push(greater_i);
//                 spreads_to.entry(greater_i).or_default().push(lesser_i);
//             };
//             dbg!(tstart);
//             let mut spread_this_time = know_set.clone();
//             while let Some(person) = spread_this_time.pop() {
//                 for &p in spreads_to.get(&person).into_iter().flatten() {
//                     if !know[p as usize] {
//                         know[p as usize] = true;
//                         know_set.push(p);
//                         spread_this_time.push(p);
//                     }
//                 }
//             }
//         }
//         know_set
//     }
// }

// Union-find sol'n
impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        type UnionFindItem = u32;
        struct UnionFind {
            parent: Vec<UnionFindItem>,
            // No rank -- we only merge with people in the same set as 0
        }
        impl UnionFind {
            fn new(n: u32) -> Self {
                Self {
                    parent: (0..n).collect(),
                }
            }
            fn len(&self) -> usize {
                self.parent.len()
            }
            fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
                while self.parent[x as usize] != x {
                    self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
                    x = self.parent[x as usize];
                }
                x
            }
            fn union(&mut self, x: UnionFindItem, y: UnionFindItem) -> bool {
                let x_root = self.find(x);
                let y_root = self.find(y);
                if x_root == y_root {
                    return y_root == 0;
                }
                let lesser_root = std::cmp::min(x_root, y_root);
                self.parent[x_root as usize] = lesser_root;
                self.parent[y_root as usize] = lesser_root;
                lesser_root == 0
            }
            fn reset_if_nonzero(&mut self, x: UnionFindItem) {
                let x_root = self.find(x);
                self.parent[x as usize] = if x_root == 0 { 0 } else { x };
            }
            fn into_zero_set(mut self) -> Vec<UnionFindItem> {
                for i in 0..self.len() {
                    self.parent[i] = self.find(i as UnionFindItem);
                }
                self.parent
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, p)| {
                        if p == 0 {
                            Some(i as UnionFindItem)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        }
        let meetings = {
            meetings.sort_unstable_by_key(|info| info[2]);
            meetings
        };
        assert!(n >= 2);
        let n = n as u32;
        assert!(n <= 100_000);
        assert!(meetings.len() > 0);
        assert!(meetings.len() <= 100_000);
        assert!(first_person >= 1);
        let first_person = first_person as u32;
        assert!((first_person as u32) < n);
        let mut uf = UnionFind::new(n);
        uf.union(0, first_person);
        let mut resets = Vec::new();
        let mut tstart = 0;
        for info in meetings {
            let x_i = info[0] as u32;
            let y_i = info[1] as u32;
            let time_i = info[2];
            if time_i > tstart {
                for r in resets.drain(..) {
                    uf.reset_if_nonzero(r);
                }
                tstart = time_i;
            }
            if !uf.union(x_i, y_i) {
                resets.push(x_i);
                resets.push(y_i);
            }
        }
        // Safety: I'm too lazy to map this and we know all the elements are
        // less than n, which is less than 100_000.
        // Using u32 saves us usize conversion overhead.
        unsafe { std::mem::transmute(uf.into_zero_set()) }
    }
}

// Tupleized union-find sol'n
// impl Solution {
//     pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
//         type UnionFindItem = u32;
//         struct UnionFind {
//             parent: Vec<UnionFindItem>,
//             // No rank -- we only merge with people in the same set as 0
//         }
//         impl UnionFind {
//             fn new(n: u32) -> Self {
//                 Self {
//                     parent: (0..n).collect(),
//                 }
//             }
//             fn len(&self) -> usize {
//                 self.parent.len()
//             }
//             fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
//                 while self.parent[x as usize] != x {
//                     self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
//                     x = self.parent[x as usize];
//                 }
//                 x
//             }
//             fn union(&mut self, x: UnionFindItem, y: UnionFindItem) -> bool {
//                 let x_root = self.find(x);
//                 let y_root = self.find(y);
//                 if x_root == y_root {
//                     y_root == 0
//                 } else if x_root == 0 {
//                     self.parent[y_root as usize] = 0;
//                     true
//                 } else {
//                     self.parent[x_root as usize] = y_root;
//                     y_root == 0
//                 }
//             }
//             fn reset_if_nonzero(&mut self, x: UnionFindItem) {
//                 let x_root = self.find(x);
//                 self.parent[x as usize] = if x_root == 0 { 0 } else { x };
//             }
//             fn into_zero_set(mut self) -> Vec<UnionFindItem> {
//                 for i in 0..self.len() {
//                     self.parent[i] = self.find(i as UnionFindItem);
//                 }
//                 self.parent
//                     .into_iter()
//                     .enumerate()
//                     .filter_map(|(i, p)| {
//                         if p == 0 {
//                             Some(i as UnionFindItem)
//                         } else {
//                             None
//                         }
//                     })
//                     .collect()
//             }
//         }
//         type MeetingTup = (u32, u32, std::num::NonZeroU32);
//         fn tupleize_flights(trusts: Vec<Vec<i32>>) -> Box<[MeetingTup]> {
//             // First, convert this vector of vectors to a more useful vector
//             // of tuples. We know each subvector has exactly 3 small eles, so
//             // we can stuff them into tuples in the same memory space as the
//             // original sub-vectors.
//             // Obviously super unsafe if run on a 16-bit system or something,
//             // but this is a toy problem on leetcode, so the environment is
//             // known.
//             let original_len = trusts.len();
//             let meeting_ptr = trusts.leak();
//             let meeting_ptr2: &mut [Vec<i32>] = &mut meeting_ptr[0..original_len];
//             let meeting_tuple_ptr =
//                 unsafe { std::mem::transmute::<_, &mut [MeetingTup]>(meeting_ptr2) };
//             for i in 0..original_len {
//                 // We know all the times are 1 to 100000, so we can safely
//                 // convert them to NonZeroU32s.
//                 // We know all the meeting members are 0 to 100000, so we
//                 // can safely convert them to u32s.
//                 let tup = unsafe {
//                     (
//                         std::mem::transmute(meeting_ptr[i][0]),
//                         std::mem::transmute(meeting_ptr[i][1]),
//                         std::num::NonZeroU32::new_unchecked(meeting_ptr[i][2] as u32),
//                     )
//                 };
//                 meeting_tuple_ptr[i] = tup;
//             }
//             unsafe { std::boxed::Box::from_raw(meeting_tuple_ptr) }
//         }
//         let meetings = {
//             let mut meetings = tupleize_flights(meetings);
//             meetings.sort_unstable_by_key(|&(_, _, time_i)| time_i);
//             meetings
//         };
//         assert!(n >= 2);
//         let n = n as u32;
//         assert!(n <= 100_000);
//         assert!(meetings.len() > 0);
//         assert!(meetings.len() <= 100_000);
//         assert!(first_person >= 1);
//         let first_person = first_person as u32;
//         assert!((first_person as u32) < n);
//         let mut uf = UnionFind::new(n);
//         uf.union(0, first_person);
//         let mut resets = Vec::new();
//         let mut tstart = meetings[0].2;
//         for &(x_i, y_i, time_i) in meetings.into_iter() {
//             if time_i > tstart {
//                 for r in resets.drain(..) {
//                     uf.reset_if_nonzero(r);
//                 }
//                 tstart = time_i;
//             }
//             if !uf.union(x_i, y_i) {
//                 resets.push(x_i);
//                 resets.push(y_i);
//             }
//         }
//         // Safety: I'm too lazy to map this and we know all the elements are
//         // less than n, which is less than 100_000.
//         // Using u32 saves us usize conversion overhead.
//         unsafe { std::mem::transmute(uf.into_zero_set()) }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let n = 6;
        let meetings = vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]];
        let first_person = 1;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(result, [0, 1, 2, 3, 5]);
    }

    #[test]
    fn ex2() {
        let n = 4;
        let meetings = vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]];
        let first_person = 3;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(result, [0, 1, 3]);
    }

    #[test]
    fn ex3() {
        let n = 5;
        let meetings = vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]];
        let first_person = 1;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(result, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn time_limit_exceeded_case1() {
        let n = 50000;
        let meetings = include_str!("tle_case1.txt")
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                let time = parts.next().unwrap().parse().unwrap();
                vec![x, y, time]
            })
            .collect();
        let first_person = 1;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(result, (0..n).collect::<Vec<_>>());
    }

    #[test]
    fn failing_case1() {
        let n = 5;
        let meetings = vec![vec![1, 4, 3], vec![0, 4, 3]];
        let first_person = 3;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(result, [0, 1, 3, 4]);
    }

    #[test]
    fn discussion_case1() {
        // tc 32/55
        let n = 294;
        let meetings = include_str!("discussion_case1.txt")
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                let time = parts.next().unwrap().parse().unwrap();
                vec![x, y, time]
            })
            .collect();
        let first_person = 27;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(
            result,
            [
                0, 4, 11, 13, 22, 24, 27, 32, 37, 38, 42, 55, 59, 63, 69, 70, 71, 73, 79, 85, 87,
                88, 102, 106, 112, 124, 128, 139, 149, 156, 159, 168, 175, 183, 184, 189, 190, 192,
                193, 194, 199, 201, 205, 210, 211, 214, 215, 218, 222, 225, 230, 232, 234, 244,
                249, 250, 255, 259, 261, 264, 269, 271, 283, 285, 287, 288
            ]
        );
    }

    #[test]
    fn discussion_case2() {
        // tc 54/55
        let n = 9;
        let meetings = [
            [8, 7, 1],
            [6, 3, 1],
            [2, 1, 1],
            [0, 1, 1],
            [3, 2, 1],
            [7, 6, 1],
            [4, 2, 2],
        ];
        let meetings = meetings.into_iter().map(Vec::from).collect();
        let first_person = 5;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(result, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
