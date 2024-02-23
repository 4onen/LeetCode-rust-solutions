// https://leetcode.com/problems/cheapest-flights-within-k-stops/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
//         assert!(n > 0);
//         let n = n as u8;
//         assert!(flights.len() as u16 <= (n as u16 * (n as u16 - 1) / 2));
//         assert!(0 <= src);
//         let src = src as u8;
//         assert!(src < n);
//         assert!(0 <= dst);
//         let dst = dst as u8;
//         assert!(dst < n);
//         assert!(0 <= k);
//         let k = k as u8;
//         assert!(k < n);
//         let mut dp_past = vec![i32::MAX as u32; n as usize];
//         let mut dp_curr = vec![i32::MAX as u32; n as usize];
//         dp_past[src as usize] = 0;
//         dp_curr[src as usize] = 0;
//         for _ in 0..k + 1 {
//             for flight in &flights {
//                 let u = flight[0] as usize;
//                 let v = flight[1] as usize;
//                 let w = flight[2] as u32;
//                 dp_curr[v] = dp_curr[v].min(dp_past[u] + w);
//             }
//             std::mem::swap(&mut dp_past, &mut dp_curr);
//         }
//         let min_to_target = dp_past[dst as usize];
//         if min_to_target >= i32::MAX as u32 {
//             -1
//         } else {
//             min_to_target as i32
//         }
//     }
// }

// Tupleized sol'n
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        assert!(n > 0);
        let n = n as u8;
        assert!(flights.len() as u16 <= (n as u16 * (n as u16 - 1) / 2));
        assert!(0 <= src);
        let src = src as u8;
        assert!(src < n);
        assert!(0 <= dst);
        let dst = dst as u8;
        assert!(dst < n);
        assert!(0 <= k);
        let k = k as u8;
        assert!(k < n);
        type FlightTup = (u8, u8, std::num::NonZeroU16);
        fn tupleize_flights(trusts: Vec<Vec<i32>>) -> Box<[FlightTup]> {
            // First, convert this vector of vectors to a more useful vector
            // of tuples. We know each subvector has exactly 3 small eles, so
            // we can stuff them into tuples in the same memory space as the
            // original sub-vectors.
            // Obviously super unsafe if run on a 16-bit system or something,
            // but this is a toy problem on leetcode, so the environment is
            // known.
            let original_len = trusts.len();
            let trusts_ptr = trusts.leak();
            let trusts_ptr2: &mut [Vec<i32>] = &mut trusts_ptr[0..original_len];
            let trusts_tuple_ptr =
                unsafe { std::mem::transmute::<_, &mut [FlightTup]>(trusts_ptr2) };
            for i in 0..original_len {
                // We know all the prices are 1 to 10000, so we can safely
                // convert them to NonZeroU16s.
                let tup = unsafe {
                    (
                        trusts_ptr[i][0] as u8,
                        trusts_ptr[i][1] as u8,
                        std::num::NonZeroU16::new_unchecked(trusts_ptr[i][2] as u16),
                    )
                };
                trusts_tuple_ptr[i] = tup;
            }
            unsafe { std::boxed::Box::from_raw(trusts_tuple_ptr) }
        }
        let flights = tupleize_flights(flights);
        let mut dp_past = vec![i32::MAX as u32; n as usize];
        let mut dp_curr = vec![i32::MAX as u32; n as usize];
        dp_past[src as usize] = 0;
        dp_curr[src as usize] = 0;
        for _ in 0..k + 1 {
            for &(from, to, price) in flights.iter() {
                dp_curr[to as usize] = std::cmp::min(
                    dp_curr[to as usize],
                    dp_past[from as usize] + price.get() as u32,
                );
            }
            std::mem::swap(&mut dp_past, &mut dp_curr);
        }
        let min_to_target = dp_past[dst as usize];
        if min_to_target >= i32::MAX as u32 {
            -1
        } else {
            min_to_target as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 700);
    }

    #[test]
    fn ex2() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 200);
    }

    #[test]
    fn ex3() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 500);
    }

    #[test]
    fn my_empty_ex1() {
        let n = 1;
        let flights = vec![];
        let src = 0;
        let dst = 0;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 0);
    }

    #[test]
    fn my_empty_ex2() {
        let n = 2;
        let flights = vec![];
        let src = 0;
        let dst = 0;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 0);
    }

    #[test]
    fn my_empty_ex3() {
        let n = 2;
        let flights = vec![];
        let src = 0;
        let dst = 1;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), -1);
    }

    #[test]
    fn my_empty_ex4() {
        let n = 2;
        let flights = vec![];
        let src = 0;
        let dst = 0;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 0);
    }

    #[test]
    fn my_empty_ex6() {
        let n = 2;
        let flights = vec![];
        let src = 0;
        let dst = 1;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), -1);
    }

    #[test]
    fn myex1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 2;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 400);
    }

    #[test]
    fn discussion_case1() {
        let n = 11;
        let flights = [
            [0, 3, 3],
            [3, 4, 3],
            [4, 1, 3],
            [0, 5, 1],
            [5, 1, 100],
            [0, 6, 2],
            [6, 1, 100],
            [0, 7, 1],
            [7, 8, 1],
            [8, 9, 1],
            [9, 1, 1],
            [1, 10, 1],
            [10, 2, 1],
            [1, 2, 100],
        ];
        let src = 0;
        let dst = 2;
        let k = 4;
        assert_eq!(
            Solution::find_cheapest_price(
                n,
                flights.into_iter().map(|x| x.to_vec()).collect(),
                src,
                dst,
                k
            ),
            11
        );
    }
}
