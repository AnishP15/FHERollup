// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::*};

// use tfhe::boolean::gen_keys;
// use tfhe::boolean::prelude::*;
use tfhe::integer::*;
use tfhe::prelude::FheTrivialEncrypt;
use tfhe::shortint::prelude::PARAM_MESSAGE_2_CARRY_2_KS_PBS;
use tfhe::FheUint32;

use serde::{Serialize, Deserialize};
// use tfhe::integer::ciphertext::BaseRadixCiphertext;

/// The solidity_storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[solidity_storage]
/// The entrypoint macro defines where Stylus execution begins. External methods
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct Counter {
  count: StorageString,
  server_key: StorageString,
}


#[external]
impl Counter {
  /// Sets the count in storage to a user-specified value.
  pub fn set_serverkey(&mut self, server_key_string: String) -> Result<(), Vec<u8>> {
    self.server_key.set_str(server_key_string);
    Ok(())
  }
    
//   /// Gets the number from storage.
//   pub fn get(&self) -> Result<U256, Vec<u8>> {
//     let ct_1 = 
//     Ok(self.count.get())
//   }

//   /// Sets the count in storage to a user-specified value.
  pub fn set_count(&mut self, count_ct: String) -> Result<(), Vec<u8>> {
    self.count.set_str(count_ct);
    Ok(())
  }

  /// Increments count by 1
  pub fn increment(&mut self) -> Result<(), Vec<u8>> {
    // let number_of_blocks = 4;
    let (mut client_key, mut server_key) = gen_keys_radix(PARAM_MESSAGE_2_CARRY_2_KS_PBS, number_of_blocks);
    // let msg1: u64 = 153;
    // let ct_1 = client_key.encrypt(msg1);
    let serialized = self.count.get_string();
    let ct_1: ciphertext::BaseRadixCiphertext<Ciphertext> = serde_json::from_str(&serialized).unwrap();
    let trivial = FheUint32::encrypt_trivial(1);
    let ct_3 = server_key.unchecked_add(&ct_1, &trivial);

    let ct_s: String = serde_json::to_string(&ct_1).unwrap();
    self.count.set_str(ct_s);
    Ok(())
  }
}
