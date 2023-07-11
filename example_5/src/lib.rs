pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn mytest() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(eq(4))
            .times(1)
            .returning(|x| x + 1);
        assert_eq!(5, mock.foo(4));
    }
}


