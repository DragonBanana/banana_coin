use serde::{Deserialize, Serialize};
use super::wallet::Wallet;

/// Entity structure
/// It represents a person or an organization. It is identified by its id.
#[derive(Serialize, Deserialize)]
pub struct Entity {
    id: String,
    name: String,
    wallet: Wallet,
}

impl Entity {
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
    /// let entity = Entity::new(
    ///     "id_0001".to_string(),
    ///     "john".to_string(),
    ///     Wallet::new(
    ///         0
    ///     ),
    /// );
    ///
    pub fn new(
        id: String,
        name: String,
        wallet: Wallet,
    ) -> Entity {
        Entity {
            id: id,
            name: name,
            wallet: wallet,
        }
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
    /// entity.add_coins(100);
    ///
    pub fn add_coins(
        self : &mut Entity,
        coins_to_add: i64,
    ) -> () {
        self.wallet.add_coins(coins_to_add);
    }
}

#[cfg(test)]
mod test_entity {
    use super::*;
    #[test]
    fn new_entity() {
        let entity = Entity::new(
            "id_0001".to_string(),
            "john".to_string(),
            Wallet::new(
                0
            ),
        );
        assert_eq!(entity.id, "id_0001".to_string());
        assert_eq!(entity.name, "john".to_string());
        assert_eq!(entity.wallet.n_coins, 0)
    }
}