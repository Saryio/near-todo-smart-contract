use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Todos{
    todos: HashMap<String, bool>
}

#[near_bindgen]
impl Todos {
    pub fn create(&mut self, task: String, done: bool){
        self.todos.insert(task, done);
        // env::log(b"Task added successfully!");
    }

    pub fn read(self)-> HashMap<String, bool>{
        return self.todos
    }

    pub fn update(&mut self, task: String, done: bool){
        self.todos.remove(&task);
        self.create(task, done);
    }

    pub fn delete(&mut self, task: String){
        self.todos.remove(&task);
    }
}

