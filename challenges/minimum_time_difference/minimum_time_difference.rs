// https://leetcode.com/problems/minimum-time-difference/

pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points: Vec<i16> = time_points
            .into_iter()
            .map(|s| {
                let hours = s[0..2].parse::<i16>().unwrap();
                let minutes = s[3..5].parse::<i16>().unwrap();
                hours * 60 + minutes
            })
            .collect();
        time_points.sort_unstable();
        let mut min_diff = i16::MAX;
        for i in 1..time_points.len() {
            min_diff = std::cmp::min(min_diff, time_points[i] - time_points[i - 1]);
        }
        min_diff = std::cmp::min(min_diff, time_points[0] + 24*60 - time_points[time_points.len() - 1]);
        min_diff as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(time_points: &[&str], expected: i32) {
        let time_points = time_points.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_min_difference(time_points), expected);
    }

    #[test]
    fn ex1() {
        test(&["23:59", "00:00"], 1)
    }

    #[test]
    fn ex2() {
        test(&["00:00", "23:59", "00:00"], 0)
    }

    #[test]
    fn failing_case1() {
        test(&["00:00","04:00","22:00"], 120)
    }
}
