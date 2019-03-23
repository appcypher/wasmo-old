
// INVALIDS

pub fn invalid_i32_add_wrong_arg_type() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
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
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
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
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
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

pub fn invalid_i64_load32_u_wrong_type() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x0a, // payload len: 10
        0x01, // entry count: 1
        //          ---         //
        0x08, // body size: 8
        0x00, // local count: 0
        0x42, // i64.const
        0x9a, //
        0x03, //     410
        0x34, // i64.load32_s
        0x02, //     align: 2
        0x08, //     offset: 8
        0x0b, // end function
    ]
}

pub fn invalid_i64_load32_u_wrong_return_type() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x0a, // payload len: 10
        0x01, // entry count: 1
        //          ---         //
        0x08, // body size: 8
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x34, // i64.load32_s
        0x02, //     align: 2
        0x08, //     offset: 8
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
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
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
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
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

pub fn valid_i32_load() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7f, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x0a, // payload len: 10
        0x01, // entry count: 1
        //          ---         //
        0x08, // body size: 8
        0x00, // local count: 0
        0x41, // i32.const
        0x9a, //
        0x03, //     410
        0x28, // i32.load
        0x02, //     align: 2
        0x08, //     offset: 8
        0x0b, // end function
    ]
}

pub fn valid_local_get() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7c, //     I32
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x06, // body size: 6
        0x01, // local count: 1
        0x01, // local type count: 1
        0x7c, //     F64
        0x20, // local.get
        0x00, //     $0
        0x0b, // end function
    ]
}

pub fn valid_local_set() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x06, // payload len: 6
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x00, // return type count: 0
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x0b, // payload len: 11
        0x01, // entry count: 1
        //          ---         //
        0x09, // body size: 9
        0x01, // local count: 1
        0x01, // local type count: 1
        0x7e, //     I64
        0x42, // i64.const
        0x9a, //
        0x03, //     410
        0x21, // local.set
        0x00, //     $0
        0x0b, // end function
    ]
}

pub fn valid_local_tee() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x07, // payload len: 7
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x02, // param type count: 2
        0x7d, //     F32
        0x7c, //     F64
        0x01, // return type count: 1
        0x7e, //     I64
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x0b, // payload len: 11
        0x01, // entry count: 1
        //          ---         //
        0x09, // body size: 9
        0x01, // local count: 1
        0x01, // local type count: 1
        0x7e, //     I64
        0x42, // i64.const
        0x9a, //
        0x03, //     410
        0x22, // local.tee
        0x00, //     $0
        0x0b, // end function
    ]
}
