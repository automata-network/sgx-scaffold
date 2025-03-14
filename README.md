<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata SGX Scaffold
[![Automata SGX SDK](https://img.shields.io/badge/Powered%20By-Automata%20SGX%20SDK-orange.svg)](https://github.com/automata-network/automata-sgx-sdk)

This is a scaffold for creating an SGX enclave with Rust. It shows how to build an enclave based on the [Automata SGX SDK](https://github.com/automata-network/automata-sgx-sdk), which makes it easier for developers to get started with SGX. The project contains the basic guide for the following scenarios:
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
Following the steps below to create your first enclave.

1. Modify the `enclave/src/lib.rs` file to add your business logic. You can use other libraries just like writing a normal Rust program. Refer to the usage of `serde_json` as an example. 
2. Update the `app/sgx/enclave.edl` file if you need to change the ECALL interface or add new ECALLs.
3. Refer to the usage `mock-lib` if you want to use libraries via OCALL. For example, you need to use a library that use instructions not allowed(such as CPUID or GETSEC) in enclave.

Refer to the [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation) repo for more details about verification of the DCAP attestation.

## Building the Enclave
### Prerequisites
In order to build the enclave, you need to have a sgx-supported machine.

If you have a machine with SGX support, please check the version of your SGX and DCAP SDK. The latest version supported by Automata SGX SDK can be found [here](https://github.com/automata-network/automata-sgx-sdk/tree/main).

If you don't have a machine with SGX support, we recommend you to create a [`DCsv3`](https://learn.microsoft.com/en-us/azure/virtual-machines/sizes/general-purpose/dcsv3-series?tabs=sizebasic) instance in Azure. Please refer to the [docker](./docker/) folder for the list of supported systems and create the instance using one of these systems. You can either install the SGX and DCAP SDK manually by following the steps outlined in the Dockerfile, or alternatively, you can use Docker to build and run the enclave directly.

### Build manually
> You need to have a sgx-supported machine with SGX and DCAP SDK installed to build the enclave manually.
#### Clone the repository
```bash
git clone https://github.com/automata-network/sgx-scaffold.git
cd sgx-scaffold
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
$ cd docker/ubuntu-20.04
$ docker compose build
```

Build image for ubuntu 22.04
```bash
$ cd docker/ubuntu-22.04
$ docker compose build
```

We also have the prebuilt docker image in [here](https://github.com/automata-network/sgx-scaffold/pkgs/container/sgx-scaffold)

#### Run with Docker

Run image for ubuntu 20.04
```bash
$ cd docker/ubuntu-20.04
$ docker compose run sgx-scaffold
```

Run image for ubuntu 22.04
```bash
$ cd docker/ubuntu-22.04
$ docker compose run sgx-scaffold
```
