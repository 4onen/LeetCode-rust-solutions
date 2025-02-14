// https://leetcode.com/problems/product-of-the-last-k-numbers/

// Naive sol'n
// pub struct ProductOfNumbers {
//     nums: Vec<u32>,
// }
// impl ProductOfNumbers {
//     pub fn new() -> Self {
//         Self {
//             nums: std::vec::Vec::new(),
//         }
//     }
//     pub fn add(&mut self, num: i32) {
//         if num == 0 {
//             self.nums.clear();
//             self.nums.push(0)
//         }
//         self.nums.push(num as u32)
//     }
//     pub fn get_product(&self, k: i32) -> i32 {
//         let k = k as usize;
//         if k > self.nums.len() {
//             0
//         } else {
//             self.nums[self.nums.len() - k..]
//                 .iter()
//                 .copied()
//                 .product::<u32>() as i32
//         }
//     }
// }

// Clever prefix mul sol'n
pub struct ProductOfNumbers {
    products: Vec<u32>,
}
impl ProductOfNumbers {
    pub fn new() -> Self {
        Self {
            products: std::vec::Vec::new(),
        }
    }
    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.products.clear();
        } else {
            self.products
                .push(self.products.last().copied().unwrap_or(1) * num as u32)
        }
    }
    pub fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k > self.products.len() {
            0
        } else if k == self.products.len() {
            self.products.last().copied().unwrap() as i32
        } else {
            let back = self
                .products
                .get(self.products.len() - 1 - k as usize)
                .copied()
                .unwrap_or(1);
            let curr = self.products.last().copied().unwrap();
            (curr / back) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    enum Action {
        Add(u8),
        GetProduct(u16),
    }

    use Action::*;

    fn test(actions: &[Action], expected: &[Option<i32>]) {
        assert_eq!(expected.len(), actions.len());
        let mut obj = ProductOfNumbers::new();
        for (&action, &expectation) in std::iter::zip(actions.iter(), expected.iter()) {
            match (action, expectation) {
                (Action::Add(n), None) => {
                    assert!(n <= 100);
                    obj.add(n as i32)
                }
                (Action::GetProduct(k), Some(e)) => {
                    assert!(k >= 1);
                    assert!(k <= 40_000);
                    assert_eq!(obj.get_product(k as i32), e)
                }
                (Action::Add(n), Some(e)) => {
                    panic!("Add {n} given expectation {e}")
                }
                (Action::GetProduct(n), None) => {
                    panic!("getProduct {n} not given expectation")
                }
            }
        }
    }

    #[test]
    fn ex1() {
        test(
            // ["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"]
            // [[],[3],[0],[2],[5],[4],[2],[3],[4],[8],[2]]
            &[
                Add(3),
                Add(0),
                Add(2),
                Add(5),
                Add(4),
                GetProduct(2),
                GetProduct(3),
                GetProduct(4),
                Add(8),
                GetProduct(2),
            ],
            // [null,null,null,null,null,null,20,40,0,null,32]
            &[
                None,
                None,
                None,
                None,
                None,
                Some(20),
                Some(40),
                Some(0),
                None,
                Some(32),
            ],
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            // ["ProductOfNumbers", "add", "add", "add", "add", "getProduct", "add", "add", "add", "add", "add", "getProduct", "add", "getProduct", "add", "getProduct", "getProduct", "add", "add", "add", "add"]
            // [[], [2], [2], [5], [10], [2], [0], [5], [0], [2], [3], [3], [10], [4], [7], [2], [1], [8], [2], [6], [5]]
            &[
                Add(2),
                Add(2),
                Add(5),
                Add(10),
                GetProduct(2),
                Add(0),
                Add(5),
                Add(0),
                Add(2),
                Add(3),
                GetProduct(3),
                Add(10),
                GetProduct(4),
                Add(7),
                GetProduct(2),
                GetProduct(1),
                GetProduct(8),
                Add(2),
                Add(6),
                Add(5),
            ],
            &[
                None,
                None,
                None,
                None,
                Some(50),
                None,
                None,
                None,
                None,
                None,
                Some(0),
                None,
                Some(0),
                None,
                Some(70),
                Some(7),
                Some(0),
                None,
                None,
                None,
            ],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            // ["ProductOfNumbers", "add", "add", "add", "add", "add", "getProduct", "add", "add", "getProduct"]
            // [[], [1], [1], [0], [5], [4], [1], [1], [0], [3]]
            &[
                Add(1),
                Add(1),
                Add(0),
                Add(5),
                Add(4),
                GetProduct(1),
                Add(1),
                Add(0),
                GetProduct(3),
            ],
            &[None, None, None, None, None, Some(4), None, None, Some(0)],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            // ["ProductOfNumbers", "add", "add", "add", "add", "add", "getProduct"]
            // [[], [9], [4], [10], [5], [0], [1]]
            &[Add(9), Add(4), Add(10), Add(5), Add(0), GetProduct(1)],
            &[None, None, None, None, None, Some(0)],
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            // ["ProductOfNumbers", "add", "add", "add", "add", "getProduct", "getProduct", "add", "getProduct", "getProduct", "add", "add", "add", "add", "add", "getProduct", "getProduct", "add", "getProduct"]
            // [[], [2], [7], [10], [9], [3], [1], [5], [1], [1], [6], [10], [9], [2], [9], [2], [4], [1], [3]]
            &[
                Add(2),
                Add(7),
                Add(10),
                Add(9),
                GetProduct(3),
                GetProduct(1),
                Add(5),
                GetProduct(1),
                GetProduct(1),
                Add(6),
                Add(10),
                Add(9),
                Add(2),
                Add(9),
                GetProduct(2),
                GetProduct(4),
                Add(1),
                GetProduct(3),
            ],
            &[
                None,
                None,
                None,
                None,
                Some(7 * 10 * 9),
                Some(9),
                None,
                Some(5),
                Some(5),
                None,
                None,
                None,
                None,
                None,
                Some(2 * 9),
                Some(10 * 9 * 2 * 9),
                None,
                Some(2 * 9 * 1),
            ],
        )
    }
}
