// https://leetcode.com/problems/letter-tile-possibilities/

pub struct Solution;

// Naive sol'n
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        type Letters = [u8;26];
        fn backtrack(letters: &mut Letters, path: &mut Vec<u8>, seen_paths: &mut std::collections::HashSet<Vec<u8>>) {
            if !seen_paths.contains(path) {
                seen_paths.insert(path.clone());
            }
            for i in 0..letters.len() as u8 {
                if letters[i as usize] > 0 {
                    letters[i as usize] -= 1;
                    path.push(i);
                    backtrack(letters, path, seen_paths);
                    path.pop();
                    letters[i as usize] += 1;
                }
            }
        }
        let mut letters = [0u8;26];
        for b in tiles.as_bytes() {
            letters[(b - b'A') as usize] += 1;
        }
        let mut path = std::vec::Vec::new();
        let mut seen_paths = std::collections::HashSet::new();
        backtrack(&mut letters, &mut path, &mut seen_paths);
        seen_paths.len() as i32 - 1
    }
}

// Small string optimization (Unbelievably slower for some reason on LC)
// impl Solution {
//     pub fn num_tile_possibilities(tiles: String) -> i32 {
//         type Letters = [u8;26];
//         type S = [u8;8];
//         fn backtrack(
//             letters: &mut Letters,
//             i: u8,
//             path: &mut S,
//             seen_paths: &mut std::collections::HashSet<S>
//         ) {
//             if !seen_paths.contains(path) {
//                 seen_paths.insert(path.clone());
//             }
//             for letter in 0..letters.len() as u8 {
//                 if letters[letter as usize] > 0 {
//                     letters[letter as usize] -= 1;
//                     path[i as usize] = letter;
//                     backtrack(letters, i+1, path, seen_paths);
//                     path[i as usize] = u8::MAX;
//                     letters[letter as usize] += 1;
//                 }
//             }
//         }
//         let mut letters = [0u8;26];
//         for b in tiles.as_bytes() {
//             letters[(b - b'A') as usize] += 1;
//         }
//         let mut path = [u8::MAX;8];
//         let mut seen_paths = std::collections::HashSet::new();
//         backtrack(&mut letters, 0, &mut path, &mut seen_paths);
//         dbg!(&seen_paths);
//         seen_paths.len() as i32 - 1
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tiles: &str, expected: i32) {
        assert!(tiles.len() >= 1);
        assert!(tiles.len() <= 7);
        for &b in tiles.as_bytes() {
            assert!(b >= b'A');
            assert!(b <= b'Z');
        }
        assert_eq!(Solution::num_tile_possibilities(tiles.to_owned()), expected, "Testing string {}", tiles);
    }

    #[test]
    fn ex1() {
        test("AAB",8)
    }

    #[test]
    fn ex2() {
        test("AAABBC", 188)
    }

    #[test]
    fn ex3() {
        test("V", 1)
    }

    #[test]
    fn discussion_case1() {
        // Special case when all letters are distinct.
        let s = &[b'A',b'B',b'C',b'D',b'E',b'F',b'G'];
        let results = &[
            1,
            1+1+2,
            1+1+1+2+2+2+6,
            4*1+6*2+4*6+1*24,
            325,
            1956,
            13699,
        ];
        for i in 1..s.len() {
            test(unsafe {std::str::from_utf8_unchecked(&s[..=i as usize])},
            results[i])
        }
    }

    #[test]
    fn discussion_case2() {
        test("DACBAC",522)
    }
}
