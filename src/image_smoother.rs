// https://leetcode.com/problems/image-smoother/

pub struct Solution;

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = img.len();
        let cols = img[0].len();
        let mut res = vec![vec![0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let iter = (i.saturating_sub(1)..rows.min(i + 2))
                    .flat_map(|x| &img[x][j.saturating_sub(1)..cols.min(j + 2)]);
                let count = iter.clone().count() as u8;
                let sum = iter.sum::<i32>();
                res[i][j] = sum / count as i32;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let img = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let res = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::image_smoother(img), res);
    }
}
