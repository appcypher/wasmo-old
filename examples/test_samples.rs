
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

pub fn valid_nop() -> Vec<u8> {
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
        0x0e, // payload len: 14
        0x01, // entry count: 1
        //          ---         //
        0x0c, // body size: 12
        0x01, // local count: 1
        0x01, // local type count: 1
        0x7e, //     I64
        0x01, // nop
        0x42, // i64.const
        0x9a, //
        0x03, //     410
        0x01, // nop
        0x22, // local.tee
        0x00, //     $0
        0x01, // nop
        0x0b, // end function
    ]
}

pub fn valid_global_get() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x00, // param type count: 0
        0x01, // return type count: 1
        0x7e, //     I64
        //=======================
        0x06, // SECTION: Global
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x7e, // content type: I64
        0x01, // mutability: true
        0x42, // i64.const
        0x02, //     2
        0x0b, // end global entry
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x04, // body size: 4
        0x00, // local count: 0
        0x23, // global.get
        0x00, //     $0
        0x0b, // end function
    ]
}

pub fn valid_global_set() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x04, // payload len: 4
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x00, // param type count: 0
        0x00, // return type count: 0
        //=======================
        0x06, // SECTION: Global
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x7e, // content type: I64
        0x01, // mutability: true
        0x42, // i64.const
        0x02, //     2
        0x0b, // end global entry
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x09, // payload len: 9
        0x01, // entry count: 1
        //          ---         //
        0x07, // body size: 7
        0x00, // local count: 0
        0x42, // i64.const
        0x9a, //
        0x03, //     410
        0x24, // global.set
        0x00, //     $0
        0x0b, // end function
    ]
}

pub fn invalid_global_get_non_existent_global() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x00, // param type count: 0
        0x01, // return type count: 1
        0x7e, //     I64
        //=======================
        0x06, // SECTION: Global
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x7e, // content type: I64
        0x01, // mutability: true
        0x42, // i64.const
        0x02, //     2
        0x0b, // end global entry
        //=======================
        0x03, // SECTION: Function
        0x02, // payload len: 2
        0x01, // entry count: 1
        //          ---         //
        0x00, // type index: 0
        //=======================
        0x0a, // SECTION: Code
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x04, // body size: 4
        0x00, // local count: 0
        0x23, // global.get
        0x01, //     $1
        0x0b, // end function
    ]
}

pub fn valid_drop() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x00, // param type count: 0
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
        0x0f, //     15
        0x41, // i32.const
        0x08, //     8
        0x6a, // i32.add
        0x41, // i32.const
        0x0a, //     10
        0x1a, // drop
        0x42, // i64.const
        0x0f, //     15
        0x1a, // drop
        0x0b, // end function
    ]
}

pub fn valid_block() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x00, // param type count: 0
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
        0x14, // payload len: 20
        0x01, // entry count: 1
        //          ---         //
        0x13, // body size: 19
        0x00, // local count: 0
        0x41, // i32.const
        0x0f, //     15
        0x41, // i32.const
        0x08, //     8
        0x6a, // i32.add
        0x1a, // drop
        0x02, // block
        0x7e, //     i64
        0x42, // i64.const
        0x0f, //     15
        0x42, // i64.const
        0x0f, //     15
        0x7d, // i64.sub
        0x0b, // end block
        0x42, // i64.const
        0x15, //     21
        0x7e, // i64.mul
        0x0b, // end function
    ]
}

pub fn valid_block_old_value_new_block_value() -> Vec<u8> {
    vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // PREAMBLE
        //=======================
        0x01, // SECTION: Type
        0x05, // payload len: 5
        0x01, // entry count: 1
        //          ---         //
        0x60, // form: func
        0x00, // param type count: 0
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
        0x12, // payload len: 18
        0x01, // entry count: 1
        //          ---         //
        0x10, // body size: 16
        0x00, // local count: 0
        0x42, // i64.const
        0x0f, //     15
        0x42, // i64.const
        0x08, //     8
        0x7e, // i64.mul
        0x02, // block
        0x7e, //     i64
        0x42, // i64.const
        0x0f, //     15
        0x42, // i64.const
        0x0f, //     15
        0x7d, // i64.sub
        0x0b, // end block
        0x7e, // i64.mul
        0x0b, // end function
    ]
}
