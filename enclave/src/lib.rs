use automata_sgx_sdk::types::SgxStatus;
// For most of the cases, you can use the external library directly.
use serde_json::json;

// Declare the OCALL function. The automata_sgx_sdk will link the OCALL to the mock_lib.
extern "C" {
    fn untrusted_execution(random_number: i32);
}

/**
 * This is an ECALL function defined in the edl file.
 * It will be called by the application.
 */
#[no_mangle]
pub unsafe extern "C" fn trusted_execution() -> SgxStatus {
    println!("=============== Trusted execution =================");
    // Use serde_json to serialize a JSON object
    let json_object = json!({
        "sgx": "hello"
    });
    println!("Serialized JSON object: {}", json_object);
    // Mock a random number
    let random_number = 4;
    println!("Generated random number: {}", random_number);
    // Call the untrusted function via OCALL
    untrusted_execution(random_number);

    println!("=============== Back to Trusted execution =================");
    // The following code is used to generate an attestation report
    // Must be run on sgx-supported machine
    let data = [0u8; 64];
    let attestation = automata_sgx_sdk::dcap::dcap_quote(data);
    let result = match attestation {
        Ok(attestation) => {
            println!("DCAP attestation: 0x{}", hex::encode(&attestation));
            SgxStatus::Success
        }
        Err(e) => {
            println!("Generating attestation failed: {:?}", e);
            SgxStatus::Unexpected
        }
    };
    println!("=============== End of trusted execution =================");

    result
}
