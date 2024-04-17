# MintNFT

This repository contains a Smart Contract for minting NFTs. Below you will find instructions on how to clone and test the project.

## Prerequisites

Before you begin, make sure you have the following installed:
- [Node.js](https://nodejs.org/) (v14.0.0 or later)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#installation)

## Installation

1. **Clone the repository**

    Open a terminal and run the following command to clone the project:

    ```bash
    git clone https://github.com/your-username/your-anchor-project.git
    cd your-anchor-project
    ```

2. **Install dependencies**

    While in the project directory, run:

    ```bash
    anchor install
    ```

    This command will install all the necessary dependencies needed for the project.

## Running the Project

1. **Build the project**

    Build the Anchor project by running:

    ```bash
    anchor build
    ```

    This will compile the smart contracts and create the necessary build files.

2. **Deploy the smart contracts**

    To deploy the smart contracts to devnet, use:

    ```bash
    anchor deploy
    ```

3. **Run tests**

    Run the tests to ensure everything is set up correctly:

    ```bash
    anchor run test
    ```