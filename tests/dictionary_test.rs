use lust::dictionary::dictionary::{self, ERROR_NOT_FOUND};
use std::collections::HashMap;

#[test]
fn search_known_word_test() {
    let mut dictionary = HashMap::new();
    dictionary.insert("test", "this is just a test");
    let got = dictionary::search(&mut dictionary, "test");
    let want = Ok("this is just a test");
    assert_eq!(got, want, "expected {:?} and got {:?}", want, got)
}
#[test]
fn search_unknown_word_test() {
    let mut dictionary = HashMap::new();
    dictionary.insert("test", "this is just a test");
    let got = dictionary::search(&mut dictionary, "unknown");
    // let want = Err("could not found the word you were looking for");
    assert_eq!(
        got, ERROR_NOT_FOUND,
        "Expected an error here {:?} instead got {:?}",
        ERROR_NOT_FOUND, got
    )
}
#[test]
fn test_add() {
    let mut dictionary = HashMap::new();

    let got = dictionary::add(&mut dictionary, "test", "this is just a test");
    let want = Ok(());
    assert_eq!(got, want, "expected {:?} but got {:?}", want, got)
}
#[test]
fn test_add_with_existing_key() {
    let mut dictionary = HashMap::from([("test", "this is just a test")]);
    let got = dictionary::add(&mut dictionary, "test", "this is just a test");
    let want = Err("cannot add word because it already exists");
    assert_eq!(got, want, "expected {:?} but got {:?} ", want, got)
}
