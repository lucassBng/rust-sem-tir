pub fn add_42(x: usize) -> usize {
    internal_add(x, 42)
}

fn internal_add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract_42(x: usize) -> usize {
    x - 42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add_42(2);
        assert_eq!(result, 44);
    }

    #[test]
    fn it_works_too() {
        assert_eq!(subtract_42(43), 1)
    }
}
