use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::{update, query};
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
}

#[update]
fn add_value(n: u64) -> u64 {
    COUNTER.with(|counter| {
        let mut count = counter.borrow_mut();
        *count += n;
        *count
    })
}

#[query]
fn get_value() -> u64 {
    COUNTER.with(|counter| *counter.borrow())
}