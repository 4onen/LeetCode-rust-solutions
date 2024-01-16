#[cfg(test)]
mod tests {
    #[test]
    fn test_optional_ordering() {
        let a = Some(1);
        let b = Some(2);
        let c = None;
        assert!(a < b);
        assert!(b > a);
        assert!(a > c);
        assert!(c < a);
        assert!(b > c);
        assert!(c < b);
        assert_eq!(std::cmp::max(a, b), b);
        assert_eq!(std::cmp::max(a, c), a);
        assert_eq!(std::cmp::max(b, c), b);
        assert_eq!(std::cmp::min(a, b), a);
        assert_eq!(std::cmp::min(a, c), c);
        assert_eq!(std::cmp::min(b, c), c);
    }
}
