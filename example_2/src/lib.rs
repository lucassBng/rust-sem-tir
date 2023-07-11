
pub fn add_42(x: usize) -> usize {
    internal_add(x, 42)
}

fn internal_add(left: usize, right: usize) -> usize {
    left + right
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_add_2_2() {
        assert_eq!(internal_add(2, 2), 4);
    }

    #[test]
    fn add_42_42() {
        assert!(add_42(42) == 84);
    }

    #[test]
    #[ignore]
    fn add_42_should_not_be_greater_than_42() {
        assert!(add_42(1) < 42)
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn add_42_max_value_panic() {
        add_42(usize::MAX);
    }
 
    #[test]
    fn less_than_42_with_result() -> Result<(), String> {
        if 43 == add_42(1) {
            Ok(())
        } else {
            Err(String::from("1 + 42 shoul be 43"))
        }
    }
}
