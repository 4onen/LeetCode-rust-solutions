// https://leetcode.com/problems/kth-largest-element-in-a-stream/

// Initial sol'n (Don't like the time complexity of this)
// pub struct KthLargest {
//     k: u16,
//     nums: Vec<std::cmp::Reverse<i32>>,
// }
// impl KthLargest {
//     pub fn new(k: i32, nums: Vec<i32>) -> Self {
//         let mut nums: Vec<std::cmp::Reverse<i32>> = unsafe { std::mem::transmute(nums) };
//         assert!(k > 0);
//         assert!(nums.len() <= 10_000);
//         let k = k as u16;
//         // Now find the kth _smallest_ element of the reversed list
//         nums.select_nth_unstable(k as usize - 1);
//         // Now we have the kth smallest element at index k-1
//         // Chop off the rest of the list
//         // This is safe because we know that k is within the bounds of the list
//         nums.truncate(k as usize);
//         // Sort the remaining elements
//         nums.sort_unstable();
//         Self { k, nums }
//     }
//     pub fn add(&mut self, val: i32) -> i32 {
//         let val = std::cmp::Reverse(val);
//         let pos = self.nums.binary_search(&val).unwrap_or_else(|x| x) as i32;
//         self.nums.insert(pos as usize, val);
//         self.nums.truncate(self.k as usize);
//         self.nums[self.k as usize - 1].0
//     }
// }

// Min-heap sol'n
pub struct KthLargest {
    k: u16,
    nums: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let nums: Vec<std::cmp::Reverse<i32>> = unsafe { std::mem::transmute(nums) };
        let mut nums: std::collections::BinaryHeap<std::cmp::Reverse<i32>> = nums.into();
        assert!(k > 0);
        let k = k as u16;
        while nums.len() > k as usize {
            nums.pop();
        }
        Self { k, nums }
    }
    pub fn add(&mut self, val: i32) -> i32 {
        if self.nums.len() < self.k as usize {
            let val = std::cmp::Reverse(val);
            self.nums.push(val);
        } else if val <= self.nums.peek().unwrap().0 {
            // Do nothing
        } else {
            let val = std::cmp::Reverse(val);
            self.nums.push(val);
            self.nums.pop();
        }
        self.nums.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }

    #[test]
    fn discussion_case1() {
        let mut obj = KthLargest::new(3, vec![4, 2]);
        assert_eq!(obj.add(1), 1);
        assert_eq!(obj.add(1), 1);
        assert_eq!(obj.add(-1), 1);
        assert_eq!(obj.add(3), 2);
        assert_eq!(obj.add(4), 3);
    }

    #[test]
    fn discussion_case2() {
        let mut obj = KthLargest::new(1, vec![]);
        assert_eq!(obj.add(2), 2);
        assert_eq!(obj.add(1), 2);
        assert_eq!(obj.add(-1), 2);
        assert_eq!(obj.add(3), 3);
        assert_eq!(obj.add(4), 4);
    }
}
