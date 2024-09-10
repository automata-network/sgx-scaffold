// Declare the mock_lib as an external crate, since it contains OCALL which will be called by the enclave.
// The automata_sgx_builder will link the `untrusted_execution` OCALL to the mock_lib.
extern crate mock_lib;

use automata_sgx_builder::types::SgxStatus;

//Enclave definition. Used by the automata_sgx_builder.
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