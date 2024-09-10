
extern crate untrusted_lib;

use automata_sgx_builder::types::SgxStatus;

//Micro enclave definition. Used by the automata_sgx_builder.
automata_sgx_builder::enclave! {
    name: Enclave,
    ecall: {
        fn trusted_execution() -> SgxStatus;
    }
}

/**
 * This is the entry point of the app.
 * It creates a new enclave(in debug mode) and calls the trusted_execution function.
 */
fn main() {
    println!("=============== Starting the app =================");
    let result = Enclave::new(true).unwrap().trusted_execution().unwrap();
    assert!(result.is_success());
    println!("=============== End of the app =================");
}