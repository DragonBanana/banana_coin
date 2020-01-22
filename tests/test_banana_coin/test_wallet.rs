extern crate banana_coin;
use super::*;

#[test]
fn new_wallet() {
    let wallet = Wallet::new(
        100
    );
    assert_eq!(wallet.n_coins, 100);
}

#[test]
fn add_coins_successful() {
    let mut wallet = Wallet::new(
        100
    );
    wallet.add_coins(100);
    assert_eq!(wallet.n_coins, 200);
}

#[test]
fn add_coins_successful_from_negative() {
    let mut wallet = Wallet::new(
        -5
    );
    wallet.add_coins(100);
    assert_eq!(wallet.n_coins, 95);
}

#[test]
#[should_panic]
fn add_coins_unsuccessful_negative_amount() {
    let mut wallet = Wallet::new(
        -5
    );
    wallet.add_coins(-10);
    assert_eq!(wallet.n_coins, 95);
}