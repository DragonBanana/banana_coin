use serde::{Deserialize, Serialize};
use crate::model::transaction_state::TransactionState;

/// Transaction structure
/// It represents a transaction between two entities. It is identified by an identifier 'id'.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Transaction {
    id: String,
    from_entity_id: String,
    to_entity_id: String,
    amount: u32,
    description: String,
    current_state: TransactionState,
    state_history: Vec<TransactionState>,
}

/// Transaction implementation.
impl Transaction {
    ///
    /// Return a new Transaction.
    ///
    /// # Arguments
    ///
    /// * `id` - A String that represents the id of the transaction.
    /// * `from_entity_id` - A String that represents the id of the entity from which the coins are withdrawn.
    /// * `to_entity_id` - A String that represents the id of the entity to which the coins are deposited.
    /// * `amount`: A i64 integer that represents the amount of coins transferred from 'from_entity_id' to 'to_entity_id' in the transaction.
    /// * `description` A String that represents the textual description of the transaction.
    /// * `current_state` A TransactionState that represents the current state of the transaction.
    /// * `state_history`: A Vec<TransactionState> that represents the history of the states of the transaction.
    ///
    /// # Examples
    ///
    /// let mut history = Vec::new();
    /// history.push(Created {timestamp: 1});
    /// history.push(OnProcess {timestamp: 4});
    /// history.push(Completed {timestamp: 7});
    /// let transaction = Transaction::new(
    ///     "id_0001".to_string(),
    ///     "entity_001".to_string(),
    ///     "entity_002".to_string(),
    ///     100,
    ///     "Moving 100 coins from entity identified by 'entity_001' to entity identified by 'entity_002'.".to_string(),
    ///     Completed{timestamp: 7},
    ///     history
    ///     );
    ///
    pub fn new(
        id: String,
        from_entity_id: String,
        to_entity_id: String,
        amount: u32,
        description: String,
        current_state: TransactionState,
        state_history: Vec<TransactionState>,
    ) -> Transaction {
        Transaction {
            id: id,
            from_entity_id: from_entity_id,
            to_entity_id: to_entity_id,
            amount: amount,
            description: description,
            current_state: current_state,
            state_history: state_history,
        }
    }

    ///
    /// Return the id of the transaction.
    ///
    /// # Examples
    ///
    /// let transaction_id: String = transaction.get_id();
    ///
    pub fn get_id(
        self: &Transaction
    ) -> String {
        return self.id.clone();
    }

    ///
    /// Return the id of the entity from which the coins are withdrawn.
    ///
    /// # Examples
    ///
    /// let transaction_from_id: String = transaction.get_from_entity_id();
    ///
    pub fn get_from_entity_id(
        self: &Transaction
    ) -> String {
        return self.from_entity_id.clone();
    }

    ///
    /// Return the id of the entity to which the coins are deposited.
    ///
    /// # Examples
    ///
    /// let transaction_to_id: String = transaction.get_to_entity_id();
    ///
    pub fn get_to_entity_id(
        self: &Transaction
    ) -> String {
        return self.to_entity_id.clone();
    }

    ///
    /// Return the amount of coins involved in the transaction.
    ///
    /// # Examples
    ///
    /// let transaction_amount: u32 = transaction.get_amount();
    ///
    pub fn get_amount(
        self: &Transaction
    ) -> u32 {
        return self.amount.clone();
    }

    ///
    /// Return the description of the transaction.
    ///
    /// # Examples
    ///
    /// let transaction_description: String = transaction.get_description();
    ///
    pub fn get_description(
        self: &Transaction
    ) -> String {
        return self.description.clone();
    }

    ///
    /// Return the current state of the transaction.
    ///
    /// # Examples
    ///
    /// let transaction_current_state: TransactionState = transaction.get_current_state();
    ///
    pub fn get_current_state(
        self: &Transaction
    ) -> TransactionState {
        return self.current_state.clone();
    }

    ///
    /// Return the history of the states of the transaction.
    ///
    /// # Examples
    ///
    /// let transaction_state_history: Vec<TransactionState> = transaction.get_state_history();
    ///
    pub fn get_state_history(
        self: &Transaction
    ) -> Vec<TransactionState> {
        return self.state_history.clone();
    }

}