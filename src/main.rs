extern crate banana_coin;
use banana_coin::model::*;
use banana_coin::model::TransactionState::*;

fn main() {
    let mut entity : Entity = Entity::new(
        "Ciao",
        "Name",
        Wallet::new(
            0
        )
    );

    entity.add_coins(100);

    let result = serde_json::to_string_pretty(&entity);
    match result {
        Ok(e) => {
            println!("{}", e)
        }
        Err(e) => {
            println!("{}", e)
        }
    }
    let mut vec = Vec::new();
    vec.push(Created {timestamp: 1});
    vec.push(OnProcess {timestamp: 4});
    vec.push(Completed {timestamp: 7});
    let transaction = Transaction::new(
        "id_0001".to_string(),
        "entity_001".to_string(),
        "entity_002".to_string(),
        100,
        "Moving 100 coins from entity identified by 'entity_001' to entity identified by 'entity_002'.".to_string(),
        Completed{timestamp: 7},
        vec
    );
    let result = serde_json::to_string_pretty(&transaction);
    match result {
        Ok(e) => {
            println!("{}", e)
        }
        Err(e) => {
            println!("{}", e)
        }
    }
    println!("Hello, world!");
}
