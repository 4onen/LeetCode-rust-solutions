// https://leetcode.com/problems/water-bottles/

pub struct Solution;

impl Solution {
    pub const fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        assert!(num_exchange >= 2);
        let mut drank = num_bottles;
        let mut empty = num_bottles;
        while empty >= num_exchange {
            let new_bottles = empty / num_exchange;
            empty = new_bottles + empty % num_exchange;
            drank += new_bottles;
        }
        drank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn test(num_bottles: u8, num_exchange: u8, expected: u8) {
        assert!(num_bottles >= 1);
        assert!(num_bottles <= 100);
        assert!(num_exchange >= 2);
        assert!(num_exchange <= 100);
        let result = Solution::num_water_bottles(num_bottles as i32, num_exchange as i32);
        assert!(result >= 1);
        assert!(result == expected as i32);
    }

    #[test]
    fn ex1() {
        test(9, 3, 13);
    }

    #[test]
    fn ex2() {
        test(15, 4, 19);
    }
}
