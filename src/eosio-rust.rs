// (C)2018 Ville Sundell, released under Apache License 2.0
// Based on Andrea Passaglia's wonderful article at: https://www.sales4k.com/blockchain/eos-contracts-rust-no-cpp/

#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

extern "C" {
    fn printn(n: u64);
    fn printui(u: u64);
    fn publication_time() -> u64;
}

const ACTION_NOW: u64 = 11328804862650482688; // Action "now"
const ACTION_ME: u64 = 10556437526556442624; // Action "me"
const ACTION_THIS: u64 = 14654009500021817344; // Action "this"
const ACTION_HELLO: u64 = 7684013976526520320; // Action "hello"
const HELLO_RUST: u64 = 7684013989323751424; // The "hello.rust" greeting as account name

#[no_mangle]
pub extern "C" fn init() {
    // This initialization function is empty for now
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    unsafe {
        match action {
            ACTION_NOW => printui(publication_time()),
            ACTION_ME => printn(receiver),
            ACTION_THIS => {
                printn(code);
                printn(action)
            }
            ACTION_HELLO => printn(HELLO_RUST),
            _ => printui(action), // You can use this fallback to convert from name to u64
        }
    }
}

#[panic_handler]
fn panic_fmt(_: &PanicInfo) -> ! {
    loop {}
}
