use std::{fs::File, io::Write};

use wasm_gen::wasm_acc::*;
use wasmer::{Store, imports, Instance, Value};

fn main() {
    let mut args = std::env::args();
    let x = args.nth(1).unwrap().parse().unwrap();
    let instructions = args
        .map(|t| {
            match t.as_str() {
                "+" => Instruction::I64Add,
                "-" => Instruction::I64Sub,
                "*" => Instruction::I64Mul,
                "/" => Instruction::I64Div,
                "x" => Instruction::LocalGet(0),
                v => Instruction::I64Const(v.parse().unwrap())
            }
        })
        .collect();

    let ftype = FunctionType { input: ResultType { inner: vec![ValType::NumType(NumType::I64)] }, output: ResultType { inner: vec![ValType::NumType(NumType::I64)] } };
    let typesec = TypeSection {
        contents: vec![ftype],
    };
    let funcsec = FunctionSection {
        function_signatures: vec![0],
    };
    let export = Export { name: "hi".to_string(), desc: ExportDescription::Func(0) };
    let exportsec = ExportSection {
        exports: vec![export],
    };
    let func = Func { locals: vec![], expr: Expression { instr: instructions } };
    let code = Code { code: func };
    let codesec = CodeSection { code: vec![code] };
    let module = Module {
        typesec,
        funcsec,
        exportsec,
        codesec,
    };
    let wasm = module.compile();
    
    let mut file = File::create("identity.wasm").unwrap();
    file.write_all(&wasm).unwrap();

    let mut store = Store::default();
    let module = wasmer::Module::new(&store, &wasm).unwrap();
    let import_objects = imports! {};
    let instance = Instance::new(&mut store, &module, &import_objects).unwrap();

    let hi = instance.exports.get_function("hi").unwrap();
    let result = hi.call(&mut store, &[Value::I64(x)]).unwrap();
    println!("{:?}", result);
}
