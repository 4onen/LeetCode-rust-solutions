// https://leetcode.com/problems/maximum-average-pass-ratio/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
//         #[derive(Clone, Copy)]
//         struct Class {
//             passing: i32,
//             total: i32,
//         }
//         impl Class {
//             pub fn ratio(&self) -> f64 {
//                 (self.passing as f64) / (self.total as f64)
//             }
//             pub fn marginal_utility(&self) -> f64 {
//                 let pp1 = self.passing as f64 + 1.;
//                 let tp1 = self.total as f64 + 1.;
//                 pp1 / tp1 - (self.passing as f64) / (self.total as f64)
//             }
//         }
//         impl PartialEq for Class {
//             fn eq(&self, other: &Self) -> bool {
//                 self.cmp(other) == std::cmp::Ordering::Equal
//             }
//         }
//         impl PartialOrd for Class {
//             fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//                 Some(self.cmp(other))
//             }
//         }
//         impl Eq for Class {}
//         impl Ord for Class {
//             fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//                 f64::partial_cmp(&self.marginal_utility(), &other.marginal_utility()).unwrap()
//             }
//         }
//         let mut perfect_classes = 0;
//         let mut heap: std::collections::BinaryHeap<Class> = classes
//             .into_iter()
//             .map(|class| Class {
//                 passing: class[0],
//                 total: class[1],
//             })
//             .filter(|&class| {
//                 if class.passing == class.total {
//                     perfect_classes += 1;
//                     false
//                 } else {
//                     true
//                 }
//             })
//             .collect();
//         if heap.len() < 1 {
//             return ((perfect_classes > 0) as i8) as f64;
//         }
//         for _ in 0..extra_students {
//             let mut worst_class = heap.peek_mut().unwrap();
//             worst_class.passing += 1;
//             worst_class.total += 1;
//         }
//         let total_classes = perfect_classes + heap.len();
//         (perfect_classes as f64 + heap.into_iter().map(|class| class.ratio()).sum::<f64>())
//             / total_classes as f64
//     }
// }

// Integer rational sol'n
// impl Solution {
//     pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
//         #[derive(Clone, Copy)]
//         struct Class {
//             failing: u32,
//             total: u32,
//         }
//         impl Class {
//             fn ratio(&self) -> f64 {
//                 (self.total - self.failing) as f64 / self.total as f64
//             }
//         }
//         impl PartialEq for Class {
//             fn eq(&self, other: &Self) -> bool {
//                 self.cmp(other) == std::cmp::Ordering::Equal
//             }
//         }
//         impl PartialOrd for Class {
//             fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//                 Some(self.cmp(other))
//             }
//         }
//         impl Eq for Class {}
//         impl Ord for Class {
//             fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//                 u64::cmp(
//                     &(self.failing as u64 * other.total as u64 * other.total as u64),
//                     &(other.failing as u64 * self.total as u64 * self.total as u64),
//                 )
//             }
//         }
//         let mut perfect_classes = 0;
//         let mut heap: std::collections::BinaryHeap<Class> = classes
//             .into_iter()
//             .map(|class| {
//                 let passing = class[0] as u32;
//                 let total = class[1] as u32;
//                 Class {
//                     failing: total - passing,
//                     total,
//                 }
//             })
//             .filter(|&class| {
//                 if class.failing == 0 {
//                     perfect_classes += 1;
//                     false
//                 } else {
//                     true
//                 }
//             })
//             .collect();
//         if heap.len() < 1 {
//             return ((perfect_classes > 0) as i8) as f64;
//         }
//         for _ in 0..extra_students {
//             let mut worst_class = heap.peek_mut().unwrap();
//             worst_class.total += 1;
//         }
//         let total_classes = perfect_classes + heap.len();
//         (perfect_classes as f64 + heap.into_iter().map(|class| class.ratio()).sum::<f64>())
//             / total_classes as f64
//     }
// }

// Integer rational sol'n with no perfect classes (No change in results, simpler code, win.)
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        #[derive(Clone, Copy)]
        struct Class {
            failing: u32,
            total: u32,
        }
        impl Class {
            fn ratio(&self) -> f64 {
                (self.total - self.failing) as f64 / self.total as f64
            }
        }
        impl PartialEq for Class {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == std::cmp::Ordering::Equal
            }
        }
        impl PartialOrd for Class {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Eq for Class {}
        impl Ord for Class {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                u64::cmp(
                    &(self.failing as u64 * other.total as u64 * other.total as u64),
                    &(other.failing as u64 * self.total as u64 * self.total as u64),
                )
            }
        }
        let mut heap: std::collections::BinaryHeap<Class> = classes
            .into_iter()
            .map(|class| {
                let passing = class[0] as u32;
                let total = class[1] as u32;
                Class {
                    failing: total - passing,
                    total,
                }
            })
            .collect();
        for _ in 0..extra_students {
            let mut worst_class = heap.peek_mut().unwrap();
            worst_class.total += 1;
        }
        let total_classes = heap.len() as f64;
        heap.into_iter().map(|class| class.ratio()).sum::<f64>() / total_classes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(classes: &[[i32; 2]], extra_students: i32, expected: f64) {
        assert!(classes.len() >= 1);
        assert!(classes.len() <= 100_000);
        assert!(extra_students >= 1);
        assert!(extra_students <= 100_000);
        for &[pass, total] in classes {
            assert!(pass >= 1);
            assert!(total >= pass);
            assert!(total <= 100_000);
        }
        let result = Solution::max_average_ratio(
            classes.iter().map(|&x| x.to_vec()).collect(),
            extra_students,
        );
        let result_diff = (result - expected).abs();
        assert!(
            result_diff < 10e-5,
            "Result: {}, Expected: {}",
            result,
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[[1, 2], [3, 5], [2, 2]], 2, 0.78333)
    }

    #[test]
    fn ex2() {
        test(&[[2, 4], [3, 9], [4, 5], [2, 10]], 4, 0.53485)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[[99_999, 100_000]], 100_000, 199_999. / 200_000.)
    }

    #[test]
    fn my_extreme_ex2() {
        test(
            &[[99_999, 100_000], [100_000; 2]],
            100_000,
            ((199_999. / 200_000.) + 1.) / 2.,
        )
    }

    #[test]
    fn my_extreme_ex3() {
        test(&[[99_999, 100_000]; 100_000], 100_000, 100_000. / 100_001.)
    }

    #[test]
    fn my_extreme_ex4() {
        test(&[[100_000; 2]; 100_000], 100_000, 1.)
    }
}
