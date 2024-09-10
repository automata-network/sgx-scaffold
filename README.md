# Automata SGX Template

This is a template project for creating an SGX enclave with Rust. It uses the [`automata-sgx-builder`](https://github.com/automata-network/automata-sgx-builder/tree/main) to build the enclave.

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

## Building the Enclave
In order to build the enclave, you need to have a sgx-supported machine.
### Clone the repository
```bash
git clone https://github.com/automata-network/sgx-template.git
cd sgx-template
```
You can click the `Use this template` button to create a new repository.

### Install cargo-sgx
```bash
cargo install cargo-sgx
```
Once you have installed `cargo-sgx`, you can check the help menu to see the available commands.
```bash
cargo sgx --help
```

### Build the Enclave
#### Build manually
```bash
cargo sgx build
```
or you can run the enclave directly
```bash
cargo sgx run
```
You can find the executable file in `./target/debug` or `./target/release` directory.