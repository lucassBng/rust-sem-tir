use example_3;

#[test]
fn it_works_add() {
    assert_eq!(4, example_3::subtract_42(example_3::add_42(4)));
}
