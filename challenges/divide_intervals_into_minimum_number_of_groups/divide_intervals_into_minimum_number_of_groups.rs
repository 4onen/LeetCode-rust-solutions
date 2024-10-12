// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/

pub struct Solution;

// Initial idea: Track group ends (broken for cases like ex1)
// impl Solution {
//     pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
//         intervals.sort_unstable_by_key(|x| (x[1],std::cmp::Reverse(x[0])));
//         dbg!(&intervals);
//         let mut group_ends: Vec<i32> = vec![];
//         'outer: for interval in intervals {
//             dbg!(&group_ends);
//             for group_end in &mut group_ends {
//                 if *group_end < interval[0] {
//                     *group_end = interval[1];
//                     continue 'outer;
//                 }
//             }
//             group_ends.push(interval[1]);
//         }
//         group_ends.len() as i32
//     }
// }

// Initial sol'n: Track amount of overlap over range
impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts = std::vec::Vec::with_capacity(intervals.len());
        let mut ends = std::vec::Vec::with_capacity(intervals.len());
        for interval in intervals {
            starts.push(interval[0]);
            ends.push(interval[1]);
        }
        starts.sort_unstable();
        ends.sort_unstable();
        let starts = starts;
        let ends = ends;
        let mut overlap = 0;
        let mut max_overlap = 0;
        let mut start_idx = 0;
        let mut end_idx = 0;
        while start_idx < starts.len() {
            if starts[start_idx] <= ends[end_idx] {
                overlap += 1;
                max_overlap = std::cmp::max(overlap, max_overlap);
                start_idx += 1;
            } else {
                overlap -= 1;
                end_idx += 1;
            }
        }
        max_overlap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(intervals: &[[i32; 2]], expected: i32) {
        assert_eq!(
            Solution::min_groups(intervals.into_iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]], 3)
    }

    #[test]
    fn ex1_1() {
        test(&[[5, 10], [6, 8], [1, 5], [2, 3]], 2)
    }

    #[test]
    fn ex2() {
        test(&[[1, 3], [5, 6], [8, 10], [11, 13]], 1)
    }

    #[test]
    fn discussion_case1() {
        test(&[[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]], 1)
    }

    #[test]
    fn discussion_case2() {
        test(&[[1, 10], [2, 6], [4, 7], [5, 9]], 4)
    }

    #[test]
    fn discussion_case3() {
        test(&[[1, 5], [3, 7], [6, 10], [8, 12]], 2)
    }

    #[test]
    fn discussion_case4() {
        test(&[[1, 5], [1, 10], [1, 3], [10, 15]], 3)
    }

    #[test]
    fn discussion_case5() {
        test(
            &[[1, 500], [10, 200], [110, 220], [250, 350], [300, 400]],
            3,
        )
    }

    #[test]
    fn discussion_case6() {
        test(&[[1, 5], [3, 8], [10, 15], [1, 5], [3, 8]], 4)
    }

    #[test]
    fn discussion_case7() {
        test(&[[10, 15], [8, 10], [5, 8], [1, 5]], 2)
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                [1, 1000000],
                [100, 200000],
                [1000, 300000],
                [500000, 600000],
                [700000, 800000],
            ],
            3,
        )
    }
}
