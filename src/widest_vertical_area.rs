// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/

pub struct Solution;

// Easy sorted vec differences implementation
// impl Solution {
//     pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
//         let xs = {
//             let mut xs: Vec<i32> = points.into_iter().map(|p| p[0]).collect();
//             xs.sort();
//             xs
//         };

//         xs.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
//     }
// }

// Harder windowed solution
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let xs: std::collections::BTreeSet<i32> = points.into_iter().map(|p| p[0]).collect();

        let mut max = 0;
        let mut xs_iter = xs.into_iter();
        if let Some(mut last) = xs_iter.next() {
            for x in xs_iter {
                max = max.max(x - last);
                last = x;
            }
            max
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![8, 7],
                vec![9, 9],
                vec![7, 4],
                vec![9, 7]
            ]),
            1
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
