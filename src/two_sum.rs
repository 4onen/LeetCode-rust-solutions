pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut matches = std::collections::HashMap::new();
    for (i, num) in nums.into_iter().enumerate() {
        if let Some(idx) = matches.get(&num) {
            return vec![*idx, i as i32];
        }
        matches.insert(target - num, i as i32);
    }
    unreachable!("All inputs should have exactly one solution!");
}

#[cfg(test)]
mod test{
    use super::two_sum;

    #[test]
    fn obvious() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }
}

