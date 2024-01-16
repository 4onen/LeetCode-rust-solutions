// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/

pub struct Solution;

// 2D array solution
// impl Solution {
//     pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
//         const MODULO: u32 = 1_000_000_000 + 7;
//         let tplus1 = target as usize + 1;
//         let mut arr: Vec<u32> = vec![0; tplus1 * (n as usize + 1)];

//         for i in 1..=k as usize {
//             if i <= target as usize {
//                 arr[1 * tplus1 + i] = 1;
//             }
//         }

//         for i in 2..=n as usize {
//             for j in 1..=target as usize {
//                 for l in 1..=k as usize {
//                     if j >= l {
//                         arr[i * tplus1 + j] =
//                             (arr[i * tplus1 + j] + arr[(i - 1) * tplus1 + (j - l)]) % MODULO;
//                     }
//                 }
//             }
//         }

//         arr[(n as usize) * tplus1 + (target as usize)] as i32
//     }
// }

// Two 1D array solution
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        if target < n || target > n * k || n < 0 || k < 0 || target < 0 {
            0
        } else {
            const MODULO: u32 = 1_000_000_000 + 7;

            let n = n as usize;
            let k = k as usize;
            let target = target as usize;

            let mut arr_past: Vec<u32> = vec![0; target + 1];
            let mut arr_current: Vec<u32> = vec![0; target + 1];

            arr_past[1..=usize::min(k, target)].fill(1);

            for _ in 2..=n {
                for j in 1..=target {
                    for l in 1..=usize::min(j, k) {
                        arr_current[j] = (arr_current[j] + arr_past[j - l]) % MODULO;
                    }
                }
                std::mem::swap(&mut arr_current, &mut arr_past);
                arr_current.fill(0);
            }

            arr_past[target as usize] as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::num_rolls_to_target(30, 30, 1000), 0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::num_rolls_to_target(30, 30, 900), 1);
    }
}
