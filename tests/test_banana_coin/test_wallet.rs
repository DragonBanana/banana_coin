extern crate banana_coin;
use banana_coin::model::*;
use self::banana_coin::error::WalletError;

#[test]
fn add_coins_successful_from_negative() {
    let mut wallet = Wallet::new(
        -5
    );
    match wallet.add_coins(100) {
        Ok(_) => assert_eq!(wallet.get_balance(), 95),
        Err(_) => assert!(false)
    }
}

#[test]
fn add_coins_unsuccessful_zero_amount() {
    let mut wallet: Wallet = Wallet::new(
        0
    );
    match wallet.add_coins(0) {
        Ok(_) => assert!(false),
        Err(WalletError::AddZeroCoinError) => assert!(true),
        Err(_) => assert!(false)
    }
}

#[test]
fn add_coins_unsuccessful_overflow_max() {
    let mut wallet: Wallet = Wallet::new(
        i64::max_value()
    );
    match wallet.add_coins(10) {
        Ok(_) => assert!(false),
        Err(WalletError::AddCoinOverflowError {..}) => assert!(true),
        Err(_) => assert!(false)
    }
}

#[test]
fn remove_coins_successful() {
    let mut wallet: Wallet = Wallet::new(
        100
    );
    match wallet.remove_coins(50, false) {
        Ok(_) => assert_eq!(wallet.get_balance(), 50),
        Err(_) => assert!(false)
    }
    match wallet.remove_coins(50, true) {
        Ok(_) => assert_eq!(wallet.get_balance(), 0),
        Err(_) => assert!(false)
    }
}

#[test]
fn remove_coins_successful_from_negative() {
    let mut wallet: Wallet = Wallet::new(
        -5
    );
    match wallet.remove_coins(50, true) {
        Ok(_) => assert_eq!(wallet.get_balance(), -55),
        Err(_) => assert!(false)
    }
}

#[test]
fn remove_coins_unsuccessful_zero_amount() {
    let mut wallet: Wallet = Wallet::new(
        100
    );
    match wallet.remove_coins(0, false) {
        Ok(_) => assert!(false),
        Err(WalletError::RemoveZeroCoinError) => assert!(true),
        Err(_) => assert!(false)
    }
    match wallet.remove_coins(0, true) {
        Ok(_) => assert!(false),
        Err(WalletError::RemoveZeroCoinError) => assert!(true),
        Err(_) => assert!(false)
    }
}

#[test]
fn remove_coins_unsuccessful_overflow_max() {
    let mut wallet: Wallet = Wallet::new(
        i64::min_value()
    );
    match wallet.remove_coins(10, true) {
        Ok(_) => assert!(false),
        Err(WalletError::RemoveCoinOverflowError {..}) => assert!(true),
        Err(_) => assert!(false)
    }
    match wallet.remove_coins(10, false) {
        Ok(_) => assert!(false),
        Err(WalletError::RemoveCoinOverflowError {..}) => assert!(true),
        Err(_) => assert!(false)
    }
}

#[test]
fn remove_coins_unsuccessful_negative_balance() {
    let mut wallet: Wallet = Wallet::new(
        0
    );
    match wallet.remove_coins(10, false) {
        Ok(_) => assert!(false),
        Err(WalletError::RemoveCoinNegativeBalanceError {..}) => assert!(true),
        Err(_) => assert!(false)
    }
}