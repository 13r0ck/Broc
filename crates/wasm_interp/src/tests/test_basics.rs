#![cfg(test)]

use crate::frame::Frame;
use crate::tests::{
    const_value, create_exported_function_no_locals, create_exported_function_with_locals,
    default_state,
};
use crate::{DefaultImportDispatcher, ImportDispatcher, Instance};
use bumpalo::{collections::Vec, Bump};
use broc_wasm_module::sections::{Import, ImportDesc};
use broc_wasm_module::{
    opcodes::OpCode, sections::ElementSegment, Export, ExportType, SerialBuffer, Serialize,
    Signature, Value, ValueType, WasmModule,
};

#[test]
fn test_loop() {
    test_loop_help(10, 55);
}

fn test_loop_help(end: i32, expected: i32) {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);
    {
        let buf = &mut module.code.bytes;

        // Loop from 0 to end, adding the loop variable to a total
        let var_i = 0;
        let var_total = 1;

        let fn_len_index = buf.encode_padded_u32(0);

        // (local i32 i32)
        buf.push(1); // one group of the given type
        buf.push(2); // two locals in the group
        buf.push(ValueType::I32 as u8);

        // loop <void>
        buf.push(OpCode::LOOP as u8);
        buf.push(ValueType::VOID as u8);

        //   local.get $i
        buf.push(OpCode::GETLOCAL as u8);
        buf.encode_u32(var_i);

        //   i32.const 1
        buf.push(OpCode::I32CONST as u8);
        buf.encode_i32(1);

        //   i32.add
        buf.push(OpCode::I32ADD as u8);

        //   local.tee $i
        buf.push(OpCode::TEELOCAL as u8);
        buf.encode_u32(var_i);

        //   local.get $total
        buf.push(OpCode::GETLOCAL as u8);
        buf.encode_u32(var_total);

        //   i32.add
        buf.push(OpCode::I32ADD as u8);

        //   local.set $total
        buf.push(OpCode::SETLOCAL as u8);
        buf.encode_u32(var_total);

        //   local.get $i
        buf.push(OpCode::GETLOCAL as u8);
        buf.encode_u32(var_i);

        //   i32.const $end
        buf.push(OpCode::I32CONST as u8);
        buf.encode_i32(end);

        //   i32.lt_s
        buf.push(OpCode::I32LTS as u8);

        //   br_if 0
        buf.push(OpCode::BRIF as u8);
        buf.encode_u32(0);

        // end
        buf.push(OpCode::END as u8);

        // local.get $total
        buf.push(OpCode::GETLOCAL as u8);
        buf.encode_u32(var_total);

        // end function
        buf.push(OpCode::END as u8);

        buf.overwrite_padded_u32(fn_len_index, (buf.len() - fn_len_index) as u32);
    }
    module.code.function_offsets.push(0);
    module.code.function_count = 1;

    module.add_function_signature(Signature {
        param_types: Vec::new_in(&arena),
        ret_type: Some(ValueType::I32),
    });
    module.export.append(Export {
        name: "test",
        ty: ExportType::Func,
        index: 0,
    });

    let mut inst =
        Instance::for_module(&arena, &module, DefaultImportDispatcher::default(), false).unwrap();
    let return_val = inst.call_export("test", []).unwrap().unwrap();

    assert_eq!(return_val, Value::I32(expected));
}

#[test]
fn test_if_else() {
    test_if_else_help(0, 222);
    test_if_else_help(1, 111);
    test_if_else_help(-123, 111);
}

fn test_if_else_help(condition: i32, expected: i32) {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    let signature = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::I32),
    };
    let local_types = [(1, ValueType::I32)];
    create_exported_function_with_locals(&mut module, "test", signature, &local_types, |buf| {
        // i32.const <condition>
        buf.push(OpCode::I32CONST as u8);
        buf.encode_i32(condition);

        // if <blocktype>
        buf.push(OpCode::IF as u8);
        buf.push(ValueType::VOID as u8);

        // i32.const 111
        buf.push(OpCode::I32CONST as u8);
        buf.encode_i32(111);

        // local.set 0
        buf.push(OpCode::SETLOCAL as u8);
        buf.encode_u32(0);

        // else
        buf.push(OpCode::ELSE as u8);

        // i32.const 222
        buf.push(OpCode::I32CONST as u8);
        buf.encode_i32(222);

        // local.set 0
        buf.push(OpCode::SETLOCAL as u8);
        buf.encode_u32(0);

        // end
        buf.push(OpCode::END as u8);

        // local.get 0
        buf.push(OpCode::GETLOCAL as u8);
        buf.encode_u32(0);

        // end function
        buf.push(OpCode::END as u8);
    });

    let is_debug_mode = false;
    let mut inst = Instance::for_module(
        &arena,
        &module,
        DefaultImportDispatcher::default(),
        is_debug_mode,
    )
    .unwrap();
    let result = inst.call_export("test", []).unwrap().unwrap();

    assert_eq!(result, Value::I32(expected));
}

#[test]
fn test_br() {
    let start_fn_name = "test";
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    let signature = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::I32),
    };
    let local_types = [(1, ValueType::I32)];
    create_exported_function_with_locals(
        &mut module,
        start_fn_name,
        signature,
        &local_types,
        |buf| {
            // i32.const 111
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(111);

            // local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            // block  ;; label = @1
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //     block  ;; label = @2
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //     block  ;; label = @3
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //         br 2 (;@1;)
            buf.push(OpCode::BR as u8);
            buf.encode_u32(2);

            //         i32.const 444
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(444);

            //         local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            //     end
            buf.push(OpCode::END as u8);

            //     i32.const 333
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(333);

            //     local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            //     end
            buf.push(OpCode::END as u8);

            //     i32.const 222
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(222);

            //     local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            // end
            buf.push(OpCode::END as u8);

            // local.get 0)
            buf.push(OpCode::GETLOCAL as u8);
            buf.encode_u32(0);

            buf.push(OpCode::END as u8);
        },
    );

    let is_debug_mode = false;
    let mut inst = Instance::for_module(
        &arena,
        &module,
        DefaultImportDispatcher::default(),
        is_debug_mode,
    )
    .unwrap();
    let result = inst.call_export(start_fn_name, []).unwrap().unwrap();

    assert_eq!(result, Value::I32(111))
}

#[test]
fn test_br_if() {
    test_br_if_help(0, 222);
    test_br_if_help(1, 111);
}

fn test_br_if_help(condition: i32, expected: i32) {
    let start_fn_name = "test";
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    let signature = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::I32),
    };
    let local_types = [(1, ValueType::I32)];
    create_exported_function_with_locals(
        &mut module,
        start_fn_name,
        signature,
        &local_types,
        |buf| {
            // i32.const 111
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(111);

            // local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            // block  ;; label = @1
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //     block  ;; label = @2
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //     block  ;; label = @3
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //         i32.const <condition>
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(condition);

            //         br_if 2 (;@1;)
            buf.push(OpCode::BRIF as u8);
            buf.encode_u32(2);

            //         i32.const 444
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(444);

            //         local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            //     end
            buf.push(OpCode::END as u8);

            //     i32.const 333
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(333);

            //     local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            //     end
            buf.push(OpCode::END as u8);

            //     i32.const 222
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(222);

            //     local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            // end
            buf.push(OpCode::END as u8);

            // local.get 0)
            buf.push(OpCode::GETLOCAL as u8);
            buf.encode_u32(0);

            buf.push(OpCode::END as u8);
        },
    );

    let is_debug_mode = true;
    let mut inst = Instance::for_module(
        &arena,
        &module,
        DefaultImportDispatcher::default(),
        is_debug_mode,
    )
    .unwrap();
    let result = inst.call_export(start_fn_name, []).unwrap().unwrap();

    assert_eq!(result, Value::I32(expected))
}

#[test]
fn test_br_table() {
    test_br_table_help(0, 333);
    test_br_table_help(1, 222);
    test_br_table_help(2, 111);
}

fn test_br_table_help(condition: i32, expected: i32) {
    let start_fn_name = "test";
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    let signature = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::I32),
    };
    let local_types = [(1, ValueType::I32)];
    create_exported_function_with_locals(
        &mut module,
        start_fn_name,
        signature,
        &local_types,
        |buf| {
            // i32.const 111
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(111);

            // local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            // block  ;; label = @1
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //     block  ;; label = @2
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //     block  ;; label = @3
            buf.push(OpCode::BLOCK as u8);
            buf.push(ValueType::VOID);

            //         i32.const <condition>
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(condition);

            //         br_table 0 1 2 (;@1;)
            buf.push(OpCode::BRTABLE as u8);
            buf.encode_u32(2); // number of non-fallback branches
            buf.encode_u32(0);
            buf.encode_u32(1);
            buf.encode_u32(2);

            //     end
            buf.push(OpCode::END as u8);

            //         i32.const 333
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(333);

            //         local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            //         br 1
            buf.push(OpCode::BR as u8);
            buf.encode_u32(1);

            //     end
            buf.push(OpCode::END as u8);

            //     i32.const 222
            buf.push(OpCode::I32CONST as u8);
            buf.encode_i32(222);

            //     local.set 0
            buf.push(OpCode::SETLOCAL as u8);
            buf.encode_u32(0);

            //         br 0
            buf.push(OpCode::BR as u8);
            buf.encode_u32(0);

            //     end
            buf.push(OpCode::END as u8);

            // local.get 0)
            buf.push(OpCode::GETLOCAL as u8);
            buf.encode_u32(0);

            buf.push(OpCode::END as u8);
        },
    );

    let is_debug_mode = false;
    let mut inst = Instance::for_module(
        &arena,
        &module,
        DefaultImportDispatcher::default(),
        is_debug_mode,
    )
    .unwrap();
    let result = inst.call_export(start_fn_name, []).unwrap().unwrap();

    assert_eq!(result, Value::I32(expected))
}

struct TestDispatcher {
    internal_state: i32,
}

impl ImportDispatcher for TestDispatcher {
    fn dispatch(
        &mut self,
        module_name: &str,
        function_name: &str,
        arguments: &[Value],
        _memory: &mut [u8],
    ) -> Option<Value> {
        assert_eq!(module_name, "env");
        assert_eq!(function_name, "increment_state");
        assert_eq!(arguments.len(), 1);
        let val = arguments[0].expect_i32().unwrap();
        self.internal_state += val;
        Some(Value::I32(self.internal_state))
    }
}

#[test]
fn test_call_import() {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);
    let start_fn_name = "test";

    // User-provided non-Wasm code, with state
    let import_dispatcher = TestDispatcher {
        internal_state: 100,
    };

    // Function 0 is the import
    module.import.imports.push(Import {
        module: "env",
        name: "increment_state",
        description: ImportDesc::Func { signature_index: 0 },
    });
    module.types.insert(Signature {
        param_types: bumpalo::vec![in &arena; ValueType::I32],
        ret_type: Some(ValueType::I32),
    });

    // Function 1, which calls the import
    module.code.function_count = 1;
    let func0_offset = module.code.bytes.len() as u32;
    module.code.function_offsets.push(func0_offset);
    module.add_function_signature(Signature {
        param_types: Vec::new_in(&arena),
        ret_type: Some(ValueType::I32),
    });
    module.export.append(Export {
        name: start_fn_name,
        ty: ExportType::Func,
        index: 1,
    });
    [
        0, // no locals
        OpCode::I32CONST as u8,
        11, // argument to increment_state
        OpCode::CALL as u8,
        0, // function 0
        OpCode::I32CONST as u8,
        12, // argument to increment_state
        OpCode::CALL as u8,
        0, // function 0
        OpCode::I32ADD as u8,
        OpCode::END as u8,
    ]
    .serialize(&mut module.code.bytes);

    if false {
        let mut buf = Vec::new_in(&arena);
        module.serialize(&mut buf);
        let filename = "/tmp/broc/call-return.wasm";
        std::fs::write(filename, buf).unwrap();
        println!("Wrote to {}", filename);
    }

    let mut inst = Instance::for_module(&arena, &module, import_dispatcher, true).unwrap();

    let return_val = inst.call_export(start_fn_name, []).unwrap().unwrap();

    assert_eq!(return_val, Value::I32(234));
}

#[test]
fn test_call_return_no_args() {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);
    let start_fn_name = "test";

    module.code.function_count = 2;

    // Function 0
    let func0_offset = module.code.bytes.len() as u32;
    module.code.function_offsets.push(func0_offset);
    module.add_function_signature(Signature {
        param_types: Vec::new_in(&arena),
        ret_type: Some(ValueType::I32),
    });
    module.export.append(Export {
        name: start_fn_name,
        ty: ExportType::Func,
        index: 0,
    });
    [
        1, // 1 group of locals
        1, // 1 local
        ValueType::I32 as u8,
        OpCode::BLOCK as u8, /*  */
        // call from inside a block. callee's implicit return should still work correctly.
        ValueType::VOID as u8,
        OpCode::CALL as u8,
        1, // function 1
        OpCode::SETLOCAL as u8,
        0, // local 0
        OpCode::END as u8,
        OpCode::GETLOCAL as u8,
        0, // local 0
        OpCode::END as u8,
    ]
    .serialize(&mut module.code.bytes);

    // Function 1
    let func1_offset = module.code.bytes.len() as u32;
    module.code.function_offsets.push(func1_offset);
    module.add_function_signature(Signature {
        param_types: Vec::new_in(&arena),
        ret_type: Some(ValueType::I32),
    });
    [
        0, // no locals
        OpCode::I32CONST as u8,
        42, // constant value (<64 so that LEB-128 is just one byte)
        OpCode::END as u8,
    ]
    .serialize(&mut module.code.bytes);

    if false {
        let mut buf = Vec::new_in(&arena);
        module.serialize(&mut buf);
        let filename = "/tmp/broc/call-return.wasm";
        std::fs::write(filename, buf).unwrap();
        println!("Wrote to {}", filename);
    }

    let mut inst =
        Instance::for_module(&arena, &module, DefaultImportDispatcher::default(), true).unwrap();

    let return_val = inst.call_export(start_fn_name, []).unwrap().unwrap();

    assert_eq!(return_val, Value::I32(42));
}

#[test]
fn test_call_return_with_args() {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    // Function 0: calculate 2+2
    let signature0 = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::I32),
    };
    create_exported_function_no_locals(&mut module, "two_plus_two", signature0, |buf| {
        buf.push(OpCode::I32CONST as u8);
        buf.push(2);
        buf.push(OpCode::I32CONST as u8);
        buf.push(2);
        buf.push(OpCode::CALL as u8);
        buf.push(1);
        buf.push(OpCode::END as u8);
    });

    // Function 1: add two numbers
    let func1_offset = module.code.bytes.len() as u32;
    module.code.function_offsets.push(func1_offset);
    module.add_function_signature(Signature {
        param_types: bumpalo::vec![in &arena; ValueType::I32, ValueType::I32],
        ret_type: Some(ValueType::I32),
    });
    [
        0, // no locals
        OpCode::GETLOCAL as u8,
        0,
        OpCode::GETLOCAL as u8,
        1,
        OpCode::I32ADD as u8,
        OpCode::END as u8,
    ]
    .serialize(&mut module.code.bytes);

    let signature0 = Signature {
        param_types: bumpalo::vec![in &arena; ValueType::I32, ValueType::I32],
        ret_type: Some(ValueType::I32),
    };
    create_exported_function_no_locals(&mut module, "add", signature0, |buf| {
        buf.push(OpCode::GETLOCAL as u8);
        buf.push(0);
        buf.push(OpCode::GETLOCAL as u8);
        buf.push(1);
        buf.push(OpCode::I32ADD as u8);
        buf.push(OpCode::END as u8);
    });

    let mut inst =
        Instance::for_module(&arena, &module, DefaultImportDispatcher::default(), false).unwrap();
    let result = inst.call_export("two_plus_two", []).unwrap().unwrap();

    assert_eq!(result, Value::I32(4));
}

#[test]
fn test_call_indirect_ok() {
    let result = test_call_indirect_help(0, 0);
    assert_eq!(result, Value::I32(111));
}

#[test]
#[should_panic(expected = "Expected signature")]
fn test_call_indirect_wrong_signature() {
    test_call_indirect_help(0, 1);
}

#[test]
#[should_panic(expected = "element index")]
fn test_call_indirect_index_out_of_bounds() {
    test_call_indirect_help(0, 2);
}

#[test]
#[should_panic(expected = "Table index")]
fn test_call_indirect_unsupported_table() {
    test_call_indirect_help(1, 0);
}

fn test_call_indirect_help(table_index: u32, elem_index: u32) -> Value {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    let is_debug_mode = true;
    let start_fn_name = "test";

    // function 0: caller
    let signature0 = || Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::I32),
    };
    create_exported_function_no_locals(&mut module, start_fn_name, signature0(), |buf| {
        buf.append_u8(OpCode::I32CONST as u8);
        buf.encode_u32(elem_index);
        buf.append_u8(OpCode::CALLINDIRECT as u8);
        buf.encode_u32(0); // signature index
        buf.encode_u32(table_index);
        buf.append_u8(OpCode::END as u8);
    });

    // function 1: callee, right signature
    create_exported_function_no_locals(&mut module, "callee1", signature0(), |buf| {
        buf.append_u8(OpCode::I32CONST as u8);
        buf.encode_i32(111);
        buf.append_u8(OpCode::END as u8);
    });

    // function 2: callee, wrong signature
    let signature1 = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::F32),
    };
    create_exported_function_no_locals(&mut module, "callee2", signature1, |buf| {
        buf.append_u8(OpCode::F32CONST as u8);
        buf.encode_f32(2.22);
        buf.append_u8(OpCode::END as u8);
    });

    // Put functions 1 and 2 in the function table
    module.element.segments.push(ElementSegment::new(&arena));
    assert_eq!(module.element.get_or_insert_fn(1), 0);
    assert_eq!(module.element.get_or_insert_fn(2), 1);

    if false {
        let mut outfile_buf = Vec::new_in(&arena);
        module.serialize(&mut outfile_buf);
        let filename = format!("/tmp/broc/call_indirect_{}_{}.wasm", table_index, elem_index);
        std::fs::write(&filename, outfile_buf).unwrap();
        println!("\nWrote to {}\n", filename);
    }

    let mut inst = Instance::for_module(
        &arena,
        &module,
        DefaultImportDispatcher::default(),
        is_debug_mode,
    )
    .unwrap();
    inst.call_export(start_fn_name, []).unwrap().unwrap()
}

// #[test]
// fn test_drop() {}

#[test]
fn test_select() {
    test_select_help(Value::F32(1.11), Value::F32(2.22), -100, Value::F32(1.11));
    test_select_help(Value::F64(1.11), Value::F64(2.22), 0, Value::F64(2.22));
}

fn test_select_help(first: Value, second: Value, condition: i32, expected: Value) {
    let arena = Bump::new();
    let mut module = WasmModule::new(&arena);

    // Function 0: calculate 2+2
    let signature0 = Signature {
        param_types: bumpalo::vec![in &arena],
        ret_type: Some(ValueType::from(expected)),
    };
    create_exported_function_no_locals(&mut module, "test", signature0, |buf| {
        const_value(buf, first);
        const_value(buf, second);
        const_value(buf, Value::I32(condition));
        buf.push(OpCode::SELECT as u8);
        buf.push(OpCode::END as u8);
    });

    let mut inst =
        Instance::for_module(&arena, &module, DefaultImportDispatcher::default(), false).unwrap();
    let result = inst.call_export("test", []).unwrap().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_set_get_local() {
    let arena = Bump::new();
    let mut inst = default_state(&arena);
    let mut module = WasmModule::new(&arena);

    let mut buffer = vec![];
    let mut cursor = 0;
    [
        (1u32, ValueType::F32),
        (1u32, ValueType::F64),
        (1u32, ValueType::I32),
        (1u32, ValueType::I64),
    ]
    .serialize(&mut buffer);

    let fn_index = 0;
    let return_addr = 0x1234;
    let return_block_depth = 0;
    let n_args = 0;
    let ret_type = Some(ValueType::I32);
    inst.current_frame = Frame::enter(
        fn_index,
        return_addr,
        return_block_depth,
        n_args,
        ret_type,
        &buffer,
        &mut inst.value_store,
        &mut cursor,
    );

    module.code.bytes.push(OpCode::I32CONST as u8);
    module.code.bytes.encode_i32(12345);
    module.code.bytes.push(OpCode::SETLOCAL as u8);
    module.code.bytes.encode_u32(2);

    module.code.bytes.push(OpCode::GETLOCAL as u8);
    module.code.bytes.encode_u32(2);

    inst.execute_next_instruction(&module).unwrap();
    inst.execute_next_instruction(&module).unwrap();
    inst.execute_next_instruction(&module).unwrap();
    assert_eq!(inst.value_store.depth(), 5);
    assert_eq!(inst.value_store.pop(), Value::I32(12345));
}

#[test]
fn test_tee_get_local() {
    let arena = Bump::new();
    let mut inst = default_state(&arena);
    let mut module = WasmModule::new(&arena);

    let mut buffer = vec![];
    let mut cursor = 0;
    [
        (1u32, ValueType::F32),
        (1u32, ValueType::F64),
        (1u32, ValueType::I32),
        (1u32, ValueType::I64),
    ]
    .serialize(&mut buffer);

    let fn_index = 0;
    let return_addr = 0x1234;
    let return_block_depth = 0;
    let n_args = 0;
    let ret_type = Some(ValueType::I32);
    inst.current_frame = Frame::enter(
        fn_index,
        return_addr,
        return_block_depth,
        n_args,
        ret_type,
        &buffer,
        &mut inst.value_store,
        &mut cursor,
    );

    module.code.bytes.push(OpCode::I32CONST as u8);
    module.code.bytes.encode_i32(12345);
    module.code.bytes.push(OpCode::TEELOCAL as u8);
    module.code.bytes.encode_u32(2);

    module.code.bytes.push(OpCode::GETLOCAL as u8);
    module.code.bytes.encode_u32(2);

    inst.execute_next_instruction(&module).unwrap();
    inst.execute_next_instruction(&module).unwrap();
    inst.execute_next_instruction(&module).unwrap();
    assert_eq!(inst.value_store.depth(), 6);
    assert_eq!(inst.value_store.pop(), Value::I32(12345));
    assert_eq!(inst.value_store.pop(), Value::I32(12345));
}

#[test]
fn test_global() {
    let arena = Bump::new();
    let mut state = default_state(&arena);
    state
        .globals
        .extend_from_slice(&[Value::F64(1.11), Value::I32(222), Value::F64(3.33)]);
    let mut module = WasmModule::new(&arena);

    module.code.bytes.push(OpCode::GETGLOBAL as u8);
    module.code.bytes.encode_u32(1);
    module.code.bytes.push(OpCode::I32CONST as u8);
    module.code.bytes.encode_i32(555);
    module.code.bytes.push(OpCode::SETGLOBAL as u8);
    module.code.bytes.encode_u32(1);
    module.code.bytes.push(OpCode::GETGLOBAL as u8);
    module.code.bytes.encode_u32(1);

    state.execute_next_instruction(&module).unwrap();
    state.execute_next_instruction(&module).unwrap();
    state.execute_next_instruction(&module).unwrap();
    state.execute_next_instruction(&module).unwrap();
    assert_eq!(state.value_store.depth(), 2);
    assert_eq!(state.value_store.pop(), Value::I32(555));
    assert_eq!(state.value_store.pop(), Value::I32(222));
}

#[test]
fn test_i32const() {
    let arena = Bump::new();
    let mut state = default_state(&arena);
    let mut module = WasmModule::new(&arena);

    module.code.bytes.push(OpCode::I32CONST as u8);
    module.code.bytes.encode_i32(12345);

    state.execute_next_instruction(&module).unwrap();
    assert_eq!(state.value_store.pop(), Value::I32(12345))
}

#[test]
fn test_i64const() {
    let arena = Bump::new();
    let mut state = default_state(&arena);
    let mut module = WasmModule::new(&arena);

    module.code.bytes.push(OpCode::I64CONST as u8);
    module.code.bytes.encode_i64(1234567890);

    state.execute_next_instruction(&module).unwrap();
    assert_eq!(state.value_store.pop(), Value::I64(1234567890))
}

#[test]
fn test_f32const() {
    let arena = Bump::new();
    let mut state = default_state(&arena);
    let mut module = WasmModule::new(&arena);

    module.code.bytes.push(OpCode::F32CONST as u8);
    module.code.bytes.encode_f32(123.45);

    state.execute_next_instruction(&module).unwrap();
    assert_eq!(state.value_store.pop(), Value::F32(123.45))
}

#[test]
fn test_f64const() {
    let arena = Bump::new();
    let mut state = default_state(&arena);
    let mut module = WasmModule::new(&arena);

    module.code.bytes.push(OpCode::F64CONST as u8);
    module.code.bytes.encode_f64(12345.67890);

    state.execute_next_instruction(&module).unwrap();
    assert_eq!(state.value_store.pop(), Value::F64(12345.67890))
}
