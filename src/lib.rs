// lib.rs
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::app;


#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct HelloApp {}

#[app::logic]
impl HelloApp {
    #[app::init]
    pub fn init() -> Self {
        HelloApp {}
    }

    pub fn say_hello(&self) -> String {
        "Hello from Calimero Node!".to_string()
    }
}
