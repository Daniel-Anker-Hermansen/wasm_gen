use wasm_gen::wasm_acc::*;

#[test]
fn i32() {
    assert_eq!(NumType::I32.compile(), vec![0x7F]);
}

#[test]
fn i32_to_i32() {
    let actual = FunctionType {
        input: ResultType { inner: vec![ValType::NumType(NumType::I32)] },
        output: ResultType { inner: vec![ValType::NumType(NumType::I32)] },
    }.compile();
    let expected = vec![0x60, 0x01, 0x7F, 0x01, 0x7F];
    assert_eq!(actual, expected);
}

#[test]
fn i32_i64_to_unit() {
    let actual = FunctionType {
        input: ResultType { inner: vec![ValType::NumType(NumType::I32), ValType::NumType(NumType::I64)] },
        output: ResultType { inner: vec![] },
    }.compile();
    let expected = vec![0x60, 0x02, 0x7F, 0x7E, 0x00];
    assert_eq!(actual, expected);
}

#[test]
fn type_section() {
    let f0 = FunctionType {
        input: ResultType { inner: vec![ValType::NumType(NumType::I32)] },
        output: ResultType { inner: vec![ValType::NumType(NumType::I32)] },
    };
    let f1 = FunctionType {
        input: ResultType { inner: vec![ValType::NumType(NumType::I32), ValType::NumType(NumType::I64)] },
        output: ResultType { inner: vec![] },
    };
    let section = TypeSection {
        contents: vec![f0, f1],
    };
    let actual = section.compile();
    let expected = vec![0x01, 0x0B, 0x02, 0x60, 0x01, 0x7F, 0x01, 0x7F, 0x60, 0x02, 0x7F, 0x7E, 0x00];
    assert_eq!(actual, expected) 
}

#[test]
fn u64() {
    let actual = u64::MAX.compile();
    let expected = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
    assert_eq!(actual, expected);
}

#[test]
fn func_sec() {
    let sec = FunctionSection {
        function_signatures: vec![12, 13, 14],
    };
    let actual = sec.compile();
    let expected = vec![0x03, 0x04, 0x03, 12, 13, 14];
    assert_eq!(actual, expected);
}

#[test]
fn export_sec() {
    let sec = ExportSection {
        exports: vec![Export { name: "hi".to_owned(), desc: ExportDescription::Func(12) }, Export { name: "ha".to_owned(), desc: ExportDescription::Func(68) }],
    };
    let actual = sec.compile();
    let expected = vec![0x07, 11, 0x02, 0x02, 104, 105, 0, 12, 0x02, 104, 97, 0, 68];
    assert_eq!(actual, expected);
}

#[test]
fn code_sec() {
    let code_sec = CodeSection {
        code: vec![Code { code: Func { locals: vec![Locals { count: 3, tpe: ValType::NumType(NumType::I64) }], expr: Expression { instr: vec![] } } }],
    };
    let actual = code_sec.compile();
    let expected = vec![10, 6, 1, 4, 1, 3, 0x7E, 0x0B];
    assert_eq!(actual, expected);
}
