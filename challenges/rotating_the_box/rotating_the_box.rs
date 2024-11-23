// https://leetcode.com/problems/rotating-the-box/

pub struct Solution;

impl Solution {
    pub fn rotate_the_box(mut r#box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        // First, apply gravity to the box along each row instead of column.
        for row in r#box.iter_mut() {
            let mut stone_count = 0;
            for i in 0..row.len() {
                match row[i] {
                    '.' => {}
                    '#' => {
                        row[i] = '.';
                        stone_count += 1;
                    }
                    '*' => {
                        row[i-stone_count..i].fill('#');
                        stone_count = 0;
                    }
                    _ => unreachable!(),
                }
            }
            let len = row.len();
            row[len-stone_count..].fill('#');
        }
        // Then, rotate the box by 90 degrees.
        let mut rotated = vec![vec!['.'; r#box.len()]; r#box[0].len()];
        for i in 0..r#box.len() {
            for j in 0..r#box[0].len() {
                rotated[j][r#box.len()-1-i] = r#box[i][j];
            }
        }
        // I'm too lazy to combine the operations. This should be fast enough,
        // and it's more readable this way.
        rotated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(b: &[&[char]], expected: &[&[char]]) {
        assert!(b.len() >= 1);
        assert!(b.len() <= 500);
        assert!(expected.len() >= 1);
        assert!(expected.len() <= 500);
        for row in b.iter() {
            assert!(row.len() >= 1);
            assert!(row.len() <= 500);
            assert_eq!(row.len(), expected.len());
            for &c in row.iter() {
                assert!(c == '.' || c == '#' || c == '*');
            }
        }
        for row in expected.iter() {
            assert!(row.len() >= 1);
            assert!(row.len() <= 500);
            assert_eq!(row.len(), b.len());
            for &c in row.iter() {
                assert!(c == '.' || c == '#' || c == '*');
            }
        }
        assert_eq!(
            Solution::rotate_the_box(b.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&['#', '.', '#']], &[&['.'], &['#'], &['#']])
    }

    #[test]
    fn ex2() {
        test(
            &[&['#', '.', '*', '.'], &['#', '#', '*', '.']],
            &[&['#', '.'], &['#', '#'], &['*', '*'], &['.', '.']],
        )
    }

    #[test]
    fn ex3() {
        test(
            &[
                &['#', '#', '*', '.', '*', '.'],
                &['#', '#', '#', '*', '.', '.'],
                &['#', '#', '#', '.', '#', '.'],
            ],
            &[
                &['.', '#', '#'],
                &['.', '#', '#'],
                &['#', '#', '*'],
                &['#', '*', '.'],
                &['#', '.', '*'],
                &['#', '.', '.'],
            ],
        )
    }
}
