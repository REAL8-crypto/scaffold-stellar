# Scaffold Stellar (REAL8 Fork)

[![Apache 2.0 licensed](https://img.shields.io/badge/license-apache%202.0-blue.svg)](LICENSE)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/AhaLabs/scaffold-stellar)

This is a fork of **Scaffold Stellar**, a developer toolkit for building decentralized applications (dApps) and smart contracts on the [**Stellar** blockchain](https://stellar.org).

It helps you go from **idea** to **working full-stack dApp** faster — by providing CLI tools, reusable contract templates, a smart contract registry, and a modern frontend.

---

## REAL8 Customizations

This fork of Scaffold Stellar includes several customizations specific to REAL8:

1. **Custom Smart Contracts**: We've added REAL8-specific smart contracts in the `contracts/real8/` directory.
   - `TokenContract.rs`: Implements REAL8's unique token logic
   - `GovernanceContract.rs`: Handles REAL8's decentralized governance features

2. **Enhanced Frontend**: The `src/` directory includes REAL8-branded components and pages.
   - Custom styling in `src/styles/real8-theme.css`
   - REAL8-specific React components in `src/components/real8/`

3. **Additional CLI Commands**: We've extended the Scaffold Stellar CLI with REAL8-specific commands:
   ```bash
   stellar scaffold real8-init  # Initialize a REAL8-specific project structure
   stellar registry real8-deploy  # Deploy REAL8 contracts with predefined parameters

4. **Modified Deployment Pipeline**: Our fork includes changes to the deployment scripts to accommodate REAL8's infrastructure:
Updated deploy_registry.sh with REAL8-specific configurations
New real8_post_deploy.sh script for additional setup steps

5. **Documentation**: We've added REAL8-specific documentation in the docs/real8/ directory.
For more details on these customizations, please refer to our REAL8 Developer Guide.

---

## Why Use Scaffold Stellar?

- Simplifies blockchain dApp development
- Generates smart contract projects and React UIs
- Deploys smart contracts and manages versions
- Easy to learn for newcomers; powerful for pros

---

## What Is Stellar?

[**Stellar**](https://www.stellar.org/) is a blockchain designed for fast, low-cost financial transactions and smart contracts written in **Rust** and compiled to **WebAssembly (Wasm)**.

With Scaffold Stellar, you write smart contracts in Rust and interact with them using modern TypeScript + React tooling.

---

## Prerequisites

Before you begin, make sure you have the following installed:

| Tool | Description | Install Link |
|------|-------------|--------------|
| [Rust & Cargo](https://www.rust-lang.org/tools/install) | For writing and compiling smart contracts | `curl https://sh.rustup.rs -sSf \| sh` |
| [Node.js & npm](https://nodejs.org/) | For frontend development | Download from official site |
| [Stellar CLI](https://github.com/stellar/stellar-cli) | for building, deploying, and interacting with smart contracts | [`Link for the repo`](https://github.com/stellar/stellar-cli)

---

## **Quickstart** (New Developers Welcome!)

This section walks you through setting up Scaffold Stellar from scratch.

### 1. Install the Scaffold Stellar CLI

```
This a fork of Scaffold Stellar, a convention-over-configuration toolkit for blockchain and distributed application development on the Stellar network. It provides a seamless development experience through CLI tools, smart contract management, and deployment utilities. 

More info at: https://github.com/AhaLabs/scaffold-stellar

The project consists of several main components:

- **stellar-scaffold-cli**: The main command-line interface for Scaffold Stellar. It provides commands for initializing projects, managing development workflows, and automating contract deployment.

- **stellar-registry-cli**: A CLI tool for managing the on-chain contract registry, handling contract publishing, deployment, and version management.

- **stellar-build**: Core build utilities and helper functions for Stellar smart contract development.

- **stellar-scaffold-macro**: Rust procedural macros that simplify contract development and integrate with the Scaffold Stellar ecosystem.

- **registry**: The on-chain smart contract that powers the Scaffold Stellar registry system, enabling contract verification, naming, and version management.


## Features

- **CLI Plugins for Stellar**
  - `stellar scaffold`: Initialize and manage Scaffold Stellar projects
    - Creates smart contract projects with best practices
    - Includes frontend setup using [scaffold-stellar-frontend](https://github.com/AhaLabs/scaffold-stellar-frontend)
  - `stellar registry`: Publish Wasm binaries and deploy smart contracts
  - Automated development workflow with hot reloading

  Currently these are available as separate binaries: `stellar-scaffold` and `stellar-registry` respectively.

- **Declarative Environment Management**
  - Environment-specific builds (development, testing, staging, production)
  - Seamless integration with both local and deployed contracts
  - Network configuration via `environments.toml`

- **Coming soon: Smart Contract Registry**
  - On-chain publishing platform for Wasm binaries
  - Version management and contract naming
  - Contract verification and dependency management

- **Coming soon: Deployment Pipeline**
  - Streamlined deployment process for testnet and mainnet
  - Contract lifecycle management
  - Automated environment updates

## Project Structure

- `stellar-scaffold-cli`: Main CLI tool for project scaffolding and development
- `stellar-registry-cli`: Contract registry and deployment management
- `stellar-build`: Build utilities for Stellar smart contracts
- `stellar-scaffold-macro`: Procedural macros for contract development

## Installation

### Development Setup
```bash
just setup && just build
```

### Direct Installation
To install the executables directly:

```bash
# Install stellar-scaffold-cli
cargo install stellar-scaffold-cli
```

The Scaffold Stellar CLI is installed as a plugin under the `stellar` CLI.

### 2. Create a New Project
```
stellar scaffold init my-project
cd my-project
```

### 3. Configure Your Environment
```
# Copy and configure environment variables
cp .env.example .env
```

Edit `.env` with your preferred network, secret keys, and other settings.

### 4. Install Frontend Dependencies
```
# Install Frontend dependencies
npm install
```
### 5. Start Development
```
npm run dev
```
You should see your React frontend at http://localhost:5173.

### 6. For testnet/mainnet deployment:
```
# First publish your contract to the registry
stellar registry publish

# Then deploy an instance with constructor parameters
stellar registry deploy \
  --deployed-name my-contract \
  --published-name my-contract \
  -- \
  --param1 value1
  
# Can access the help docs with --help
stellar registry deploy \
  --deployed-name my-contract \
  --published-name my-contract \
  -- \
  --help

# Install the deployed contract locally
stellar registry install my-contract
```

## Project Layout
After scaffolding a project, your folder structure will look like this:

```
my-project/
├── contracts/            # Rust smart contracts (compiled to WASM)
├── packages/             # Auto-generated TypeScript contract clients
├── src/                  # React frontend code
│   ├── components/       # Reusable UI pieces
│   ├── contracts/        # Contract interaction logic
│   ├── App.tsx           # Main app component
│   └── main.tsx          # Entry point
├── environments.toml     # Configuration per environment (dev/test/prod)
├── .env                  # Local environment variables
├── package.json          # Frontend packages
├── target/               # Build outputs
```

This template provides a ready-to-use frontend application with example smart contracts and their TypeScript clients. You can use these as reference while building your own contracts and UI. The frontend is set up with `Vite`, `React`, and includes basic components for interacting with the contracts.

See the [CLI Documentation](https://github.com/AhaLabs/scaffold-stellar/blob/main/docs/cli.md) for detailed command information and the [Environments Guide](https://github.com/AhaLabs/scaffold-stellar/blob/main/docs/environments.md) for configuration details.

---

## CLI Tools
Scaffold Stellar provides two main CLI tools:

stellar-scaffold
Initialize and manage dApp projects:
```
stellar scaffold init my-project
stellar scaffold build
```
Manage contract deployment and versions:
```
stellar registry publish    # Publish contract to the registry
stellar registry deploy     # Deploy a contract instance
stellar registry install    # Install deployed contracts locally
```
> Use `--help` on any command for usage instructions.

---
## Smart Contract Deployment
1. Publish Your Contract
```
stellar registry publish
```
2. Deploy the Contract
```
stellar registry deploy \
  --deployed-name my-contract \
  --published-name my-contract \
  -- \
  --param1 value1
```
3. Install the Deployed Contract
```
stellar registry install my-contract
```
> You can deploy to testnet or mainnet depending on your `.env` and `environments.toml`.

---
## Concept: What Is the Contract Registry?
The registry is an on-chain smart contract that lets you:
* Publish and verify other contracts
* Manage multiple versions
* Reuse deployed contracts across dApps

>This means your contracts can be upgraded, shared, and used like packages.
---
## Project Structure (Top-Level)
Your repo contains the following key folders:

|Folder	| Purpose |
|-------|---------|
|`.cargo/`, `.config/`	| Rust and build settings| 
|`contracts/` |	Example smart contracts|
|`crates/`|	Internal Rust libraries and helpers|
|`docs/`|	Documentation files|
|`npm/`|	Shared frontend packages|
|`deploy_registry.sh`|	Helper script to deploy the registry|
|`justfile` |	Commands you can run with just|

---

Documentation 
* [CLI Commands](https://github.com/AhaLabs/scaffold-stellar/blob/main/docs/cli.md)
* [Environment Setup](https://github.com/AhaLabs/scaffold-stellar/blob/main/docs/environments.md)
* [Registry Guide](https://github.com/AhaLabs/scaffold-stellar/blob/main/docs/registry.md)

---
## Additional Developer Resources
- Video: [Intro to Scaffold Stellar](https://www.youtube.com/watch?v=559ht4K4pkM)
- Video: [Which Frontend?](https://www.youtube.com/watch?v=pz7O54Oia_w)
- Video: [Get Started Building](https://www.youtube.com/watch?v=H-M962aPuTk)
- Video: [Live Demo of Scaffold Stellar](https://www.youtube.com/watch?v=0syGaIn3ULk) 👈 Start Here

---
## Contributing
We love contributions! If you’re new, check these out:

[Contributing Guide](https://github.com/AhaLabs/scaffold-stellar/blob/main/CONTRIBUTING.md)

## License

#### This project is licensed under the Apache-2.0 License — see the [LICENSE](https://github.com/scaffold-stellar/blob/main/LICENSE) file for details.
---

## Need Help?
If you’re new to Stellar, Rust, or smart contracts:

Ask questions in the repo Discussions tab

Search [DeepWiki](https://deepwiki.org/)

Or just open an issue — we're happy to help!

Happy hacking! 
---
