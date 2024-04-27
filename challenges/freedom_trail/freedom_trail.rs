// https://leetcode.com/problems/freedom-trail/

pub struct Solution;

// Initial (incomplete) approach
// impl Solution {
//     pub fn find_rotate_steps(ring: String, key: String) -> i32 {
//         let ring = ring.as_bytes();
//         assert!(ring.len() <= 100);
//         assert!(ring.len() >= 1);
//         let key = key.as_bytes();
//         assert!(key.len() <= 100);
//         assert!(key.len() >= 1);
//         let key_len = key.len() as i8;
//         let mut dp = Vec::from_iter(
//             (0..ring.len() as i32).map(|idx| std::cmp::min(idx, ring.len() as i32 - idx)),
//         );
//         let mut key = key.into_iter();
//         let mut prev_char = *key.next().unwrap();
//         let mut final_prev_char_pos = ring.iter().rposition(|&c| c == prev_char).unwrap() as i8;
//         for &c in key {
//             let mut last_prev_char_pos = final_prev_char_pos;
//             for pos in 0..ring.len() as i8 {
//                 if ring[pos as usize] == prev_char {
//                     last_prev_char_pos = pos;
//                 } else if ring[pos as usize] == c {
//                     dp[pos as usize] = dp[final_prev_char_pos] + if pos > last_prev_char_pos {
//                         std::cmp::min(pos-last_prev_char_pos, ring.len() as i8 - pos + )
//                     }
//                 }
//             }
//             // todo!();
//             std::mem::swap(&mut this_positions, &mut last_positions);
//             this_positions.clear();
//         }
//         last_positions
//             .into_iter()
//             .map(|idx| dp[idx as usize])
//             .min()
//             .unwrap()
//             + key_len as i32
//     }
// }

// DFS sol'n
// type Steps = u16; // 101 possible positions times 100 steps = 10100 < 60000
// type Position = u8;
// type Distance = u8;
// const fn distance(start: Position, end: Position, ring_len: Position) -> Distance {
//     if start >= end {
//         let clockwise = start - end;
//         let anticlockwise = ring_len - start + end;
//         if clockwise < anticlockwise {
//             clockwise
//         } else {
//             anticlockwise
//         }
//     } else {
//         distance(end, start, ring_len)
//     }
// }
// impl Solution {
//     pub fn find_rotate_steps(ring: String, key: String) -> i32 {
//         #[derive(Debug)]
//         #[repr(align(4))] // Basically free perf on x86
//         struct SearchState {
//             next_key_pos: Position,
//             ring_pos: Position,
//             steps: u16,
//         }
//         let ring = ring.as_bytes();
//         assert!(ring.len() <= 100);
//         assert!(ring.len() >= 1);
//         let key = key.as_bytes();
//         assert!(key.len() <= 100);
//         assert!(key.len() >= 1);
//         let ring_positions = {
//             let mut ring_positions: [Vec<Position>; 26] = Default::default();
//             for i in 0..ring.len() as u8 {
//                 ring_positions[(ring[i as usize] - b'a') as usize].push(i);
//             }
//             ring_positions
//         };
//         let mut best_score_visited =
//             std::collections::HashMap::<(Position, Position), Steps>::new();
//         let mut search_stack = vec![SearchState {
//             next_key_pos: 0,
//             ring_pos: 0,
//             steps: 0,
//         }];
//         let mut best_found = u16::MAX;
//         while let Some(state) = search_stack.pop() {
//             let key_pos = state.next_key_pos;
//             match best_score_visited.entry((key_pos, state.ring_pos)) {
//                 std::collections::hash_map::Entry::Occupied(mut entry)
//                     if *entry.get() > state.steps =>
//                 {
//                     *entry.get_mut() = state.steps;
//                 }
//                 std::collections::hash_map::Entry::Vacant(entry) => {
//                     entry.insert(state.steps);
//                 }
//                 _ => continue, // We've already visited this state with a better score. Trim.
//             }
//             if key_pos >= key.len() as u8 {
//                 if state.steps < best_found {
//                     best_found = state.steps;
//                 }
//                 continue;
//             }
//             for &ring_pos in ring_positions[(key[key_pos as usize] - b'a') as usize].iter() {
//                 let steps =
//                     state.steps + distance(state.ring_pos, ring_pos, ring.len() as u8) as Steps;
//                 let mut next_key_pos = key_pos + 1;
//                 while next_key_pos < key.len() as u8
//                     && key[next_key_pos as usize] == key[key_pos as usize]
//                 {
//                     next_key_pos += 1;
//                 }
//                 search_stack.push(SearchState {
//                     next_key_pos,
//                     ring_pos,
//                     steps,
//                 })
//             }
//         }
//         key.len() as i32 + best_found as i32
//     }
// }

// DFS Sol'n improved distance calculation
// const fn distance(start: Position, end: Position, ring_len: Position) -> Distance {
//     let (smaller, larger) = if start < end {
//         (start, end)
//     } else {
//         (end, start)
//     };
//     let clockwise = larger - smaller;
//     let anticlockwise = ring_len - larger + smaller;
//     if clockwise < anticlockwise {
//         clockwise
//     } else {
//         anticlockwise
//     }
// }

// Hash map ring positions DFS Sol'n
// impl Solution {
//     pub fn find_rotate_steps(ring: String, key: String) -> i32 {
//         #[derive(Debug)]
//         #[repr(align(4))] // Basically free perf on x86
//         struct SearchState {
//             next_key_pos: Position,
//             ring_pos: Position,
//             steps: u16,
//         }
//         let ring = ring.as_bytes();
//         assert!(ring.len() <= 100);
//         assert!(ring.len() >= 1);
//         let key = key.as_bytes();
//         assert!(key.len() <= 100);
//         assert!(key.len() >= 1);
//         let ring_positions = {
//             let mut ring_positions = std::collections::HashMap::<u8, Vec<Position>>::new();
//             for i in 0..ring.len() as u8 {
//                 ring_positions.entry(ring[i as usize]).or_default().push(i);
//             }
//             ring_positions
//         };
//         let mut best_score_visited =
//             std::collections::HashMap::<(Position, Position), Steps>::new();
//         let mut search_stack = vec![SearchState {
//             next_key_pos: 0,
//             ring_pos: 0,
//             steps: 0,
//         }];
//         let mut best_found = u16::MAX;
//         while let Some(state) = search_stack.pop() {
//             let key_pos = state.next_key_pos;
//             match best_score_visited.entry((key_pos, state.ring_pos)) {
//                 std::collections::hash_map::Entry::Occupied(mut entry)
//                     if *entry.get() > state.steps =>
//                 {
//                     *entry.get_mut() = state.steps;
//                 }
//                 std::collections::hash_map::Entry::Vacant(entry) => {
//                     entry.insert(state.steps);
//                 }
//                 _ => continue, // We've already visited this state with a better score. Trim.
//             }
//             if key_pos >= key.len() as u8 {
//                 if state.steps < best_found {
//                     best_found = state.steps;
//                 }
//                 continue;
//             }
//             for &ring_pos in ring_positions[&key[key_pos as usize]].iter() {
//                 let steps =
//                     state.steps + distance(state.ring_pos, ring_pos, ring.len() as u8) as Steps;
//                 let mut next_key_pos = key_pos + 1;
//                 while next_key_pos < key.len() as u8
//                     && key[next_key_pos as usize] == key[key_pos as usize]
//                 {
//                     next_key_pos += 1;
//                 }
//                 search_stack.push(SearchState {
//                     next_key_pos,
//                     ring_pos,
//                     steps,
//                 })
//             }
//         }
//         key.len() as i32 + best_found as i32
//     }
// }

// Recursive DFS Sol'n
type Steps = u16; // 101 possible positions times 100 steps = 10100 < 60000
type Position = u8;
type Distance = u8;
#[inline]
const fn distance(start: Position, end: Position, ring_len: Position) -> Distance {
    let a = (start + ring_len - end) % ring_len;
    let b = (end + ring_len - start) % ring_len;
    if a < b {
        a
    } else {
        b
    }
}
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.as_bytes();
        assert!(ring.len() <= 100);
        assert!(ring.len() >= 1);
        let key = key.as_bytes();
        assert!(key.len() <= 100);
        assert!(key.len() >= 1);
        struct Searcher<'a> {
            key: &'a [u8],
            ring: &'a [u8],
            key_len: u8,
            ring_len: u8,
            cache: Vec<Vec<Steps>>,
        }
        impl<'a> Searcher<'a> {
            fn new(key: &'a [u8], ring: &'a [u8]) -> Self {
                Self {
                    key,
                    ring,
                    key_len: key.len() as u8,
                    ring_len: ring.len() as u8,
                    cache: vec![vec![0; key.len()]; ring.len()],
                }
            }
            fn solve(&mut self, key_pos: Position, ring_pos: Position) -> Steps {
                if key_pos >= self.key_len {
                    return 0;
                }
                assert!(ring_pos < self.ring_len);
                let cached = self.cache[ring_pos as usize][key_pos as usize];
                if cached > 0 {
                    // We'll just recompute if it's 0.
                    return cached;
                }
                let steps = self.solve_recurse(key_pos, ring_pos);
                // This is wrong only for cache entry 0,0, but we shouldn't ever need that.
                // Even if it is wrong, we'll just find whatever the correct value is on the next
                // iteration.
                self.cache[ring_pos as usize][key_pos as usize] = steps;
                steps
            }
            fn solve_recurse(&mut self, key_pos: Position, ring_pos: Position) -> Steps {
                let key_char = self.key[key_pos as usize];
                if key_char == self.ring[ring_pos as usize] {
                    return self.solve(key_pos + 1, ring_pos);
                }
                let mut steps = Steps::MAX;
                for next_ring_pos in 0..self.ring.len() as Position {
                    if self.ring[next_ring_pos as usize] != key_char {
                        continue;
                    }
                    let distance = distance(ring_pos, next_ring_pos, self.ring_len);
                    let these_steps = distance as Steps + self.solve(key_pos + 1, next_ring_pos);
                    if these_steps < steps {
                        steps = these_steps;
                    }
                }
                steps
            }
        }
        Searcher::new(key, ring).solve(0, 0) as i32 + key.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_distance(start: Position, end: Position, ring_len: Position, expected: Distance) {
        let result = distance(start, end, ring_len);
        assert_eq!(result, expected);
    }

    #[test]
    fn distance1() {
        test_distance(0, 7, 8, 1)
    }

    #[test]
    fn distance2() {
        test_distance(7, 0, 8, 1)
    }

    #[test]
    fn distance3() {
        test_distance(0, 1, 8, 1)
    }

    #[test]
    fn distance4() {
        test_distance(1, 0, 8, 1)
    }

    #[test]
    fn distance5() {
        test_distance(0, 2, 8, 2)
    }

    #[test]
    fn distance6() {
        test_distance(2, 0, 8, 2)
    }

    fn test(ring: &str, key: &str, expected: u32) {
        assert!(ring.len() <= 100);
        assert!(ring.len() >= 1);
        assert!(ring.bytes().all(|c| c >= b'a' && c <= b'z'));
        assert!(key.len() <= 100);
        assert!(key.len() >= 1);
        assert!(key.bytes().all(|c| c >= b'a' && c <= b'z'));
        let result = Solution::find_rotate_steps(ring.to_string(), key.to_string());
        assert!(result > 0);
        assert_eq!(result as u32, expected)
    }

    #[test]
    fn ex1() {
        test("godding", "gd", 4)
    }

    #[test]
    fn ex2() {
        test("godding", "godding", 13)
    }

    #[test]
    fn ex2_myvariant1() {
        test("goddin", "goddin", 11)
    }

    #[test]
    fn ex2_myvariant2() {
        test("gog", "gog", 5)
    }

    #[test]
    fn ex2_myvariant3() {
        test("go", "go", 3)
    }

    #[test]
    fn discussion_case1() {
        test("aaaaa", "aaaaa", 5)
    }

    #[test]
    fn discussion_case1_myvariant2() {
        test("aa", "aa", 2)
    }

    #[test]
    fn discussion_case1_myvariant3() {
        test("aaa", "aaa", 3)
    }

    #[test]
    fn discussion_examples_guy_case1() {
        test("y", "y", 1)
    }

    #[test]
    fn discussion_examples_guy_case2() {
        test("dccjgocjnscdalkwrlzmsrhnprvvsvqjtqnzirtaasdeldiiokttozjfkwjghhaibtzkoepdvfkhxgmxwtwsrgiryzqljpsntjei", "lsdsvjtj", 49)
    }

    #[test]
    fn discussion_case2() {
        test("iotfo", "fioot", 11)
    }

    #[test]
    fn discussion_case3() {
        test(
            "caotmcaataijjxi",
            "oatjiioicitatajtijciocjcaaxaaatmctxamacaamjjx",
            137,
        )
    }

    #[test]
    fn discussion_case4() {
        test("aaaabcaaaaaaaba", "bc", 7)
    }

    #[test]
    fn discussion_case5() {
        test("abcde", "ade", 6)
    }

    #[test]
    fn discussion_case6() {
        test("soonooowonooo", "snw", 9)
    }

    #[test]
    fn discussion_case6_myvariant1() {
        test("sonoownoo", "snw", 7)
    }

    #[test]
    fn discussion_case6_myvariant2() {
        test("snown", "snw", 5)
    }
}
