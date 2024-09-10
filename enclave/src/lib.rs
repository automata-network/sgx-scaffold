use automata_sgx_builder::types::SgxStatus;
use untrusted_lib::untrusted_execution;

/**
 * This is an ECALL function defined in the edl file.
 * It will be called by the application.
 */
#[no_mangle]
pub unsafe extern "C" fn trusted_execution() -> SgxStatus {
    println!("=============== Trusted execution =================");
    // Mock a random number
    let random_number = 4;
    println!("Generated random number: {}", random_number);
    println!("=============== End of trusted execution =================");
    // Call the untrusted function via OCALL
    untrusted_execution(random_number);
    // The following code is used to generate an attestation report
    // Must be run on sgx-supported machine
    // let data = [0u8; 64];
    // let attestation = automata_sgx_builder::dcap::dcap_quote(data);
    // match attestation {
    //     Ok(attestation) => {
    //         println!("Attestation successful: {:?}", attestation);
    //         SgxStatus::Success
    //     }
    //     Err(e) => {
    //         println!("Generating attestation failed: {:?}", e);
    //         SgxStatus::Unexpected
    //     }
    // }
    SgxStatus::Success
}