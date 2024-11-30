use lust::greet::greet;
#[test]
fn greet_me() {
    let result = greet::hello("Sadique", "");
    assert_eq!(result, "Hello, Sadique", "");
}
#[test]
fn greet_the_world() {
    let result = greet::hello("", "");
    assert_eq!(result, "Hello, World");
}
#[test]
fn greet_me_in_spanish() {
    let result = greet::hello("Sadique", "spanish");
    assert_eq!(result, "Hola, Sadique");
}
