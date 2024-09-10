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
├── <a href="./untrusted_lib/">untrusted_lib</a>: The untrusted library used by the enclave
│ └── <a href="./untrusted_lib/src/lib.rs">lib.rs</a>: Main library file for the untrusted library implementation
</pre>

## Building the Enclave
### Clone the repository
```bash
git clone https://github.com/automata-network/automata-sgx-template.git
cd automata-sgx-template
```

### Install cargo-sgx
```bash
cargo install cargo-sgx
```
Once you have installed `cargo-sgx`, you can check the help menu to see the available commands.
```bash
cargo sgx --help
```

### Build the Enclave
#### Build on local machine
```bash
cargo sgx build --std
```
or you can run the enclave directly
```bash
cargo sgx run --std
```

#### Build on sgx-supported machine
```bash
cargo sgx build
```
You can find the executable file in `./target/debug` or `./target/release` directory.