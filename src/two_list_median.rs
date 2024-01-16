// https://leetcode.com/problems/median-of-two-sorted-arrays/

pub struct Solution;

fn median(nums: &[i32]) -> f64 {
    assert!(nums.len() > 0);

    let halfway = nums.len() >> 1;
    if nums.len() & 1 == 0 {
        (nums[halfway - 1] + nums[halfway]) as f64 / 2.0
    } else {
        // 1 >> 1 = 0
        // 3 >> 1 = 1
        // 5 >> 1 = 2
        // 7 >> 1 = 3
        nums[halfway] as f64
    }
}

fn merge<T: Ord + std::fmt::Debug>(n1: Vec<T>, n2: Vec<T>) -> Vec<T> {
    let result_len = n1.len() + n2.len();
    let mut result = Vec::with_capacity(result_len);

    unsafe {
        result.set_len(result_len);
    }

    let mut i1 = n1.into_iter().peekable();
    let mut i2 = n2.into_iter().peekable();

    for i in 0..result_len {
        result[i] = match (i1.peek(), i2.peek()) {
            (None, None) => {
                unreachable!("We've run off the end of the result!")
            }
            (Some(_), None) => i1.next().unwrap(),
            (None, Some(_)) => i2.next().unwrap(),
            (Some(v1), Some(v2)) if v1 <= v2 => i1.next().unwrap(),
            (Some(_), Some(_)) => i2.next().unwrap(),
        };
    }

    result
}

impl Solution {
    // Naive solution: Merge and sort and get median
    // Cost: O((m+n)log(m+n))
    // pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //     let nums3 = {
    //         let mut nums: Vec<i32> = nums1.into_iter().chain(nums2.into_iter()).collect();
    //         nums.sort();
    //         nums
    //     };
    //     median(&nums3)
    // }

    // Linear solution: Because the two lists are sorted,
    //  we can use the merge step of mergesort to get a sorted,
    //  merged list more efficiently.
    // Cost: O(m+n)
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums3 = merge(nums1, nums2);
        median(&nums3)
    }

    // Linear solution two: Because we only care about the middle two elements,
    // we can also use a different linear time algorithm often known as select-
    // -kth-element. This is likely to have a larger constant factor than
    // the above merging approach (i.e. will be slower.)
    // Also less maintainable because it implements its own median selection.
    // pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //     let mut nums3: Vec<i32> = nums1.into_iter().chain(nums2.into_iter()).collect();
    //     fn unsorted_median(nums: &mut [i32]) -> f64 {
    //         assert!(nums.len() > 0);
    //         let halfway = nums.len() >> 1;
    //         if nums.len() & 1 == 0 {
    //             // Since the midpoint lies between two numbers,
    //             // we first select the one just before,
    //             // then from the now-partitioned list we select the first
    //             // num of the second partition, which will be the number
    //             // right after the midpoint of the original.
    //             let (_, middle1, rest) = nums.select_nth_unstable(halfway - 1);
    //             (*middle1 + *rest.select_nth_unstable(0).1) as f64 / 2.0
    //         } else {
    //             *nums.select_nth_unstable(halfway).1 as f64
    //         }
    //     }
    //     unsorted_median(&mut nums3)
    // }

    // Logarithmic solution: Leetcode says this exists but the merge-median
    // linear solution is so fast and clean I don't want to spend the time.
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn one_empty() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2, 3]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![2, 3]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 4], vec![2, 3]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]),
            2.5
        );
    }
}
