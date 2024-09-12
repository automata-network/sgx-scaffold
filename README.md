# Automata SGX Template
This is a template project for creating an SGX enclave with Rust. It shows how to build an enclave based on the [Automata SGX SDK](https://github.com/automata-network/automata-sgx-sdk/tree/main), which makes it easier for developers to get started with SGX. The project contains the basic guide for the following scenarios:
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

Refer to the [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation) repo for more details about verification of the DCAP attestation.

## Environment Setup


## Building the Enclave
### Prerequisites
In order to build the enclave, you need to have a sgx-supported machine.

If you have a machine with SGX support, please check the version of your SGX and DCAP SDK. The latest version supported by Automata SGX SDK can be found [here](https://github.com/automata-network/automata-sgx-sdk/tree/main).

If you don't have a machine with SGX support, we recommend you to create a [`DCsv3`](https://learn.microsoft.com/en-us/azure/virtual-machines/sizes/general-purpose/dcsv3-series?tabs=sizebasic) instance in Azure. You can refer to the [Dockerfile](./docker/) for installing the SGX and DCAP SDK or use docker to build the enclave.

### Build mannually
> You need to have a sgx-supported machine with SGX and DCAP SDK installed to build the enclave manually.
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

#### Generate new signing key

```bash
cargo sgx gen-key app/sgx/private.pem
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

> You need to have a sgx-supported machine to build the enclave with docker. Make sure you got the docker and docker-compose installed.

Build image for ubuntu 20.04
```bash
$ cd docker/20.04
$ docker compose build
```

We also have the prebuilt docker image in [here](https://github.com/automata-network/sgx-template/pkgs/container/sgx-template)

#### Run with Docker

```bash
$ cd docker/20.04
$ docker compose run sgx-template
```
