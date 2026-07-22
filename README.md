<div align="center">

# Rust Projects — A Systems & Blockchain Engineering Journey

**Low-level systems programming and blockchain protocol work, built from scratch in Rust.**

[![Rust](https://img.shields.io/badge/Rust-2021%2F2024-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-Anchor%20%26%20Native-14F195?style=for-the-badge&logo=solana&logoColor=black)](https://solana.com/)
[![Blockchain](https://img.shields.io/badge/Blockchain-secp256k1%20%7C%20PoW-F7931A?style=for-the-badge&logo=bitcoin&logoColor=white)](https://bitcoin.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](./LICENSE)

</div>

---

## About This Repository

I'm a systems-oriented engineer with a deep interest in **blockchain protocols and low-level infrastructure**. This monorepo is a curated collection of the projects I build to master Rust — from the fundamentals of memory-safe systems programming up to writing a Bitcoin node and deploying Solana smart contracts.

Every project here is written **from scratch** — no boilerplate generators, no shortcuts on the hard parts. The goal is to genuinely understand how the machines and networks we rely on actually work: shells, HTTP servers, interpreters, cryptography, consensus, and on-chain programs.

The repository is deliberately organized by **increasing difficulty**, so it doubles as a map of the journey from language fundamentals to protocol engineering.

---

## Repository Structure

```
Rust-Projects/
├── Beginner/
│   └── Solana-Native-Contracts/     # First steps into on-chain programming
├── Intermediate/
│   ├── Cli-tool/                    # A POSIX-style shell
│   └── HTTP-Server/                 # A multi-threaded HTTP/1.1 server
└── Advance/
    ├── Lox-Interpretor/             # A tree-walking language interpreter (scanner)
    ├── rsbtc/                       # A Bitcoin implementation in Rust (workspace)
    └── Solana-Smart-Contracts/
        ├── counter_program/         # Native Solana program (Borsh, CPI)
        └── voting/                  # Anchor voting dApp with PDAs
```

---

## Projects

### Advanced

<table>
<tr>
<td width="200"><strong>rsbtc</strong><br/><sub>Bitcoin in Rust</sub></td>
<td>

A from-scratch **Bitcoin implementation** structured as a Cargo **workspace** with four crates: a shared `btc_lib`, plus `node`, `miner`, and `wallet` binaries. Models the real protocol:

- **Blocks, headers & transactions** with a UTXO-style input/output model
- **`secp256k1` ECDSA** key generation, signing, and verification (`k256`, `ecdsa`)
- **SHA-256** hashing and **Merkle roots** over transaction sets
- **256-bit arithmetic** (`U256`) for Proof-of-Work difficulty targets
- **CBOR** serialization (`ciborium`) and strongly-typed error handling (`thiserror`)

**Stack:** `k256` · `ecdsa` · `sha256` · `uint` · `serde` · `ciborium` · `chrono` · `uuid`

</td>
</tr>
<tr>
<td><strong>Solana Smart Contracts</strong><br/><sub>Native + Anchor</sub></td>
<td>

Two on-chain Solana programs demonstrating both ends of the tooling spectrum:

- **`counter_program`** — a **native** Solana program (no framework) using **Borsh** serialization, manual account/rent handling, and **Cross-Program Invocation (CPI)** to `create_account`. Supports `Initialize` and `Double` instructions.
- **`voting`** — an **Anchor** program implementing a full voting dApp: create polls, register candidates, and cast time-gated votes using **Program Derived Addresses (PDAs)**, `InitSpace` account sizing, and custom on-chain error codes.

**Stack:** `solana-program` · `anchor-lang` · `borsh` · PDAs · CPI

</td>
</tr>
<tr>
<td><strong>Lox-Interpretor</strong><br/><sub>Language tooling</sub></td>
<td>

A **tree-walking interpreter** for the Lox language (from *Crafting Interpreters*). The current milestone implements the **scanner/lexer**: turning raw source into a token stream with literals, line tracking, and precise error reporting (unexpected characters, unterminated strings). Uses proper Unix exit codes (`64`/`65`) like a real compiler front-end.

**Stack:** Pure Rust · zero dependencies

</td>
</tr>
</table>

### Intermediate

<table>
<tr>
<td width="200"><strong>Cli-tool</strong><br/><sub>A POSIX-style shell</sub></td>
<td>

A working command-line **shell** written from scratch. Implements a REPL with built-ins (`echo`, `type`, `exit`), **`PATH` resolution**, executable-bit detection via Unix file permissions, and spawning/waiting on external child processes.

**Stack:** `std::process` · `std::os::unix` · `std::env`

</td>
</tr>
<tr>
<td><strong>HTTP-Server</strong><br/><sub>Networking from the socket up</sub></td>
<td>

A **multi-threaded HTTP/1.1 server** built directly on TCP sockets — no web framework. Parses raw request lines and headers, routes requests, echoes paths, reflects the `User-Agent` header, and returns correct `200`/`404` responses. Spawns a thread per connection for concurrency.

**Stack:** `std::net::TcpListener` · `std::thread` · `BufReader`

</td>
</tr>
</table>

### Beginner

<table>
<tr>
<td width="200"><strong>Solana-Native-Contracts</strong><br/><sub>On-chain foundations</sub></td>
<td>

The starting point for Solana development — a native program scaffold with unit tests, laying the groundwork that the advanced contracts build on.

</td>
</tr>
</table>

---

## Skills Demonstrated

| Domain | What's shown |
| --- | --- |
| **Systems Programming** | Shells, TCP servers, process management, Unix permissions, exit codes |
| **Blockchain / Protocols** | Bitcoin data structures, Proof-of-Work targets, UTXO model, Merkle trees |
| **Cryptography** | `secp256k1` ECDSA signing & verification, SHA-256, key serialization |
| **Solana Development** | Native programs, Anchor framework, PDAs, CPI, Borsh, rent & account model |
| **Language Tooling** | Lexing/scanning, token streams, structured error reporting |
| **Rust Craft** | Cargo workspaces, trait-based design, `serde`, typed errors with `thiserror` |

---

## Getting Started

All projects are standard Cargo packages. Requires a [Rust toolchain](https://rustup.rs/).

```bash
# Clone the repo
git clone https://github.com/Vineet1101/Rust-Projects.git
cd Rust-Projects

# Run any Rust project (example: the HTTP server)
cd Intermediate/HTTP-Server
cargo run

# Build the Bitcoin workspace
cd Advance/rsbtc
cargo build --workspace
```

For the **Solana / Anchor** programs:

```bash
cd Advance/Solana-Smart-Contracts/voting
anchor build && anchor test
```

> **Note:** This is a living repository — several projects are actively evolving as I work through deeper milestones (e.g., wiring up the miner/node/wallet binaries in `rsbtc` and extending the Lox interpreter into a full parser + evaluator).

---

## Roadmap

- [ ] Complete the `rsbtc` mining loop, P2P node networking, and wallet CLI
- [ ] Extend the Lox interpreter with a parser, AST, and tree-walking evaluator
- [ ] Add an on-chain test suite and client for the Solana voting program
- [ ] Grow the collection toward async networking and distributed systems

---

## Contact

**Vineet Goel**
[![GitHub](https://img.shields.io/badge/GitHub-Vineet1101-181717?style=flat&logo=github)](https://github.com/Vineet1101)

---

## License

Released under the [MIT License](./LICENSE).

<div align="center"><sub>Built with Rust and a lot of curiosity about how systems really work.</sub></div>
