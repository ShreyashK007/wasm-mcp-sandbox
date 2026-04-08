use anyhow::{Context, Result};
use std::path::Path;
use wasmtime::{Config, Engine, Linker, Module, Store};
// Notice the change here: WasiP1Ctx is at the root, preview1 is its own module
use wasmtime_wasi::{WasiCtxBuilder, WasiP1Ctx, preview1}; 

struct SandboxState {
    wasi: WasiP1Ctx,
}

pub struct WasmVault {
    engine: Engine,
}

impl WasmVault {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config).context("Failed to initialize Wasmtime engine")?;
        Ok(Self { engine })
    }

    pub fn execute_tool(&self, wasm_path: impl AsRef<Path>, input_args: &[&str]) -> Result<String> {
        let mut linker = Linker::new(&self.engine);
        
        // We use the preview1 linker
        preview1::add_to_linker_sync(&mut linker, |s: &mut SandboxState| &mut s.wasi)?;

        let module = Module::from_file(&self.engine, wasm_path)
            .context("Failed to load or compile the .wasm file. Does it exist?")?;

        let stdout_pipe = wasmtime_wasi::pipe::MemoryOutputPipe::new(4096);

        let mut wasi_builder = WasiCtxBuilder::new();
        wasi_builder.stdout(stdout_pipe.clone());
        wasi_builder.args(input_args);
        
        let wasi_ctx = wasi_builder.build_p1();

        let mut store = Store::new(&self.engine, SandboxState { wasi: wasi_ctx });
        store.set_fuel(500_000).unwrap();

        let instance = linker.instantiate(&mut store, &module)?;
        let start_func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        
        match start_func.call(&mut store, ()) {
            Ok(_) => {
                let output_bytes = stdout_pipe.contents();
                let result_string = String::from_utf8_lossy(&output_bytes).into_owned();
                Ok(result_string)
            }
            Err(trap) => {
                Err(anyhow::anyhow!("Sandbox Execution Trapped: {}", trap))
            }
        }
    }
}