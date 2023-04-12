pub struct WasmAcc {
    bytes: Vec<u8>,
}

impl WasmAcc {
    pub fn new() -> WasmAcc {
        WasmAcc { bytes: Vec::new() }
    }
 
    pub fn write<V>(&mut self, v: V) where V: WriteToWasm {
        v.write_to_wasm(self);
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.bytes
    }
}

pub trait WriteToWasm {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc);

    fn compile(&self) -> Vec<u8> {
        let mut wasm_acc = WasmAcc::new();
        self.write_to_wasm(&mut wasm_acc);
        wasm_acc.to_vec()
    }
}

impl<V> WriteToWasm for &V where V: WriteToWasm + ?Sized {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        V::write_to_wasm(self, wasm_acc);
    }
}

pub struct Module {
    pub typesec: TypeSection,
    pub funcsec: FunctionSection,
    pub exportsec: ExportSection,
    pub codesec: CodeSection,
}

impl WriteToWasm for Module {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.bytes.extend([0x00, 0x61, 0x73, 0x6D]);
        wasm_acc.bytes.extend([0x01, 0x00, 0x00, 0x00]);
        wasm_acc.write(&self.typesec);
        wasm_acc.write(&self.funcsec);
        wasm_acc.write(&self.exportsec);
        wasm_acc.write(&self.codesec);
    }
}

pub struct CodeSection {
    pub code: Vec<Code>,
}

impl WriteToWasm for CodeSection {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(10u8);
        let code_bytes = self.code.compile();
        wasm_acc.write(code_bytes.len() as u32);
        wasm_acc.bytes.extend(code_bytes);
    }
}

pub struct Code {
    pub code: Func,
}

impl WriteToWasm for Code {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        let func_bytes = self.code.compile();
        wasm_acc.write(func_bytes.len() as u32);
        wasm_acc.bytes.extend(func_bytes);
    }
}

pub struct Func {
    pub locals: Vec<Locals>,
    pub expr: Expression,
}

impl WriteToWasm for Func {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(&self.locals);
        wasm_acc.write(&self.expr);
    }
}

pub struct Locals {
    pub count: u32,
    pub tpe: ValType,
}

impl WriteToWasm for Locals {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(self.count);
        wasm_acc.write(&self.tpe);
    }
}

pub struct Expression {
    pub instr: Vec<Instruction>,
}

impl WriteToWasm for Expression {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        for instr in &self.instr {
            wasm_acc.write(instr);
        }
        wasm_acc.write(0x0Bu8);
    }
}

pub enum Instruction {
    LocalGet(u32),
    I64Add,
    I64Sub,
    I64Mul,
    I64Div,
    I64Const(i64),
}

impl WriteToWasm for Instruction {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        match self {
            Instruction::LocalGet(idx) => {
                wasm_acc.write(0x20u8);
                wasm_acc.write(idx);
            }
            Instruction::I64Add => wasm_acc.write(0x7Cu8),
            Instruction::I64Const(v) => {
                wasm_acc.write(0x42u8);
                wasm_acc.write(v);
            },
            Instruction::I64Sub => wasm_acc.write(0x7Du8),
            Instruction::I64Mul => wasm_acc.write(0x7Eu8),
            Instruction::I64Div => wasm_acc.write(0x7Fu8),
        }
    }
}

pub struct ExportSection {
    pub exports: Vec<Export>,
}

impl WriteToWasm for ExportSection {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(7u8);
        let exports_bytes = self.exports.compile();
        wasm_acc.write(exports_bytes.len() as u32);
        wasm_acc.bytes.extend(exports_bytes);
    }
}

pub struct Export {
    pub name: String,
    pub desc: ExportDescription,
}

impl WriteToWasm for Export {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(&self.name);
        wasm_acc.write(&self.desc);
    }
}

pub enum ExportDescription {
    Func(u32),
}

impl WriteToWasm for ExportDescription {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        match self {
            ExportDescription::Func(idx) => {
                wasm_acc.write(0u8);
                wasm_acc.write(idx);
            },
        }
    }
}

pub struct FunctionSection {
    pub function_signatures: Vec<u32>,
}

impl WriteToWasm for FunctionSection {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(0x03u8);
        let signatures_bytes = self.function_signatures.compile();
        wasm_acc.write(signatures_bytes.len() as u32);
        wasm_acc.bytes.extend(signatures_bytes);
    }
}

pub struct TypeSection {
    pub contents: Vec<FunctionType>,
}

impl WriteToWasm for TypeSection {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(1u8);
        let section_bytes = self.contents.compile();
        wasm_acc.write(section_bytes.len() as u32);
        wasm_acc.bytes.extend(section_bytes);
    } 
}

pub struct FunctionType {
    pub input: ResultType,
    pub output: ResultType,
}

impl WriteToWasm for FunctionType {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(0x60u8);
        wasm_acc.write(&self.input);
        wasm_acc.write(&self.output);
    }
}

pub struct ResultType {
    pub inner: Vec<ValType>,
}

impl WriteToWasm for ResultType {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(&self.inner);
    }
}

pub enum ValType {
    NumType(NumType)
}

impl WriteToWasm for ValType {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        match self {
            ValType::NumType(v) => wasm_acc.write(v),
        }
    }
}

pub enum NumType {
    I32,
    I64,
}

impl WriteToWasm for NumType {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        match self {
            NumType::I32 => wasm_acc.write(0x7Fu8),
            NumType::I64 => wasm_acc.write(0x7Eu8),
        }
    }
}

impl WriteToWasm for u8 {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.bytes.push(*self);
    }
}

impl WriteToWasm for u32 {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(*self as u64);
    }
}

impl WriteToWasm for u64 {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        let mut rest = *self;
        while rest > 127 {
            let byte = 128 | (127 & rest as u8);
            wasm_acc.write(byte);
            rest >>= 7;
        }
        wasm_acc.write(rest as u8);
    }
}

impl WriteToWasm for i32 {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(*self as i64);
    }
}

impl WriteToWasm for i64 {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        let mut rest = *self;
        while rest > 63 || rest < -64 {
            let byte = 128 | (127 & rest as u8);
            wasm_acc.write(byte);
            rest >>= 7;
        }
        wasm_acc.write(rest as u8 & 127);
    }
}

impl WriteToWasm for String {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(self.as_bytes())
    }
}

impl<V> WriteToWasm for [V] where V: WriteToWasm {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        wasm_acc.write(self.len() as u32);
        for v in self {
            wasm_acc.write(v);
        }
    }
}

impl<V> WriteToWasm for Vec<V> where V: WriteToWasm {
    fn write_to_wasm(&self, wasm_acc: &mut WasmAcc) {
        <[V]>::write_to_wasm(self, wasm_acc);
    }
}
