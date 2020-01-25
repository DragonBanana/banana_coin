extern crate banana_coin;

use banana_coin::model::*;
use banana_coin::model::transaction_state::{TransactionState, TransactionState::*};

#[test]
fn equality() {
    assert!(Created { timestamp: 1 } == Created { timestamp: 1 });
    assert!(OnProcess { timestamp: 2 } == OnProcess { timestamp: 2 });
    assert!(Completed { timestamp: 3 } == Completed { timestamp: 3 });
    assert!(Failed { timestamp: 4 } == Failed { timestamp: 4 });
    assert!(Blocked { timestamp: 5 } == Blocked { timestamp: 5 });
}

#[test]
fn inequality_state() {
    assert!(Created { timestamp: 1 } != OnProcess { timestamp: 1 });
    assert!(OnProcess { timestamp: 2 } != Completed { timestamp: 2 });
    assert!(Completed { timestamp: 3 } != Failed { timestamp: 3 });
    assert!(Failed { timestamp: 4 } != Blocked { timestamp: 4 });
    assert!(Blocked { timestamp: 5 } != Created { timestamp: 5 });
}

#[test]
fn inequality_timestamp() {
    assert!(Created { timestamp: 1 } != Created { timestamp: 2 });
    assert!(OnProcess { timestamp: 2 } != OnProcess { timestamp: 3 });
    assert!(Completed { timestamp: 3 } != Completed { timestamp: 4 });
    assert!(Failed { timestamp: 4 } != Failed { timestamp: 5 });
    assert!(Blocked { timestamp: 5 } != Blocked { timestamp: 6 });
}