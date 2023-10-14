use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

fn main() {
    let engine = Engine::default();

    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasm.wasm").unwrap();

    let mut linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap(); // enable STDIO
    let instance = linker.instantiate(&mut store, &module).unwrap();

    let hello_fn = instance.get_typed_func::<(),()>(&mut store, "hello").unwrap();
    hello_fn.call(&mut store, ()).unwrap();
}