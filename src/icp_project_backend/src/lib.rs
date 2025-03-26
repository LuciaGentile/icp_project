use candid::CandidType;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<i64> = RefCell::new(0);
}

#[init]
fn init() {
    COUNTER.with(|counter| {
        *counter.borrow_mut() = CanisterState { somma_totale: 0 };
    });
}

#[update]
fn add_value(n: i64) -> i64 {
    COUNTER.with(|counter| {
        let mut count = counter.borrow_mut();
        *count += n;
        *count
    })
}

#[query]
fn get_value() -> i64 {
    COUNTER.with(|counter| *counter.borrow())
}