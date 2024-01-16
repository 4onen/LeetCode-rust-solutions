// https://leetcode.com/problems/path-crossing/

pub struct Solution;

// Store every position sol'n
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut pos = (0i16, 0i16);
        for b in path.as_bytes() {
            visited.insert(pos);
            match b {
                b'N' => pos.1 += 1,
                b'S' => pos.1 -= 1,
                b'E' => pos.0 += 1,
                b'W' => pos.0 -= 1,
                _ => unreachable!("Invalid input"),
            }
            if visited.contains(&pos) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        const INPUT: &str = "NES";
        assert_eq!(Solution::is_path_crossing(INPUT.to_string()), false);
    }

    #[test]
    fn ex2() {
        const INPUT: &str = "NESWW";
        assert_eq!(Solution::is_path_crossing(INPUT.to_string()), true);
    }
}
