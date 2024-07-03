use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct Wallet {
    balance: u64,
}

static mut WALLETS: Option<HashMap<String, Wallet>> = None;

#[init]
fn init() {
    unsafe {
        WALLETS = Some(HashMap::new());
    }
}

#[update]
fn create_wallet(address: String) {
    unsafe {
        let wallets = WALLETS.as_mut().unwrap();
        wallets.insert(address, Wallet { balance: 0 });
    }
}

#[update]
fn send_tokens(from: String, to: String, amount: u64) -> Result<(), String> {
    unsafe {
        let wallets = WALLETS.as_mut().unwrap();
        let sender_wallet = wallets.get_mut(&from).ok_or("Sender wallet not found")?;
        let receiver_wallet = wallets.get_mut(&to).ok_or("Receiver wallet not found")?;
        
        if sender_wallet.balance < amount {
            return Err("Insufficient balance".into());
        }

        sender_wallet.balance -= amount;
        receiver_wallet.balance += amount;
        Ok(())
    }
}

#[query]
fn get_balance(address: String) -> u64 {
    unsafe {
        let wallets = WALLETS.as_ref().unwrap();
        let wallet = wallets.get(&address).unwrap();
        wallet.balance
    }
}
