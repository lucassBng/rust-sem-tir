#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use example_3;


#[quickcheck]
#[ignore]
fn double_42_is_identity(x: usize) -> bool {
    x == example_3::subtract_42(example_3::add_42(x))
}
