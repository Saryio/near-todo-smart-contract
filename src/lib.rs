use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, env};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskInfo{
    signer_id: String,
    done: bool
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Todos{
    todos: HashMap<String, TaskInfo>
}
#[near_bindgen]
impl Todos {

    pub fn create(&mut self, task: String, done: bool){
        self.todos.insert(task, TaskInfo{
            signer_id: env::signer_account_id(),
            done
        });
    }

    pub fn read(&self) -> &HashMap<String, TaskInfo>{
        &self.todos
    }

    pub fn update(&mut self, task: String, done: bool){
        self.todos.remove(&task);
        self.create(task, done);
    }

    pub fn delete(&mut self, task: String){
        self.todos.remove(&task);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: "alice.testnet".to_string(),
//             signer_account_id: "robert.testnet".to_string(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: "jane.testnet".to_string(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 150000,
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 19,
//         }
//     }

//     #[test]
//     fn test(){
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         // let contract = Todos { todos: HashMap::new() };
//         env::log(b"testetetetetete")
//     }

//     #[test]
//     fn read_and_create() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Todos { todos: HashMap::new() };
//         contract.create("Make a video call".to_string(), false);

//         assert_eq!(contract.read().get("Make a video call").unwrap().done, false);
//     }

//     #[test]
//     fn update(){
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Todos { todos: HashMap::new() };

//         contract.create("Dar um tapa no psyke".to_string(), false);
//         assert_eq!(contract.read().get("Dar um tapa no psyke").unwrap().done, false);
//         contract.update("Dar um tapa no psyke".to_string(), true);
//         assert_eq!(contract.read().get("Dar um tapa no psyke").unwrap().done, true);
//     }

//     #[test]
//     fn delete() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Todos { todos: HashMap::new() };
//         contract.create("Drink watter".to_string(), true);
//         contract.delete("Drink watter".to_string());
//     }
// }
