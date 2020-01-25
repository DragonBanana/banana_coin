use serde::{Deserialize, Serialize};

use crate::error::WalletError;

/// Wallet structure
/// It represents a wallet. It contains coins.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Wallet {
    /// Amount of coins contained in the wallet.
    /// It is a signed integer 'i64', so it can be positive and negative.
    balance: i64
}

/// Wallet implementation.
impl Wallet {
    ///
    /// Return a new Wallet.
    ///
    /// # Arguments
    ///
    /// * `balance` - A i64 integer that represents the amount of coins in the wallet.
    ///
    /// # Example
    /// ```
    /// use banana_coin::model::Wallet;
    /// let wallet = Wallet::new(
    ///     100
    /// );
    /// # assert_eq!(wallet.get_balance(), 100)
    /// ```
    ///
    pub fn new(
        balance: i64
    ) -> Wallet {
        Wallet {
            balance: balance
        }
    }

    ///
    /// Return a copy of the amount of coins in the wallet.
    ///
    /// # Example
    /// ```
    /// use banana_coin::model::Wallet;
    /// let wallet = Wallet::new(
    ///     100
    /// );
    /// let wallet_balance : i64 = wallet.get_balance();
    /// # assert_eq!(wallet_balance, 100)
    /// ```
    ///
    pub fn get_balance(
        self: & Wallet,
    ) -> i64 {
        self.balance.clone()
    }

    ///
    /// Add coins to the wallet.
    ///
    /// # Arguments
    ///
    /// * `coins_to_add` - A u32 number that represents the amount of coin to add to the wallet.
    ///
    /// # Example
    ///
    /// ```
    /// # use banana_coin::model::Wallet;
    /// let mut wallet = Wallet::new(
    ///     100
    /// );
    /// # assert_eq!(wallet.get_balance(), 100);
    /// let result = wallet.add_coins(100);
    /// match result {
    ///     Ok(_) => println!("Coins added"),
    ///     Err(e) => println!("{}", e)
    /// }
    /// # assert_eq!(wallet.get_balance(), 200);
    /// ```
    ///
    pub fn add_coins(
        self: &mut Wallet,
        coins_to_add: u32,
    ) -> Result<(), WalletError> {
        // Checking for overflow
        match coins_to_add {
            x if x == 0 => Err(WalletError::AddZeroCoinError),
            x if x > 0 => {
                match self.balance.checked_add(coins_to_add as i64) {
                    Some(new_balance) => {
                        self.balance = new_balance;
                        Ok(())
                    }
                    None => {
                        Err(WalletError::AddCoinOverflowError {
                            current_amount: self.balance,
                            added_amount: coins_to_add,
                        })
                    }
                }
            }
            _ => Err(WalletError::AddCoinError)
        }
    }

    ///
    /// Remove coins to the wallet.
    ///
    /// # Arguments
    ///
    /// * `coins_to_remove` - A u32 number that represents the amount of coin to remove from the wallet.
    /// * `allow_negative_balance` - A boolean that represents if, after the remove operation, a negative balance is allowed.
    ///
    /// # Example
    ///
    /// ```
    /// # use banana_coin::model::Wallet;
    /// let mut wallet = Wallet::new(
    ///     100
    /// );
    /// # assert_eq!(wallet.get_balance(), 100);
    /// let result = wallet.remove_coins(100, true);
    /// match result {
    ///     Ok(_) => println!("Coins removed"),
    ///     Err(e) => println!("{}", e)
    /// }
    /// # assert_eq!(wallet.get_balance(), 0);
    /// ```
    ///
    pub fn remove_coins(
        self: &mut Wallet,
        coins_to_remove: u32,
        allow_negative_balance: bool,
    ) -> Result<(), WalletError> {
        // Checking for overflow
        match coins_to_remove {
            x if x == 0 => Err(WalletError::RemoveZeroCoinError),
            x if x > 0 => {
                match self.balance.checked_sub(coins_to_remove as i64) {
                    Some(total_coins) => {
                        if !allow_negative_balance && total_coins < 0 {
                            return Err(WalletError::RemoveCoinNegativeBalanceError {
                                current_amount: self.balance,
                                removed_amount: coins_to_remove,
                                negative_balance_allowed: allow_negative_balance,
                            })
                        }
                        self.balance = total_coins;
                        Ok(())
                    }
                    None => {
                        Err(WalletError::RemoveCoinOverflowError {
                            current_amount: self.balance,
                            removed_amount: coins_to_remove,
                        })
                    }
                }
            }
            _ => Err(WalletError::RemoveCoinError)
        }
    }
}