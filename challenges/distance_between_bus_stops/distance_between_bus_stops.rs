// https://leetcode.com/problems/distance-between-bus-stops/

pub struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let forward = distance[std::cmp::min(start, destination) as usize
            ..std::cmp::max(start, destination) as usize]
            .iter()
            .sum();
        let all_around: i32 = distance.into_iter().sum();
        std::cmp::min(forward, all_around - forward)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(distance: &[i32], start: u16, destination: u16, expected: i32) {
        assert_eq!(
            Solution::distance_between_bus_stops(
                distance.to_vec(),
                start as i32,
                destination as i32
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4], 0, 1, 1)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4], 0, 2, 3)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3, 4], 0, 3, 4)
    }
}
