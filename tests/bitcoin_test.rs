use lust::bitcoin::bitcoin;
#[test]
fn wallet_test() {
    let mut wallet = bitcoin::new_wallet(10);

    assert_eq!(wallet.balance(), 10, "initiated balance should exist");

    wallet.deposite(10);
    assert_eq!(
        wallet.balance(),
        20,
        "balance is the sum of opening balance and deposit amount"
    );
    wallet.withdraw(15).unwrap();
    assert_eq!(
        wallet.balance(),
        5,
        "balance is the sum of opening balance and depsite amount and withraw amount"
    );
}
#[test]
fn withdraw_with_insufficient_funds() {
    let mut wallet = bitcoin::new_wallet(5);
    let error_response = wallet.withdraw(10);
    assert!(
        matches!(error_response, Err(_)),
        "expected an error got this {:?}",
        error_response
    );
    assert_eq!(error_response, Err("cannot withdraw, insufficient funds"))
}
