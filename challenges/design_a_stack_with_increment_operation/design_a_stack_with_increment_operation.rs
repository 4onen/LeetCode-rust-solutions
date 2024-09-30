// https://leetcode.com/problems/design-a-stack-with-increment-operation/

// Initial sol'n
// pub struct CustomStack {
//     stack: Vec<i32>,
//     max_size: usize,
// }
// impl CustomStack {
//     #[allow(non_snake_case)] // Can't change the case of this function arg.
//     pub fn new(maxSize: i32) -> Self {
//         let max_size = maxSize as usize;
//         Self {
//             stack: Vec::with_capacity(max_size),
//             max_size,
//         }
//     }
//     pub fn push(&mut self, x: i32) {
//         if self.stack.len() < self.max_size {
//             self.stack.push(x);
//         }
//     }
//     pub fn pop(&mut self) -> i32 {
//         self.stack.pop().unwrap_or(-1)
//     }
//     pub fn increment(&mut self, k: i32, val: i32) {
//         assert!(k >= 0);
//         let k = k as usize;
//         for i in 0..std::cmp::min(k, self.stack.len()) {
//             self.stack[i] += val;
//         }
//     }
// }

// Stack-alloc sol'n
// const MAX_MAX_SIZE: u16 = 1000;
// pub struct CustomStack {
//     len: u16,
//     max_size: u16,
//     stack: [i32; MAX_MAX_SIZE as usize],
// }
// impl CustomStack {
//     #[allow(non_snake_case)] // Can't change the case of this function arg.
//     pub fn new(maxSize: i32) -> Self {
//         let max_size = maxSize as u16;
//         assert!(max_size >= 1);
//         assert!(max_size <= MAX_MAX_SIZE);
//         Self {
//             len: 0,
//             max_size,
//             // Safety: `MaybeUninit` is safe to use here because we init
//             // elements of the array when we push to the stack, so cannot
//             // read uninitialized memory (if the rest of the code is correct).
//             #[allow(invalid_value)]
//             stack: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
//         }
//     }
//     pub fn push(&mut self, x: i32) {
//         if self.len < self.max_size {
//             self.stack[self.len as usize] = x;
//             self.len += 1;
//         }
//     }
//     pub fn pop(&mut self) -> i32 {
//         if self.len <= 0 {
//             return -1;
//         }
//         self.len -= 1;
//         self.stack[self.len as usize]
//     }
//     pub fn increment(&mut self, k: i32, val: i32) {
//         assert!(k >= 0);
//         let k = k as u16;
//         for i in 0..std::cmp::min(k, self.len) {
//             self.stack[i as usize] += val;
//         }
//     }
// }

// Optimized increment iter sol'n (still stack allocated)
const MAX_MAX_SIZE: u16 = 1000;
pub struct CustomStack {
    len: u16,
    max_size: u16,
    stack: [i32; MAX_MAX_SIZE as usize],
}
impl CustomStack {
    #[allow(non_snake_case)] // Can't change the case of this function arg.
    pub fn new(maxSize: i32) -> Self {
        let max_size = maxSize as u16;
        assert!(max_size >= 1);
        assert!(max_size <= MAX_MAX_SIZE);
        Self {
            len: 0,
            max_size,
            // Safety: `MaybeUninit` is safe to use here because we init
            // elements of the array when we push to the stack, so cannot
            // read uninitialized memory (if the rest of the code is correct).
            #[allow(invalid_value)]
            stack: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        }
    }
    pub fn push(&mut self, x: i32) {
        if self.len < self.max_size {
            self.stack[self.len as usize] = x;
            self.len += 1;
        }
    }
    pub fn pop(&mut self) -> i32 {
        if self.len <= 0 {
            return -1;
        }
        self.len -= 1;
        self.stack[self.len as usize]
    }
    pub fn increment(&mut self, k: i32, val: i32) {
        assert!(k >= 0);
        let k = k as u16;
        self.stack
            .iter_mut()
            .take(k as usize)
            .for_each(|x| *x += val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestStep {
        New(u16),
        Push(u16),
        Increment(u16, u8),
        Pop(u8),  // expected
        PopEmpty, // expected
    }

    use TestStep::*;

    fn test(steps: &[TestStep]) {
        let mut obj = None;
        for (i, &step) in steps.iter().enumerate() {
            match (step, &mut obj) {
                (TestStep::New(max_size), obj) => {
                    assert!(max_size >= 1);
                    assert!(max_size <= 1000);
                    if obj.is_some() {
                        println!("Step {}: Warning: Stack was already created.", i);
                    }
                    *obj = Some(CustomStack::new(max_size as i32));
                }
                (TestStep::Push(x), Some(obj)) => {
                    assert!(x >= 1);
                    assert!(x <= 1000);
                    obj.push(x as i32)
                }
                (TestStep::Increment(k, val), Some(obj)) => {
                    assert!(k >= 1);
                    assert!(k <= 1000);
                    assert!(val <= 100);
                    obj.increment(k as i32, val as i32)
                }
                (TestStep::Pop(expected), Some(obj)) => {
                    assert_eq!(obj.pop(), expected as i32, "Step {}", i);
                }
                (TestStep::PopEmpty, Some(obj)) => {
                    assert_eq!(obj.pop(), -1, "Step {}", i);
                }
                (_, None) => {
                    unreachable!("Warning: Stack was not created.");
                }
            }
        }
    }

    #[test]
    fn ex1() {
        test(&[
            New(3),
            Push(1),
            Push(2),
            Pop(2),
            Push(2),
            Push(3),
            Push(4),
            Increment(5, 100),
            Increment(2, 100),
            Pop(103),
            Pop(202),
            Pop(201),
            PopEmpty,
        ]);
    }

    #[test]
    fn myex0() {
        test(&[
            New(1),
            PopEmpty,
            PopEmpty,
            Push(1),
            Pop(1),
            PopEmpty,
            Push(1),
            Push(2),
            Pop(1),
            PopEmpty,
            PopEmpty,
        ]);
    }
}
