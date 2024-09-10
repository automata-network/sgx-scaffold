#[no_mangle]
pub fn untrusted_execution(random_number: i32) {
    println!("=============== Untrusted execution =================");
    println!("Received random number: {}", random_number);
    println!("=============== End of untrusted execution =================");
}