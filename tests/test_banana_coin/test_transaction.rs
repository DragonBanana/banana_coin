extern crate banana_coin;
use banana_coin::model::*;
use banana_coin::model::transaction_state::{TransactionState, TransactionState::*};

#[test]
fn new_transaction() {
    let mut history = Vec::new();
    history.push(Created {timestamp: 1});
    history.push(OnProcess {timestamp: 4});
    history.push(Completed {timestamp: 7});
    let transaction = Transaction::new(
        "id_0001".to_string(),
        "entity_001".to_string(),
        "entity_002".to_string(),
        100,
        "Moving 100 coins from entity identified by 'entity_001' to entity identified by 'entity_002'.".to_string(),
        Completed{timestamp: 7},
        history
    );
    assert_eq!(transaction.get_id(), "id_0001".to_string());
    assert_eq!(transaction.get_from_entity_id(), "entity_001".to_string());
    assert_eq!(transaction.get_to_entity_id(), "entity_002".to_string());
    assert_eq!(transaction.get_amount(), 100);
    assert_eq!(transaction.get_description(),
               "Moving 100 coins from entity identified by 'entity_001' to entity identified by 'entity_002'.".to_string());
    assert_eq!(transaction.get_current_state(), Completed { timestamp: 7 });
    assert!(transaction.get_state_history().contains(&Created {timestamp: 1}));
    assert!(transaction.get_state_history().contains(&OnProcess {timestamp: 4}));
    assert!(transaction.get_state_history().contains(&Completed {timestamp: 7}));


}