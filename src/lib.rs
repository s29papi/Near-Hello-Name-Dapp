

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();
       
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloContract {
    greeting: String,
}

#[near_bindgen]
impl HelloContract  {
    pub fn get_greeting(&self) -> &str  {
        return &self.greeting;
    }

     pub fn input_name(&mut self, name: String) {
        let mut s = String::from("hello, ");

        s.push_str(&name);

        self.greeting =  s;

        let log_message = format!("Created greeting with your name {}", self.greeting);

        env::log(log_message.as_bytes());
    }
}

#[cfg(test)]
mod tests {

  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};
  fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
     VMContext {
         current_account_id: "alice.testnet".to_string(),
         signer_account_id: "robert.testnet".to_string(),
         signer_account_pk: vec![0, 1, 2],
         predecessor_account_id: "jane.testnet".to_string(),
         input,
         block_index: 0,
         block_timestamp: 0,
         account_balance: 0,
         account_locked_balance: 0,
         storage_usage: 0,
         attached_deposit: 0,
         prepaid_gas: 10u64.pow(18),
         random_seed: vec![0, 1, 2],
         is_view,
         output_data_receivers: vec![],
         epoch_height: 19,
     }
 }

 // mark individual unit tests with #[test] for them to be registered and fired
 #[test]
 fn create_value_and_get_value() {
     // set up the mock context into the testing environment
     let context = get_context(vec![], false);
     testing_env!(context);
     // instantiate a contract variable with the greeting equal to hello, world!
     let mut contract = HelloContract{ greeting: String::from("hello, world!") };
     contract.input_name("Bieber!".to_string());
     println!("Greeting after calling create_value(func): {}", contract.get_greeting());
     // confirm that we are greeting the right person!
     assert_eq!("hello, Bieber!", contract.get_greeting());
 }
}
