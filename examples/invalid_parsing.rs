use wasmlite_parser::Parser;
use wasmlite_utils::{debug, verbose};

fn main() {
    // Parse wasm bytes
    // let wasm_module = Parser::new(&invalid_i32_add_wrong_arg_type()).module();
    // let wasm_module = Parser::new(&invalid_i32_add_wrong_arg_arity()).module();
    // let wasm_module = Parser::new(&invalid_wrong_body_size()).module();
    // let wasm_module = Parser::new(&valid_i32_add_more_args_on_stack()).module();
    let wasm_module = Parser::new(&valid_i64_add_nested_operation()).module();

    match wasm_module {
        Ok(module) => verbose!("wasm_module = {:#?}", module),
        Err(error) => debug!("ERROR!\n\n{:#?}", error),
    }
}

// INVALIDS

pub fn invalid_i32_add_wrong_arg_type() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60,
        0x02,
        0x7d,
        0x7c,
        0x01,
        0x7f,
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00,
        //=======================
        0x0a, // SECTION: Code
        0x0b, // payload len: 11
        0x01, // entry count: 1
        //          ---         //
        0x09, // body size: 9
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x42, // i64.const
        0xe9, //
        0x00, //     105
        0x6a, // i32.add
        0x0b, // end function
    ]
}

pub fn invalid_i32_add_wrong_arg_arity() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60,
        0x02,
        0x7d,
        0x7c,
        0x01,
        0x7f,
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00,
        //=======================
        0x0a, // SECTION: Code
        0x0b, // payload len: 11
        0x01, // entry count: 1
        //          ---         //
        0x09, // body size: 9
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x6a, // i32.add
        0x0b, // end function
    ]
}

pub fn invalid_wrong_body_size() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60,
        0x02,
        0x7d,
        0x7c,
        0x01,
        0x7f,
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00,
        //=======================
        0x0a, // SECTION: Code
        0x0b, // payload len: 11
        0x01, // entry count: 1
        //          ---         //
        0x09, // body size: 9
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x41, // i32.const
        0xe9, //
        0x00, //     105
        0x6a, // i32.add
        0x0b, // end function
    ]
}

// VALIDS

pub fn valid_i32_add_more_args_on_stack() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60,
        0x02,
        0x7d,
        0x7c,
        0x01,
        0x7f,
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00,
        //=======================
        0x0a, // SECTION: Code
        0x0e, // payload len: 14
        0x01, // entry count: 1
        //          ---         //
        0x0c, // body size: 12
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x41, // i32.const
        0xe9, //
        0x00, //     105
        0x6a, // i32.add
        0x0b, // end function
    ]
}

pub fn valid_i64_add_nested_operation() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60,
        0x02,
        0x7d,
        0x7c,
        0x01,
        0x7f,
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00,
        //=======================
        0x0a, // SECTION: Code
        0x0f, // payload len: 15
        0x01, // entry count: 1
        //          ---         //
        0x0d, // body size: 13
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x41, // i32.const
        0xe9, //
        0x00, //     105
        0x6a, // i32.add
        0x6a, // i32.add
        0x0b, // end function
    ]
}

        // i32.clz       | 0x67  |
        // i32.ctz       | 0x68  |
        // i32.popcnt    | 0x69  |
        // i32.add       | 0x6a  |
        // i32.sub       | 0x6b  |
        // i32.mul       | 0x6c  |
        // i32.div_s     | 0x6d  |
        // i32.div_u     | 0x6e  |
        // i32.rem_s     | 0x6f  |
        // i32.rem_u     | 0x70  |
        // i32.and       | 0x71  |
        // i32.or        | 0x72  |
        // i32.xor       | 0x73  |
        // i32.shl       | 0x74  |
        // i32.shr_s     | 0x75  |
        // i32.shr_u     | 0x76  |
        // i32.rotl      | 0x77  |
        // i32.rotr      | 0x78  |
        // i64.clz       | 0x79  |
        // i64.ctz       | 0x7a  |
        // i64.popcnt    | 0x7b  |
        // i64.add       | 0x7c  |
        // i64.sub       | 0x7d  |
        // i64.mul       | 0x7e  |
        // i64.div_s     | 0x7f  |
        // i64.div_u     | 0x80  |
        // i64.rem_s     | 0x81  |
        // i64.rem_u     | 0x82  |
        // i64.and       | 0x83  |
        // i64.or        | 0x84  |
        // i64.xor       | 0x85  |
        // i64.shl       | 0x86  |
        // i64.shr_s     | 0x87  |
        // i64.shr_u     | 0x88  |
        // i64.rotl      | 0x89  |
        // i64.rotr      | 0x8a  |
        // f32.abs       | 0x8b  |
        // f32.neg       | 0x8c  |
        // f32.ceil      | 0x8d  |
        // f32.floor     | 0x8e  |
        // f32.trunc     | 0x8f  |
        // f32.nearest   | 0x90  |
        // f32.sqrt      | 0x91  |
        // f32.add       | 0x92  |
        // f32.sub       | 0x93  |
        // f32.mul       | 0x94  |
        // f32.div       | 0x95  |
        // f32.min       | 0x96  |
        // f32.max       | 0x97  |
        // f32.copysign  | 0x98  |
        // f64.abs       | 0x99  |
        // f64.neg       | 0x9a  |
        // f64.ceil      | 0x9b  |
        // f64.floor     | 0x9c  |
        // f64.trunc     | 0x9d  |
        // f64.nearest   | 0x9e  |
        // f64.sqrt      | 0x9f  |
        // f64.add       | 0xa0  |
        // f64.sub       | 0xa1  |
        // f64.mul       | 0xa2  |
        // f64.div       | 0xa3  |
        // f64.min       | 0xa4  |
        // f64.max       | 0xa5  |
        // f64.copysign  | 0xa6  |
