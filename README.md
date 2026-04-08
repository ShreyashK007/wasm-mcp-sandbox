# 🛡️ Wasm-MCP-Sandbox

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)
![Wasmtime](https://img.shields.io/badge/Wasmtime-Native-blue.svg)
![MCP](https://img.shields.io/badge/Protocol-MCP-purple.svg)

A high-performance, zero-trust WebAssembly execution engine natively integrated with the Model Context Protocol (MCP). 

This project allows local AI agents (like Claude Desktop) to autonomously execute, evaluate, and iterate on generated code inside strictly isolated, capability-driven WASM sandboxes. 

## ⚡ Why This Exists

Currently, giving an AI agent the ability to execute code locally relies on slow, resource-heavy Docker containers or direct, unsafe host execution. 

**Wasm-MCP-Sandbox** solves this by acting as a microscopic, ultra-secure judge system. It spins up an isolated WebAssembly environment in milliseconds, executes the AI's compiled logic (C++, Rust, Python-via-Wasm), profiles the output, and instantly tears the environment down. 

### Core Features
* **Zero-Trust Execution:** Tools are isolated from the host OS. No default file system, network, or environment variable access.
* **Sub-Millisecond Cold Starts:** Powered by the `wasmtime` engine, bypassing the heavy virtualization overhead of Docker or MicroVMs.
* **Capability-Based Security (WASI):** Explicitly grant tools granular permissions (e.g., allowing an algorithmic tool read-only access to a specific `test_cases/` folder).
* **MCP Native:** Instantly plug-and-play with the Claude Desktop App. Drops any `.wasm` file into the `/tools` directory, and it is automatically exposed to the AI as a callable function.

---

## 🏗️ Architecture Flow

1. **The Brain:** The AI agent (via Claude Desktop) determines a tool needs to be run and formats an MCP JSON-RPC request.
2. **The Orchestrator:** The Rust host receives the MCP request over standard I/O.
3. **The Vault:** Rust instantly provisions a `wasmtime` runtime environment with strictly enforced memory constraints.
4. **The Execution:** The target `.wasm` file is loaded into the sandbox, arguments are injected, and the code executes at near-native speeds.
5. **The Feedback Loop:** `stdout` and `stderr` are captured, the sandbox is destroyed, and the results are routed back to the AI for self-correction.

---

## 🚀 Quick Start

### Prerequisites
* [Rust & Cargo](https://rustup.rs/) installed.
* [Claude Desktop App](https://claude.ai/download) installed.

### 1. Build the Server
Clone the repository and build the release binary:
```bash
git clone [https://github.com/YOUR_USERNAME/wasm-mcp-sandbox.git](https://github.com/YOUR_USERNAME/wasm-mcp-sandbox.git)
cd wasm-mcp-sandbox
cargo build --release
