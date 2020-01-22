use banana_coin::*;

use std::string::ToString;

#[test]
fn new_entity() {
    let entity = Entity::new(
        "id_0001".to_string(),
        "john".to_string(),
        Wallet::new(
            0
        ),
    );
}