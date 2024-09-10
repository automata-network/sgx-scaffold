# Automata SGX Template
This is a template project for creating an SGX enclave with Rust. It shows how to build an enclave based on the [Automata SGX SDK](https://github.com/automata-network/automata-sgx-builder/tree/main), which makes it easier for developers to get started with SGX. The project contains the basic guide for the following scenarios:
- Call the code inside the enclave via ECALL
- Call the code outside the enclave via OCALL
- Use libraries inside the enclave
- Generate DCAP attestation report

## Project Structure
<pre>
├── <a href="./app/">app</a>: The main application
│ ├── <a href="./app/sgx">sgx</a>: Configurations for the enclave
│ │ ├── <a href="./app/sgx/config.xml">config.xml</a>: Developer defined parameters of the enclave
│ │ ├── <a href="./app/sgx/enclave.edl">enclave.edl</a>: Enclave Definition Language file defining the enclave interface
│ │ ├── <a href="./app/sgx/enclave.lds">enclave.lds</a>: Linker script for the enclave
│ │ └── <a href="./app/sgx/private.pem">private.pem</a>: Developer key used to sign the enclave, do not use this key to sign your enclave in production, please use your own key
│ ├── <a href="./app/src/main.rs">src/main.rs</a>: Main entrypoint for the application
│ └── <a href="./app/build.rs">build.rs</a>: Builder code used to build the application, you don't need change it
├── <a href="./enclave/">enclave</a>: The SGX enclave implementation
│ └── <a href="./enclave/src/lib.rs">lib.rs</a>: Main library file for the enclave implementation
├── <a href="./mock-lib/">mock-lib</a>: A mock library which is called by the enclave via OCALL
│ └── <a href="./mock-lib/src/lib.rs">lib.rs</a>: Main library file for the mock library implementation
</pre>

## Your First Enclave
In order to create your first enclave, you need to modify the `enclave/src/lib.rs` file to add your business logic. Do not forget to update the `app/sgx/enclave.edl` file if you need to update the ECALL interface.

When building the enclave, you can use other libraries just like writing a normal Rust program. Refer to the usage of `serde_json` in `enclave/src/lib.rs` as an example. 

In the case that you must call the library via OCALL, you can refer to the usage of `mock-lib` as an example. Basically, you should add the dependency of that library in `app` crate and declare it as an external crate in `app/src/main.rs` file. Then add the OCALL in `app/sgx/enclave.edl` file. After that, add the function declaration in `enclave/src/lib.rs` file.

Refer to the [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation) repo for more details about the verification ofDCAP attestation.

## Environment Setup


## Building the Enclave
In order to build the enclave, you need to have a sgx-supported machine.

### Build mannually
#### Clone the repository
```bash
git clone https://github.com/automata-network/sgx-template.git
cd sgx-template
```
You can click the `Use this template` button to create a new repository.

#### Install cargo-sgx
```bash
cargo install cargo-sgx
```
Once you have installed `cargo-sgx`, you can check the help menu to see the available commands.
```bash
cargo sgx --help
```

#### Build the Enclave
```bash
cargo sgx build
```
or you can run the enclave directly
```bash
cargo sgx run
```
You can find the executable file in `./target/debug` or `./target/release` directory.

### Build with Docker
