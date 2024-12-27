use lust::dictionary::dictionary::{self, ERROR_NOT_FOUND, ERROR_WORD_EXIST, ERROR_WORD_NOT_EXIST};
use std::collections::HashMap;

fn setup_dictionary() -> HashMap<&'static str, &'static str> {
    return HashMap::from([("test", "this is just a test")]);
}
fn setup_dictionary_with_key(key: &str) -> HashMap<&str, &'static str> {
    return HashMap::from([(key, "this is just a test")]);
}
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
    let word = "test";
    let definition = "this is just a test";

    let got = dictionary::add(&mut dictionary, word, definition);
    let want = Ok(format!("definition {} added for key {}", definition, word));
    assert_eq!(got, want, "expected {:?} but got {:?}", want, got)
}
#[test]
fn test_add_with_existing_key() {
    let mut dictionary = setup_dictionary();
    let got = dictionary::add(&mut dictionary, "test", "this is just a test");

    assert_eq!(
        got, ERROR_WORD_EXIST,
        "expected {:?} but got {:?} ",
        ERROR_WORD_EXIST, got
    )
}
#[test]
fn test_update() {
    let word = "test";
    let new_definition = "new definition";
    let mut dictionary = setup_dictionary_with_key(&word);
    let got = dictionary::update(&mut dictionary, word, new_definition);
    let want = Ok(format!("updated {} to {}", word, new_definition));
    assert_eq!(got, want, "expected{:?} but got {:?}", want, got)
}
#[test]
fn test_update_non_existing_key() {
    let mut dictionary = setup_dictionary();
    let got = dictionary::update(&mut dictionary, "unknown", "unknown definition");

    assert_eq!(
        got, ERROR_WORD_NOT_EXIST,
        "expectd {:?} but got {:?}",
        ERROR_WORD_NOT_EXIST, got
    )
}

#[test]
fn test_delete() {
    let key = "test";
    let mut dictionary = setup_dictionary_with_key(key);
    let got = dictionary::delete(&mut dictionary, key);
    let want = Ok(format!("the definition and key {} removed", key));
    assert_eq!(got, want, "expected {:?} but got {:?}", want, got);
}

#[test]
fn test_delete_value_does_not_exist() {
    let mut dictionary = setup_dictionary();
    let got = dictionary::delete(&mut dictionary, "unknown");
    assert_eq!(
        got, ERROR_WORD_NOT_EXIST,
        "expected {:?} but got {:?}",
        ERROR_WORD_NOT_EXIST, got
    )
}
