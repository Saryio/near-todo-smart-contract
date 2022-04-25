use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, env, AccountId};


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskInfo{
    signer_id: AccountId,
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

    pub fn read_all(&self) -> &HashMap<String, TaskInfo>{
        &self.todos
    }

    pub fn read(&self, task: String) -> &TaskInfo{
        &self.todos.get(&task).unwrap()
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
//     use near_sdk::{MockedBlockchain, PublicKey, Gas};
//     use near_sdk::{testing_env, VMContext};

//     fn get_context(input: vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: AccountId::new_unchecked("alice.testnet".to_string()),
//             signer_account_id: AccountId::from("robert.testnet"),
//             signer_account_pk: PublicKey::from([0,1,2]),
//             predecessor_account_id: AccountId::from("jane.testnet"),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 150000,
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: Gas(10u64.pow(18)),
//             random_seed: [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 32, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32],
//             output_data_receivers: vec![],
//             epoch_height: 19,
//         }
//     }

//     #[test]
//     fn read() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Todos { todos: HashMap::new()};
//         contract.create("Some task here".to_string(), false);
//         let contract_task = contract.read().get("Some task here").unwrap();
//         assert_eq!(TaskInfo{signer_id: AccountId::from("alice.testnet"), done: false}, None);
//     }

//     #[test]
//     fn read_and_create() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Todos { todos: HashMap::new() };
//         contract.create("make a video call".to_string(), false);

//         assert_eq!(contract.read().get("make a video call").unwrap().done, false);
//     }

//     #[test]
//     fn update(){
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = Todos { todos: hashmap::new() };

//         contract.create("dar um tapa no psyke".to_string(), false);
//         assert_eq!(contract.read().get("dar um tapa no psyke").unwrap().done, false);
//         contract.update("dar um tapa no psyke".to_string(), true);
//         assert_eq!(contract.read().get("dar um tapa no psyke").unwrap().done, true);
//     }

//     #[test]
//     fn delete() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = todos { todos: hashmap::new() };
//         contract.create("drink watter".to_string(), true);
//         contract.delete("drink watter".to_string());
//     }
// }
