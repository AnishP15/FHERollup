#![no_main]
#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;

use stylus_sdk::stylus_proc::entrypoint;

// use tfhe::boolean::gen_keys;
// use tfhe::boolean::prelude::*;
use tfhe::integer::gen_keys_radix;

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

#[entrypoint]
fn user_main(client_key: Vec<u8>, server_key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
    let (mut client_key, mut server_key) = gen_keys();

    // We use the client secret key to encrypt two messages:
    let ct_1 = client_key.encrypt(false);
    let ct_2 = server_key.not(&ct_1);
    let output_1 = client_key.decrypt(&ct_2);
    // println!(output_1);
    assert_eq!(output_1, true);

    Ok(input)
}
