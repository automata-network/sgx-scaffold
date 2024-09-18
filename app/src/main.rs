// Declare the mock_lib as an external crate, since it contains OCALL which will be called by the enclave.
// The automata_sgx_sdk will link the `untrusted_execution` OCALL to the mock_lib.
extern crate mock_lib;

use automata_sgx_sdk::types::SgxStatus;

//Enclave definition. Used by the automata_sgx_sdk.
automata_sgx_sdk::enclave! {
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
    let result = Enclave::new().trusted_execution().unwrap();
    if !result.is_success() {
        println!("{:?}", result);
    }
    println!("=============== End of the app =================");
}