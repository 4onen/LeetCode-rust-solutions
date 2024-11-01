// https://leetcode.com/problems/delete-characters-to-make-fancy-string/

pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut b = s.into_bytes();
        let mut write = 2;
        for read in 2..b.len() {
            if b[read] == b[write - 1] && b[read] == b[write - 2] {
                continue;
            }
            b[write] = b[read];
            write += 1;
        }
        b.truncate(write);
        unsafe { String::from_utf8_unchecked(b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected: &str) {
        assert_eq!(Solution::make_fancy_string(input.to_string()), expected);
    }

    // fn test_prop(input: &str) {
    //     let res = Solution::make_fancy_string(input.to_string());
    //     let res_bytes = res.as_bytes();
    //     for i in 2..res_bytes.len() {
    //         assert!(res_bytes[i] != res_bytes[i - 1] || res_bytes[i] != res_bytes[i - 2]);
    //     }
    // }

    #[test]
    fn ex1() {
        test("leeetcode", "leetcode")
    }

    #[test]
    fn ex2() {
        test("aaabaaaa", "aabaa")
    }

    #[test]
    fn ex3() {
        test("aab", "aab")
    }

    #[test]
    fn discussion_case1() {
        test("a", "a")
    }

    #[test]
    fn discussion_case2() {
        test(
            "leeetcodeeeeeeeeeeeeeeeeeeeeeedddddddddddddddddddddd",
            "leetcodeedd",
        )
    }

    #[test]
    fn discussion_case9() {
        test("aaabaaaahgdkjzgfbdsjlhzvbldjfvhjjjjjjjjjjjjjjjjjjjjjjjjjgzzzzzzzzzzzzzzzzzzzzzzcbvhgvhgvkhgkvhgvkhgvhgkvkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkvvvvvvvvvvvvvvvvvvvvvvvvvvvvggggggggggggggggggffffffffffffffffffffffdddddddddddddddddddddddddddffffffffffffffffffffffffffffffffgggggggggggggggggggggggggggccccccccccccccccccccccccfffffffffffffffffffffdddddddddddddddfgjdhdhgfghfhjgfjhgfhgfhgggggggggggggggggggggggggggggggggggffdddddddddddddxcccccccccccccccccccccxxxxxxxxxxxxxxxxxxxxxxxxxxxxxzzzzzzzzzzzzzzzzzzzzzzzzsssssssssssssssssssssssssswwwwwwwwwwwwwwwwwdfffdfdgfdgfdgfghfhgfhgfhghgfhgfhghgfhghfhgfhgfhgfgfhgfhgfhgfhghfhgfhgfhghfhgfjhjfhjgfjhgfjhgjfcjgfcjgfxjgxdfdfxgcvvcxvcxvcxdxfdxfgdgfdcgfhgfhgvhg","aabaahgdkjzgfbdsjlhzvbldjfvhjjgzzcbvhgvhgvkhgkvhgvkhgvhgkvkkvvggffddffggccffddfgjdhdhgfghfhjgfjhgfhgfhggffddxccxxzzsswwdffdfdgfdgfdgfghfhgfhgfhghgfhgfhghgfhghfhgfhgfhgfgfhgfhgfhgfhghfhgfhgfhghfhgfjhjfhjgfjhgfjhgjfcjgfcjgfxjgxdfdfxgcvvcxvcxvcxdxfdxfgdgfdcgfhgfhgvhg");
    }
}
