// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/

pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = std::collections::VecDeque::from(students);
        let mut sandwiches_stack = sandwiches;
        sandwiches_stack.reverse();
        let mut time_since_last_student_ate = 0;
        while !students.is_empty() && time_since_last_student_ate < students.len() {
            let student = students.pop_front().unwrap();
            if student == sandwiches_stack.last().unwrap().clone() {
                sandwiches_stack.pop();
                time_since_last_student_ate = 0;
            } else {
                students.push_back(student);
                time_since_last_student_ate += 1;
            }
        }
        students.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }

    #[test]
    fn discussion_case1() {
        let students = [1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1];
        let sandwiches = [0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1];
        assert_eq!(
            Solution::count_students(students.to_vec(), sandwiches.to_vec()),
            6
        );
    }
}
