#![feature(c_unwind)]

use std::sync::atomic::{AtomicI32, Ordering};

static COUNTER: AtomicI32 = AtomicI32::new(0);

struct WithCounter;

impl WithCounter {
    pub fn new() -> WithCounter {
        COUNTER.fetch_add(1, Ordering::SeqCst);
        WithCounter
    }
}
impl Drop for WithCounter {
    fn drop(&mut self) {
        COUNTER.fetch_sub(1, Ordering::SeqCst);
    }
}

#[no_mangle]
pub extern "C" fn current() -> i32 {
    COUNTER.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C-unwind" fn call_with_counter(func: extern "C-unwind" fn(*mut ()) -> *mut (), data: *mut ()) -> *mut () {
    let _with = WithCounter::new();
    func(data)
}
