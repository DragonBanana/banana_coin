use serde::{Deserialize, Serialize};
use crate::model::transaction_state::TransactionState::*;

/// Transaction state enumeration
/// It represents the state of a transaction. Each state has the associated timestamp.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum TransactionState {
    Created {timestamp : u64},
    OnProcess {timestamp : u64},
    Completed {timestamp : u64},
    Failed {timestamp : u64},
    Blocked {timestamp : u64},
}


impl PartialEq for TransactionState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Created {timestamp : a}, Created {timestamp : b}) => a == b,
            (OnProcess {timestamp : a}, OnProcess {timestamp : b}) => a == b,
            (Completed {timestamp : a}, Completed {timestamp : b}) => a == b,
            (Failed {timestamp : a}, Failed {timestamp : b}) => a == b,
            (Blocked {timestamp : a}, Blocked {timestamp : b}) => a == b,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}