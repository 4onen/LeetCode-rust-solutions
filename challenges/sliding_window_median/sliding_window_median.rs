// https://leetcode.com/problems/sliding-window-median/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
//         assert!(k > 0);
//         let k = k as u32;
//         assert!(k <= nums.len() as u32);
//         assert!(nums.len() <= 100000);
//         let mut working_window = nums[..k as usize].to_vec();
//         working_window.sort_unstable();
//         let mut result = Vec::with_capacity(nums.len() - k as usize + 1);
//         for i in k as usize..nums.len() {
//             result.push(if k % 2 == 0 {
//                 (working_window[k as usize / 2] as f64 + working_window[k as usize / 2 - 1] as f64)
//                     / 2.0
//             } else {
//                 working_window[k as usize / 2] as f64
//             });
//             let remove_index = working_window.binary_search(&nums[i - k as usize]).unwrap();
//             working_window.remove(remove_index);
//             let insert_index = working_window.binary_search(&nums[i]).unwrap_or_else(|x| x);
//             working_window.insert(insert_index, nums[i]);
//         }
//         // Calculate the last median
//         result.push(if k % 2 == 0 {
//             (working_window[k as usize / 2] as f64 + working_window[k as usize / 2 - 1] as f64)
//                 / 2.0
//         } else {
//             working_window[k as usize / 2] as f64
//         });
//         result
//     }
// }

// Heap pair sol'n
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        assert!(k > 0);
        let k = k as u32;
        assert!(k <= nums.len() as u32);
        assert!(nums.len() <= 100000);
        struct SplitHeapWindow {
            small_half: std::collections::BinaryHeap<i32>,
            large_half: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
            removed_elements: std::collections::HashMap<i32, u32>,
            odd: bool,
        }
        impl SplitHeapWindow {
            fn new(initial_window: &[i32], size_hint: u32) -> Self {
                // Max-heap
                let mut small_half: std::collections::BinaryHeap<i32> = {
                    let mut working_slice = Vec::with_capacity(size_hint as usize);
                    working_slice.extend_from_slice(initial_window);
                    std::collections::BinaryHeap::from(working_slice)
                };
                // Min-heap
                let mut large_half: std::collections::BinaryHeap<std::cmp::Reverse<i32>> =
                    std::collections::BinaryHeap::with_capacity(size_hint as usize);
                for _ in 0..initial_window.len() / 2 {
                    large_half.push(std::cmp::Reverse(small_half.pop().unwrap()));
                }
                // Hash map to track removed elements
                let removed_elements: std::collections::HashMap<i32, u32> =
                    std::collections::HashMap::with_capacity(size_hint as usize);
                let odd = initial_window.len() % 2 == 1;
                Self {
                    small_half,
                    large_half,
                    removed_elements,
                    odd,
                }
            }
            fn median(&self) -> f64 {
                if self.odd {
                    *self.small_half.peek().unwrap() as f64
                } else {
                    (*self.small_half.peek().unwrap() as f64
                        + self.large_half.peek().unwrap().0 as f64)
                        / 2.0
                }
            }
            fn remove(&mut self, element: i32) {
                if self.removed_elements.contains_key(&element) {
                    *self.removed_elements.get_mut(&element).unwrap() += 1;
                } else {
                    self.removed_elements.insert(element, 1);
                }
                loop {
                    if self.small_half.is_empty() {
                        break;
                    }
                    let Some(count) = self
                        .removed_elements
                        .get_mut(self.small_half.peek().unwrap())
                    else {
                        break;
                    };
                    let top = self.small_half.pop().unwrap();
                    *count -= 1;
                    if *count == 0 {
                        self.removed_elements.remove(&top);
                    }
                }
                loop {
                    if self.large_half.is_empty() {
                        break;
                    }
                    let Some(count) = self
                        .removed_elements
                        .get_mut(&self.large_half.peek().unwrap().0)
                    else {
                        break;
                    };
                    let top = self.large_half.pop().unwrap().0;
                    *count -= 1;
                    if *count == 0 {
                        self.removed_elements.remove(&top);
                    }
                }
            }
            fn median_push(&mut self, remove: i32, add: i32) {
                let mut balance = 0;
                if remove <= *self.small_half.peek().unwrap() {
                    balance -= 1;
                } else {
                    balance += 1;
                }
                if add > *self.small_half.peek().unwrap() {
                    self.large_half.push(std::cmp::Reverse(add));
                    balance -= 1;
                } else {
                    self.small_half.push(add);
                    balance += 1;
                }
                if balance > 0 {
                    self.large_half
                        .push(std::cmp::Reverse(self.small_half.pop().unwrap()));
                } else if balance < 0 {
                    self.small_half.push(self.large_half.pop().unwrap().0);
                }
            }
        }
        let mut window = SplitHeapWindow::new(&nums[..k as usize], nums.len() as u32);
        // Result vector
        let mut result = Vec::with_capacity(nums.len() - k as usize + 1);
        // Let's go!
        for i in k as usize..nums.len() {
            result.push(window.median());
            let add = nums[i];
            let remove = nums[i - k as usize];
            window.median_push(remove, add);
            window.remove(remove);
        }
        // Calculate the last median
        result.push(window.median());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ABS_ANSWER_LIMIT: f64 = 1e-5;

    fn test(nums: &[i32], k: i32, expected: &[f64]) {
        let result = Solution::median_sliding_window(nums.to_vec(), k);
        assert_eq!(result.len(), expected.len());
        let mut invalid_positions = vec![];
        for i in 0..result.len() {
            if (result[i] - expected[i]).abs() > ABS_ANSWER_LIMIT {
                invalid_positions.push((i, result[i], expected[i]));
            }
        }
        assert_eq!(invalid_positions, vec![]);
    }

    #[test]
    fn ex1() {
        test(
            &[1, 3, -1, -3, 5, 3, 6, 7],
            3,
            &[1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[1, 2, 3, 4, 2, 3, 1, 4, 2],
            3,
            &[
                2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000,
            ],
        )
    }

    #[test]
    fn myex1() {
        test(
            &[1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1],
            1,
            &[
                1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0,
            ],
        )
    }

    #[test]
    fn myex2() {
        test(
            &[1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1],
            2,
            &[
                1.5, 2.5, 2.5, 1.5, 1.5, 2.5, 2.5, 1.5, 1.5, 2.5, 2.5, 1.5, 1.5, 2.5, 2.5, 1.5,
            ],
        )
    }

    #[test]
    fn myex3() {
        test(
            &[1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1],
            3,
            &[
                2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
            ],
        )
    }

    #[test]
    fn myex4() {
        test(
            &[1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1],
            4,
            &[
                2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
            ],
        )
    }

    #[test]
    fn myex5() {
        test(
            &[1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1],
            5,
            &[
                2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
            ],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let nums = (0..100000).collect::<Vec<i32>>();
        let expected = (1..99999).map(|i| i as f64).collect::<Vec<f64>>();
        test(&nums, 3, &expected);
    }

    #[test]
    fn my_extreme_ex2() {
        let nums = (0..100000).collect::<Vec<i32>>();
        let expected = [49999.5; 1];
        test(&nums, 100000, &expected);
    }

    #[test]
    fn my_extreme_ex3() {
        let nums = (0..100000).collect::<Vec<i32>>();
        let expected = [49999.0, 50000.0];
        test(&nums, 99999, &expected);
    }

    #[test]
    fn my_extreme_ex4() {
        let nums = (i32::MAX - 100000..i32::MAX).collect::<Vec<i32>>();
        let expected = [i32::MAX as f64 - 50001.0, i32::MAX as f64 - 50000.0];
        test(&nums, 99999, &expected);
    }

    #[test]
    fn my_extreme_ex5() {
        let nums = (i32::MIN..i32::MIN + 100000).collect::<Vec<i32>>();
        let expected = [i32::MIN as f64 + 49999.5];
        test(&nums, 100000, &expected);
    }
}
