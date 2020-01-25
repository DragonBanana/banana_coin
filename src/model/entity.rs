use serde::{Deserialize, Serialize};

use crate::error::EntityError;
use crate::model::Wallet;

/// Entity structure
/// It represents a person or an organization. It is identified by its id.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Entity<'a> {
    id: &'a str,
    name: &'a str,
    wallet: Wallet,
}

impl<'a> Entity<'a> {
    ///
    /// Return a new Entity.
    ///
    /// # Arguments
    ///
    /// * `id` - A String that represents the id of the entity.
    /// * `name` - A String that represents the name of the entity.
    /// * `wallet` - A Wallet that represents the wallet of the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use banana_coin::model::{Entity, Wallet};
    /// let entity = Entity::new(
    ///     "id_0001",
    ///     "john",
    ///     Wallet::new(
    ///         0
    ///     )
    /// );
    /// # assert_eq!(entity.get_id(), "id_0001");
    /// # assert_eq!(entity.get_name(), "john");
    /// # assert_eq!(entity.get_wallet().get_balance(), 0)
    /// ```
    ///
    pub fn new(
        id: &'a str,
        name: &'a str,
        wallet: Wallet,
    ) -> Entity<'a> {
        Entity {
            id: id,
            name: name,
            wallet: wallet,
        }
    }

    ///
    /// Return a copy of the id string of the entity.
    ///
    /// # Example
    /// ```
    /// # use banana_coin::model::{Entity, Wallet};
    /// let entity = Entity::new(
    ///     "id_0001",
    ///     "john",
    ///     Wallet::new(
    ///         0
    ///     )
    /// );
    /// let entity_id : &str = entity.get_id();
    /// # assert_eq!(entity_id, "id_0001")
    /// ```
    ///
    pub fn get_id(
        self: &Entity <'a>,
    ) -> &str {
        self.id.clone()
    }

    ///
    /// Return a copy of the name string of the entity.
    ///
    /// # Example
    /// ```
    /// # use banana_coin::model::{Entity, Wallet};
    /// let entity = Entity::new(
    ///     "id_0001",
    ///     "john",
    ///     Wallet::new(
    ///         0
    ///     )
    /// );
    /// let entity_name : &str = entity.get_name();
    /// # assert_eq!(entity_name, "john")
    /// ```
    ///
    pub fn get_name(
        self: &Entity <'a>,
    ) -> &str {
        self.name.clone()
    }

    ///
    /// Return a copy of the wallet struct of the entity.
    ///
    /// # Example
    /// ```
    /// # use banana_coin::model::{Entity, Wallet};
    /// let entity = Entity::new(
    ///     "id_0001",
    ///     "john",
    ///     Wallet::new(
    ///         0
    ///     )
    /// );
    ///
    /// //The returned wallet is a copy of the current one
    /// let mut entity_wallet : Wallet = entity.get_wallet();
    ///
    /// entity_wallet.add_coins(100);
    ///
    /// assert_eq!(entity.get_wallet().get_balance(), 0);
    /// assert_eq!(entity_wallet.get_balance(), 100);
    /// ```
    ///
    pub fn get_wallet(
        self: &Entity <'a>,
    ) -> Wallet {
        self.wallet.clone()
    }

    ///
    /// Add coins to the wallet of the entity.
    ///
    /// # Arguments
    ///
    /// * `coins_to_add` - A i64 number that represents the amount of coin to add to the wallet.
    ///
    /// # Examples
    ///
    /// ```
    /// # use banana_coin::model::{Entity, Wallet};
    /// let mut entity = Entity::new(
    ///     "id_0001",
    ///     "john",
    ///     Wallet::new(
    ///         0
    ///     )
    /// );
    /// let result = entity.add_coins(100);
    /// # match result {
    /// #    Ok(_) => assert!(true),
    /// #    Err(_) => assert!(true)
    /// # }
    /// ```
    ///
    pub fn add_coins(
        self: &mut Entity<'a>,
        coins_to_add: u32,
    ) -> Result<(), EntityError> {
        match self.wallet.add_coins(coins_to_add) {
            Ok(_) => Ok(()),
            Err(error) => Err(EntityError::WalletOperationError { error })
        }
    }

    ///
    /// Remove coins to the wallet of the entity.
    ///
    /// # Arguments
    ///
    /// * `coins_to_remove` - A i64 number that represents the amount of coin to remove from the wallet.
    /// * `allow_negative_balance` - A boolean that represents if, after the remove operation, a negative wallet balance is allowed.
    ///
    /// # Example
    ///
    /// ```
    /// # use banana_coin::model::{Entity, Wallet};
    /// let mut entity = Entity::new(
    ///     "id_0001",
    ///     "john",
    ///     Wallet::new(
    ///         0
    ///     )
    /// );
    /// let result = entity.remove_coins(100, true);
    /// # match result {
    /// #    Ok(_) => assert!(true),
    /// #    Err(_) => assert!(true)
    /// # }
    /// ```
    ///
    pub fn remove_coins(
        self: &mut Entity<'a>,
        coins_to_add: u32,
        allow_negative_balance: bool,
    ) -> Result<(), EntityError> {
        match self.wallet.remove_coins(coins_to_add, allow_negative_balance) {
            Ok(_) => Ok(()),
            Err(error) => Err(EntityError::WalletOperationError { error })
        }
    }
}

