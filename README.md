# Moonlight OS - Rust Nightly Development Setup

Welcome to the Moonlight OS Rust Development Setup! This repository provides you with instructions to set up a development environment for Moonlight OS, an operating system built using the Rust programming language. By following the steps below, you'll be ready to contribute to and experiment with Moonlight OS, as it takes shape in its early development stages.

## Getting Started

These instructions will help you quickly set up your Moonlight OS development environment. Before you begin, ensure you have Rust and Cargo installed on your system. If not, you can install them using the official Rustup tool by following the instructions at [https://rustup.rs/](https://rustup.rs/).

### Prerequisites

- Rustup: [https://rustup.rs/](https://rustup.rs/)

## Setting up Nightly Rust and Moonlight OS

1. Open a terminal window.

2. Install the nightly Rust toolchain by entering the following command:

   ```shell
   rustup toolchain install nightly

3. Set the nightly toolchain as the default for Moonlight OS development:

   ```shell
   rustup override set nightly

4. Add the necessary LLVM tools to your Rust installation:

   ```shell
   rustup component add llvm-tools
   
## Building and Running Moonlight OS

This section guides you through the process of building and running Moonlight OS using the nightly Rust toolchain.

1. Install the bootimage tool, which is required to create a bootable disk image for Moonlight OS:

   ```shell
   cargo install bootimage
   
2. Build and run MoonlightOS using the following command;

   ```shell
   cargo run
   
## Contributing
We welcome contributions to the Moonlight OS project! If you encounter any issues, have ideas for improvements, or want to contribute to the development of Moonlight OS, please feel free to open an issue 
or create a pull request on the official repository.
