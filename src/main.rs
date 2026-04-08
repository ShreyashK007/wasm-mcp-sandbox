use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

// Bring our new sandbox module into scope
mod sandbox;
use sandbox::WasmVault;
fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut line = String::new();

    eprintln!("🚀 Wasm-MCP-Sandbox is booting up...");

    // Initialize the Vault exactly once. This is the expensive part.
    let vault = match WasmVault::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("FATAL: Failed to initialize Wasm engine: {}", e);
            return;
        }
    };
    
    eprintln!("🛡️ Wasmtime Engine initialized with strict fuel constraints.");
    eprintln!("👂 Listening for MCP messages on stdio...");

    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap_or(0) == 0 { break; }

        if let Ok(req) = serde_json::from_str::<Value>(&line) {
            let id = req["id"].clone();
            let method = req["method"].as_str().unwrap_or("");

            let response = match method {
                "initialize" => {
                    json!({
                        "jsonrpc": "2.0", "id": id,
                        "result": { "protocolVersion": "2024-11-05", "capabilities": { "tools": {} }, "serverInfo": { "name": "wasm-sandbox", "version": "0.1.0" } }
                    })
                },
                
                "tools/list" => {
                    json!({
                        "jsonrpc": "2.0", "id": id,
                        "result": {
                            "tools": [
                                {
                                    // Let's expose a real WASM tool!
                                    "name": "run_wasm_math",
                                    "description": "Executes a compiled WebAssembly algorithm inside a zero-trust sandbox.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "number": { "type": "string", "description": "The number to process" }
                                        },
                                        "required": ["number"]
                                    }
                                }
                            ]
                        }
                    })
                },
                
                "tools/call" => {
                    let tool_name = req["params"]["name"].as_str().unwrap_or("");
                    
                    if tool_name == "run_wasm_math" {
                        let target_num = req["params"]["arguments"]["number"].as_str().unwrap_or("0");
                        
                        // THE MAGIC HAPPENS HERE
                        // We ask the Vault to execute a specific .wasm file, passing in Claude's arguments.
                        match vault.execute_tool("C:\\Users\\shrey\\wasm-mcp-sandbox\\tools\\math_algo.wasm", &["math_algo", target_num]) {
                            Ok(output) => {
                                json!({
                                    "jsonrpc": "2.0", "id": id,
                                    "result": { "content": [{ "type": "text", "text": output }] }
                                })
                            },
                            Err(e) => {
                                // If the WASM crashes or times out, we send the error BACK to Claude so it can fix it!
                                json!({
                                    "jsonrpc": "2.0", "id": id,
                                    "result": { "content": [{ "type": "text", "text": format!("SANDBOX ERROR: {}", e) }], "isError": true }
                                })
                            }
                        }
                    } else {
                        json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32601, "message": "Tool not found"}})
                    }
                },
                _ => continue,
            };

            let res_str = serde_json::to_string(&response).unwrap();
            println!("{}", res_str);
            stdout.flush().unwrap();
        }
    }
}