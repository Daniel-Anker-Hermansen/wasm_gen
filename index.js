async function run() {    
    const input = await fetch("identity.wasm");
    const imports = {};
    imports.wbg = {};
    const { instance, module } = await WebAssembly.instantiateStreaming(await input, imports);
    let wasm = instance.exports;
    console.log(wasm.hi(BigInt(12)));
}

run().then(() => console.log("finished"));
