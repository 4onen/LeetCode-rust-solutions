// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
//         match points.len() {
//             0 => 0,
//             1..=100_000 => {
//                 points.sort_unstable_by_key(|x| x[1]);
//                 let mut arrows = 1;
//                 let mut end = points[0][1];
//                 for i in 1..points.len() {
//                     if points[i][0] > end {
//                         arrows += 1;
//                         end = points[i][1];
//                     }
//                 }
//                 arrows
//             }
//             _ => unreachable!("Invalid point count."),
//         }
//     }
// }

// Tupleized sol'n
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        type BalloonTup = (i32, i32);
        fn tupleize_balloons(trusts: Vec<Vec<i32>>) -> Box<[BalloonTup]> {
            let original_len = trusts.len();
            let trusts_ptr = trusts.leak();
            let trusts_ptr2: &mut [Vec<i32>] = &mut trusts_ptr[0..original_len];
            let trusts_tuple_ptr =
                unsafe { std::mem::transmute::<_, &mut [BalloonTup]>(trusts_ptr2) };
            for i in 0..original_len {
                let tup = (trusts_ptr[i][0], trusts_ptr[i][1]);
                trusts_tuple_ptr[i] = tup;
            }
            unsafe { std::boxed::Box::from_raw(trusts_tuple_ptr) }
        }
        match points.len() {
            0 => 0,
            1..=100_000 => {
                let mut points = tupleize_balloons(points);
                points.sort_unstable_by_key(|x| x.1);
                let mut arrows = 1;
                let mut end = points[0].1;
                for i in 1..points.len() {
                    if points[i].0 > end {
                        arrows += 1;
                        end = points[i].1;
                    }
                }
                arrows
            }
            _ => unreachable!("Invalid point count."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(2, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn ex2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(4, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn ex3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(2, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn discussion_case1() {
        let points = vec![];
        assert_eq!(0, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn discussion_case2() {
        let points = vec![vec![-2147483646, -2147483645], vec![2147483646, 2147483647]];
        assert_eq!(2, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn discussion_case3() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(2, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn discussion_case4() {
        let points = vec![vec![1, 2]];
        assert_eq!(1, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn discussion_case5() {
        let points = vec![vec![2, 3], vec![2, 3]];
        assert_eq!(1, Solution::find_min_arrow_shots(points));
    }

    #[test]
    fn failing_case1() {
        let points = vec![
            vec![9, 12],
            vec![1, 10],
            vec![4, 11],
            vec![8, 12],
            vec![3, 9],
            vec![6, 9],
            vec![6, 7],
        ];
        assert_eq!(2, Solution::find_min_arrow_shots(points));
    }
}
