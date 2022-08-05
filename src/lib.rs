#[cfg(test)]
mod tests {
    #[test]
    fn test_success() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Make this test fail");
    }
}