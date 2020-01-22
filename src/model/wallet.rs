use serde::{Deserialize, Serialize};

static ERROR_MESSAGE_ADD_COINS_OVERFLOW: &'static str = "Can not add coins to the wallet.";
static ERROR_MESSAGE_ADD_COINS_NEGATIVE_AMOUNT: &'static str = "Can not add a negative amount of coins to the wallet. Use remove_coins function";
static ERROR_MESSAGE_ADD_COINS_ZERO_AMOUNT: &'static str = "Can not add a zero to the wallet.";
static ERROR_MESSAGE_ADD_COINS: &'static str = "Add coins generic error.";


/// Wallet structure
/// It represents a wallet. It contains coins.
#[derive(Serialize, Deserialize)]
pub struct Wallet {
    /// Amount of coins contained in the wallet.
    /// It is a signed integer 'i64', so it can be positive and negative.
    pub n_coins: i64
}

impl Wallet {
    ///
    /// Return a new Wallet.
    ///
    /// # Arguments
    ///
    /// * `n_coins` - A i64 integer that represents the amount of coins in the wallet.
    ///
    /// # Examples
    ///
    /// let wallet = Wallet::new(
    ///     n_coins: 100,
    /// );
    ///
    pub fn new(
        n_coins: i64
    ) -> Wallet {
        Wallet {
            n_coins: n_coins
        }
    }

    ///
    /// Add coins to the wallet.
    ///
    /// # Arguments
    ///
    /// * `coins_to_add` - A i64 number that represents the amount of coin to add to the wallet.
    ///
    /// # Examples
    ///
    /// wallet.add_coins(100);
    ///
    pub fn add_coins(
        self: &mut Wallet,
        coins_to_add: i64,
    ) -> () {
        // Checking for overflow
        match coins_to_add {
            x if x < 0 => panic!(ERROR_MESSAGE_ADD_COINS_NEGATIVE_AMOUNT),
            x if x == 0 => panic!(ERROR_MESSAGE_ADD_COINS_ZERO_AMOUNT),
            x if x > 0 => {
                match self.n_coins.checked_add(coins_to_add) {
                    Some(total_coins) => {
                        self.n_coins = total_coins;
                    }
                    None => {
                        panic!(ERROR_MESSAGE_ADD_COINS_OVERFLOW)
                    }
                }
            }
            _ => panic!(ERROR_MESSAGE_ADD_COINS)
        }
    }
}