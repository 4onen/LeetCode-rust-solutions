// https://leetcode.com/problems/open-the-lock/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
//         #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
//         #[repr(align(4))]
//         struct LockState {
//             wheels: [u8; 4],
//         }
//         impl LockState {
//             const fn from_wheels(wheels: [u8; 4]) -> Self {
//                 LockState { wheels }
//             }
//             #[inline]
//             const fn next_states(&self) -> [Self; 8] {
//                 [
//                     LockState::from_wheels([
//                         self.wheels[0],
//                         self.wheels[1],
//                         self.wheels[2],
//                         self.wheels[3].wrapping_add(9) % 10, // -1
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0],
//                         self.wheels[1],
//                         self.wheels[2],
//                         self.wheels[3].wrapping_add(1) % 10,
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0],
//                         self.wheels[1],
//                         self.wheels[2].wrapping_add(9) % 10, // -1
//                         self.wheels[3],
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0],
//                         self.wheels[1],
//                         self.wheels[2].wrapping_add(1) % 10,
//                         self.wheels[3],
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0],
//                         self.wheels[1].wrapping_add(9) % 10, // -1
//                         self.wheels[2],
//                         self.wheels[3],
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0],
//                         self.wheels[1].wrapping_add(1) % 10,
//                         self.wheels[2],
//                         self.wheels[3],
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0].wrapping_add(9) % 10, // -1
//                         self.wheels[1],
//                         self.wheels[2],
//                         self.wheels[3],
//                     ]),
//                     LockState::from_wheels([
//                         self.wheels[0].wrapping_add(1) % 10,
//                         self.wheels[1],
//                         self.wheels[2],
//                         self.wheels[3],
//                     ]),
//                 ]
//             }
//         }
//         impl std::str::FromStr for LockState {
//             type Err = ();
//             fn from_str(s: &str) -> Result<Self, Self::Err> {
//                 let s = s.as_bytes();
//                 if s.len() < 4 {
//                     Err(())
//                 } else {
//                     Ok(Self::from_wheels([
//                         s[0] - b'0',
//                         s[1] - b'0',
//                         s[2] - b'0',
//                         s[3] - b'0',
//                     ]))
//                 }
//             }
//         }
//         /// Consumes a vector of strings to
//         /// convert them into a vector of LockStates
//         ///
//         /// Panics if any string is too short
//         /// to convert to a LockState, or if any char is not in the range '0'..='9'
//         unsafe fn lockstate_vec_from_string_vec(i: Vec<String>) -> Vec<LockState> {
//             let len = i.len();
//             let p_strings = i.leak();
//             let p_locks =
//                 std::slice::from_raw_parts_mut(p_strings.as_mut_ptr() as *mut LockState, len);
//             for (i, p_string) in p_strings[..len].into_iter().enumerate() {
//                 let lock: LockState = p_string
//                     .parse()
//                     .expect("Failed to unwrap LockState while converting from string vec.");
//                 p_locks[i] = lock;
//             }
//             std::vec::Vec::from_raw_parts(
//                 p_locks.as_mut_ptr(),
//                 len,
//                 p_strings.len() * std::mem::size_of::<String>() / std::mem::size_of::<LockState>(),
//             )
//         }
//         let target: LockState = target.parse().unwrap();
//         let deadends = unsafe { lockstate_vec_from_string_vec(deadends) };
//         const ZERO_LOCK: LockState = LockState::from_wheels([0, 0, 0, 0]);
//         let mut steps = 0;
//         let mut visited = std::collections::HashSet::new();
//         let mut curr = std::collections::HashSet::new();
//         curr.insert(ZERO_LOCK);
//         let mut next = std::collections::HashSet::new();
//         loop {
//             if curr.is_empty() {
//                 break -1;
//             }
//             if curr.contains(&target) {
//                 break steps;
//             }
//             for lockstate in curr.drain() {
//                 if deadends.contains(&lockstate) {
//                     continue;
//                 }
//                 if !visited.insert(lockstate) {
//                     continue;
//                 }
//                 for nextstate in lockstate.next_states() {
//                     next.insert(nextstate);
//                 }
//             }
//             steps += 1;
//             std::mem::swap(&mut curr, &mut next);
//         }
//     }
// }

// Visited+deadend merge sol'n
// impl Solution {
//     pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
//         type LockState = u16;
//         const fn next_states(lockstate: u16) -> [u16; 8] {
//             let mut out = [0; 8];
//             let mut out_idx = 0u8;
//             let mut digit_sel = 1;
//             while digit_sel < 10_000 {
//                 let digit = (lockstate / digit_sel) % 10;
//                 out[out_idx as usize] = if digit == 0 {
//                     lockstate + digit_sel * 9
//                 } else {
//                     lockstate - digit_sel
//                 };
//                 out[(out_idx + 1) as usize] = if digit == 9 {
//                     lockstate - digit_sel * 9
//                 } else {
//                     lockstate + digit_sel
//                 };
//                 out_idx += 2;
//                 digit_sel *= 10;
//             }
//             out
//         }
//         type StepCount = i32;
//         const NOT_VISITED: StepCount = 10_000;
//         const DEAD_END: StepCount = -1;
//         const STARTING_STATE: LockState = 0;
//         let mut visited: Vec<StepCount> = vec![NOT_VISITED; 10_000];
//         let target: u16 = target.parse().unwrap();
//         for deadend in deadends {
//             let deadend: LockState = deadend.parse().unwrap();
//             if deadend == STARTING_STATE {
//                 return DEAD_END as i32;
//             }
//             visited[deadend as usize] = DEAD_END;
//         }
//         let mut queue = std::collections::vec_deque::VecDeque::<u16>::new();
//         queue.push_back(STARTING_STATE);
//         visited[STARTING_STATE as usize] = 0;
//         while let Some(lockstate) = queue.pop_front() {
//             let dist = visited[lockstate as usize];
//             for next_state in next_states(lockstate) {
//                 if visited[next_state as usize] == NOT_VISITED {
//                     queue.push_back(next_state);
//                     visited[next_state as usize] = dist + 1;
//                 }
//             }
//             if visited[target as usize] != NOT_VISITED {
//                 return visited[target as usize];
//             }
//         }
//         -1
//     }
// }

// Visited+deadend merge sol'n with bool visited arr
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        type LockState = u16;
        const fn next_states(lockstate: u16) -> [u16; 8] {
            let mut out = [0; 8];
            let mut out_idx = 0u8;
            let mut digit_sel = 1;
            while digit_sel < 10_000 {
                let digit = (lockstate / digit_sel) % 10;
                out[out_idx as usize] = if digit == 0 {
                    lockstate + digit_sel * 9
                } else {
                    lockstate - digit_sel
                };
                out[(out_idx + 1) as usize] = if digit == 9 {
                    lockstate - digit_sel * 9
                } else {
                    lockstate + digit_sel
                };
                out_idx += 2;
                digit_sel *= 10;
            }
            out
        }
        const STARTING_STATE: LockState = 0;
        let mut visited = vec![false; 10_000];
        let target: u16 = target.parse().unwrap();
        for deadend in deadends {
            let deadend: LockState = deadend.parse().unwrap();
            if deadend == STARTING_STATE {
                return -1;
            }
            visited[deadend as usize] = true;
        }
        let mut steps = 0;
        let mut queue = std::collections::vec_deque::VecDeque::<u16>::new();
        queue.push_back(STARTING_STATE);
        visited[STARTING_STATE as usize] = true;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let lockstate = queue.pop_front().unwrap();
                if lockstate == target {
                    return steps;
                }
                for next_state in next_states(lockstate) {
                    if visited[next_state as usize] == false {
                        queue.push_back(next_state);
                        visited[next_state as usize] = true;
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(deadends: &[&str], target: &str, expected: i32) {
        let deadends = deadends.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::open_lock(deadends, target.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test(&["0201", "0101", "0102", "1212", "2002"], "0202", 6);
    }

    #[test]
    fn ex2() {
        test(&["8888"], "0009", 1);
    }

    #[test]
    fn ex3() {
        test(
            &[
                "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
            ],
            "8888",
            -1,
        );
    }

    #[test]
    fn discussion_case0() {
        test(&["0000"], "8888", -1);
    }

    #[test]
    fn myex1() {
        test(&[], "5555", 20);
    }
}
