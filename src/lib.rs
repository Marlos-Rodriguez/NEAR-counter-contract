use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Counter {
    value: i32,
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i32 {
        self.value
    }

    pub fn increase(&mut self) {
        self.value += 1;
        let log_message = format!("Value Increased to {}", self.value);

        env::log_str(log_message.as_str());
    }

    pub fn decrease(&mut self) {
        self.value -= 1;
        let log_message = format!("Value decrease to {}", self.value);

        env::log_str(log_message.as_str());
    }

    pub fn reset(&mut self) {
        self.value = 0;
        let log_message = format!("Value reset to {}", self.value);

        env::log_str(log_message.as_str());
    }
}

// Unit Test
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::new_unchecked("robert.testnet".to_string()))
            .is_view(is_view)
            .build()
    }

    #[test]
    fn increase() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = Counter { value: 0 };
        contract.increase();

        println!("Value afther increase {}", contract.value);

        assert_eq!(contract.get_num(), 1)
    }

    #[test]
    fn decrease() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = Counter { value: 0 };
        contract.decrease();

        println!("Value afther decrease {}", contract.value);

        assert_eq!(contract.get_num(), -1)
    }

    #[test]
    fn reset() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = Counter { value: 10 };
        println!("Value before reset {}", contract.value);

        contract.reset();

        println!("Value afther reset {}", contract.value);

        assert_eq!(contract.get_num(), 0)
    }
}
