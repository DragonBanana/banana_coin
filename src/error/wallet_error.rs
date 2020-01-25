use std::{error, fmt};
use std::fmt::{Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::WalletError::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum WalletError {
    AddCoinError,
    AddZeroCoinError,
    AddCoinOverflowError { current_amount: i64, added_amount: u32 },
    RemoveCoinError,
    RemoveZeroCoinError,
    RemoveCoinOverflowError { current_amount: i64, removed_amount: u32 },
    RemoveCoinNegativeBalanceError { current_amount: i64, removed_amount: u32, negative_balance_allowed: bool },
}

impl error::Error for WalletError {}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            AddCoinError =>
                write!(f, "Impossible to add coin to the wallet. This error should never happen."),
            AddZeroCoinError =>
                write!(f, "The wallet does not allow to add a zero/null amount of coins."),
            AddCoinOverflowError { current_amount, added_amount } =>
                write!(f, "An overflow has been detected: the current amount is {} and the added amount is {}", current_amount, added_amount),
            RemoveCoinError =>
                write!(f, "Impossible to remove coin from the wallet. This error should never happen."),
            RemoveZeroCoinError =>
                write!(f, "The wallet does not allow to remove a zero/null amount of coins."),
            RemoveCoinOverflowError { current_amount, removed_amount } =>
                write!(f, "An overflow has been detected: the current amount is {} and the removed amount is {}", current_amount, removed_amount),
            RemoveCoinNegativeBalanceError { current_amount, removed_amount, negative_balance_allowed } =>
                write!(f, "Removing coins from this wallet will return a negative balance: the current amount is {} and the removed amount is {} and negative balance flag is {}", current_amount, removed_amount, negative_balance_allowed)
        }
    }
}