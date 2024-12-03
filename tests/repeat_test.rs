use lust::repeat::repeat;

#[test]
fn repeat_what_i_saidd() {
    let reuslt: String = repeat::repeat("a", 5);
    assert_eq!(reuslt, "aaaaa")
}
