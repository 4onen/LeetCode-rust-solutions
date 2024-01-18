// https://leetcode.com/problems/simplify-path/

pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for part in path.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                part => {
                    stack.push(part);
                }
            }
        }
        let mut result = String::with_capacity(path.len());
        for part in stack {
            result.push('/');
            result.push_str(part);
        }
        if result.is_empty() {
            result.push('/');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo"
        );
    }

    #[test]
    fn discussion_case1() {
        // Input: "/home/user/Documents/../Pictures"
        // Output: "/home/user/Pictures"
        assert_eq!(
            Solution::simplify_path("/home/user/Documents/../Pictures".to_string()),
            "/home/user/Pictures"
        );
    }

    #[test]
    fn discussion_case2() {
        // Input: "/../home/user/Documents"
        // Output: "/home/user/Documents"
        assert_eq!(
            Solution::simplify_path("/../home/user/Documents".to_string()),
            "/home/user/Documents"
        );
    }

    #[test]
    fn discussion_case3() {
        // Input: "/home/user/../../usr/local/bin"
        // Output: "/usr/local/bin"
        assert_eq!(
            Solution::simplify_path("/home/user/../../usr/local/bin".to_string()),
            "/usr/local/bin"
        );
    }

    #[test]
    fn discussion_case4() {
        // Input: "/home/user/./Downloads/../Pictures/././"
        // Output: "/home/user/Pictures"
        assert_eq!(
            Solution::simplify_path("/home/user/./Downloads/../Pictures/././".to_string()),
            "/home/user/Pictures"
        );
    }

    #[test]
    fn discussion_case5() {
        // Input: "/home/user/Documents/../../usr/local/bin"
        // Output: "/usr/local/bin"
        assert_eq!(
            Solution::simplify_path("/home/user/Documents/../../usr/local/bin".to_string()),
            "/home/usr/local/bin"
        );
    }

    #[test]
    fn discussion_case6() {
        // Input: "/home/user/Documents/../../../usr/local/bin"
        // Output: "/usr/local/bin"
        assert_eq!(
            Solution::simplify_path("/home/user/Documents/../../../usr/local/bin".to_string()),
            "/usr/local/bin"
        );
    }
}
