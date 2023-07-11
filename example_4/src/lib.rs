/// Adds 42 to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = example_4::add_42(arg);
///
/// assert_eq!(47, answer);
/// ```
pub fn add_42(x: usize) -> usize {
    42 + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_42(2);
        assert_eq!(result, 44);
    }
}
