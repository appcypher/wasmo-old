This parser is designed to parse an entire wasm module and generate corresponding intermediate representation. It is not the best for parsing a wasm module in bits. For that, you may need  [wasmparser](https://github.com/yurydelendik/wasmparser.rs)

_Resources for learning more about wasm binary:_
- https://webassembly.github.io/spec/
- https://github.com/WebAssembly/design/blob/master/BinaryEncoding.md
- https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md

### THE WASM SPEC
The [binary encoding document](https://github.com/WebAssembly/design/blob/master/BinaryEncoding.md) on wasm design repo is no longer maintained, the spec is supposed to be the source of truth but I was a bit skeptical about it at first because I didn't find the details I needed in time and I was confused by the statement "[all integers are encoded using the LEB128  variable-length integer encoding, in either unsigned or signed variant](https://webassembly.github.io/spec/core/binary/values.html#integers)". The binary doc had mentioned things like `uint8, uint16 and uint32`, but those were not mentioned anywhere in the spec. I later came across the [byte section](https://webassembly.github.io/spec/core/binary/values.html#bytes) and that was the `uint8, uint16 and uint32` I was looking for.

I have an issue with some aspects of wasm space and one of it is that `code section` contains `function_bodies` and thes `function_bodies` start with a varuint32 `body_size` which signifies the entire length of the body. The instructions included. However for global, data and element sections, the entries don't start with a `body_size` representing the length of the enire entry. So to know the end of the instruction section, you have to reach an `end_byte`. Alright, but the instructions should have been placed last in the entry so that one can get other relevant information about an entry.

### TODO
- [ ] Finish instruction parsing
- [x] Complete other sections (table, memory, global, export, start, element, data)
- [ ] Add custom sections (name section and linking section)
- [ ] Parse-time validations
- [ ] Tests
- [ ] Fuzz tests

This parser adheres to the following syntactic rules.

TYPES
==================================

### TYPES [varint7]
type     | hex   | leb
:--------|:------|:----
i32      | 0x7f  | -0x01
i64      | 0x7e  | -0x02
f32      | 0x7d  | -0x03
f64      | 0x7c  | -0x04
funcref  | 0x70  | -0x10
func     | 0x60  | -0x20
()       | 0x40  | -0x40

### EXTERNAL_KIND [uint8]
function_import  | 0x00
table_import     | 0x01
memory_import    | 0x02
global_import    | 0x03

### SECTION_ID (should have been uint8, varuint7 is expensive)
type      | hex
:---------|:------
Type      | 0x1
Import    | 0x2
Function  | 0x3
Table     | 0x4
Memory    | 0x5
Global    | 0x6
Export    | 0x7
Start     | 0x8
Element   | 0x9
Code      | 0xA
Data      | 0xB

--------------------------------------

### VALUE_TYPE
- i32, i64, f32, f64

### BLOCK_TYPE
- value_type, ()

### ELEM_TYPE
- funcref

### FUNC_TYPE
fields        | type
:-------------|:---------
form          | varint7
param_count   | varuint32
param_types   | value_type*
return_count  | varuint1 (to know if return type is present)
return_type   | valuetype? // TODO: multiple return types

### TABLE_TYPE
fields        | type
:-------------|:------
element_type  | elem_type
limits        | limits

### MEMORY_TYPE
fields        | type
:-------------|:------
limits        | limits

### GLOBAL_TYPE
fields        | type
:-------------|:------
content_type  | value_type
mutability    | varuint1

### RESIZABLE_LIMIT
fields        | type
:-------------|:------
flags         | varuint1 (to know if maximum is present)
minimum       | varuint32
maximum       | varuint32?

### INIT_EXPR
...


SECTIONS
==================================

### PREAMBLE (compulsory)
fields        | type
:-------------|:------
magic_number  | uint32
version       | uint32

### SECTION* (a particular section can only appear once)
fields        | type
:-------------|:------
id            | varuint7   (0 for custom section)
payload_len   | varuint32  (size in bytes)
name_len      | varuint32? (only if id == 0)
name          | uint8*     (must be valid utf8)
payload_data  | ...

### SECTION ORDER
type        | hex
:-----------|:------
Type        | 0x1
Import      | 0x2
Function    | 0x3
Table       | 0x4
Memory      | 0x5
Global      | 0x6
Export      | 0x7
Start       | 0x8
Element     | 0x9
Code        | 0xA
Data        | 0xB

### TYPE SECTION
fields      | type
:-----------|:------
count       | varuint32  (signature count)
entries     | func_type*

### IMPORT SECTION
fields      | type
:-----------|:------
count       | varuint32  (entry count)
entries     | import_entry*

### IMPORT_ENTRY
fields      | type
:-----------|:------
module_len  | varuint32
module_name  | byte*
field_len   | varuint32
field_name   | byte*
kind        | external_kind
import      | import_type

- IMPORT_TYPE

    NOTE: While function imports reuse declarations from the type section, other types of import don't.

    ### FUNCTION IMPORT
    fields      | type
    :-----------|:------
    type        | varuint32 (type index of function)

    ### TABLE IMPORT
    fields      | type
    :-----------|:------
    type        | table_type

    ### MEMORY IMPORT
    fields      | type
    :-----------|:------
    type        | memory_type

    ### GLOBAL IMPORT
    fields      | type
    :-----------|:------
    type        | global_type

### FUNCTION SECTION
fields      | type
:-----------|:------
count       | varuint32
types       | varuint32* (type indices of functions)

### TABLE SECTION
fields      | type
:-----------|:------
count       | varuint32
entries     | table_entry*

### MEMORY SECTION
fields      | type
:-----------|:------
count       | varuint32
entries       | memory_entry*

### GLOBAL SECTION
fields      | type
:-----------|:------
count       | varuint32
entries     | global_entry*

- GLOBAL ENTRY

    ### GLOBAL_ENTRY
    fields        | type
    :-------------|:------
    content_type  | value_type
    mutability    | varuint1
    code          | byte*
    end           | byte [0x0b]

### EXPORT SECTION
fields      | type
:-----------|:------
count       | varuint32  (entry count)
entries     | export_entry*

### EXPORT_ENTRY
fields      | type
:-----------|:------
name_len    | varuint32
name        | byte*
kind        | external_kind
index       | varuint32

### START SECTION
fields           | type
:----------------|:------
function_index   | varuint32

### ELEMENT SECTION
fields      | type
:-----------|:------
count       | varuint32  (entry count)
entries     | element_entry*

- ELEMENT ENTRY

    ### ELEMENT_ENTRY
    fields       | type
    :------------|:------
    table_index  | varuint32
    expr         | byte*
    func_count   | varuint32
    func_indices | varuint32*

### CODE SECTION
fields      | type
:-----------|:------
count       | varuint32 (count of function bodies)
bodies      | function_body*

FUNCTION_BODY

fields      | type
:-----------|:------
body_size   | varuint32 (in bytes)
local_count | varuint32
locals      | local_entry*
code        | byte*
end         | byte [0x0b]

- LOCAL ENTRY
    ### LOCAL_ENTRY
    fields      | type
    :-----------|:------
    count       | varuint32
    type        | value_type

- OPERATORS

    op_code - unint8

    - CONTROL FLOW

        operator      | id   | immediates
        :-------------|:-----|:---------
        unreachable	  | 0x00 |
        nop	          | 0x01 |
        block	      | 0x02 | block_type (sig)
        loop	      | 0x03 | block_type (sig)
        if	          | 0x04 | block_type (sig)
        else	      | 0x05 |
        end	          | 0x0b |
        br	          | 0x0c |
        br_if	      | 0x0d |
        br_table	  | 0x0e |
        return	      | 0x0f |

    - CALL

        operator      | id   | immediates
        :-------------|:-----|:---------
        call     	  | 0x00 | varuint32  (function_index)
        call_indirect | 0x01 | varuint32, varuint1 (type_index, reserved)

    - PARAMETRIC

        operator      | id   | immediates
        :-------------|:-----|:---------
        drop     	  | 0x1a |
        select        | 0x1b |

    - VARIABLE ACCESS

        operator      | id   | immediates
        :-------------|:-----|:---------
        get_local	  | 0x20 | varuint32 (local_index)
        set_local	  | 0x21 | varuint32 (local_index)
        tee_local	  | 0x22 | varuint32 (local_index)
        get_global	  | 0x23 | varuint32 (global_index)
        set_global	  | 0x24 | varuint32 (global_index)

    - MEMORY

        operator      | id    | immediates
        :-------------|:------|:---------
        i32.load      | 0x28  | memory_immediate
        i64.load      | 0x29  | memory_immediate
        f32.load      | 0x2a  | memory_immediate
        f64.load      | 0x2b  | memory_immediate
        i32.load8_s   | 0x2c  | memory_immediate
        i32.load8_u   | 0x2d  | memory_immediate
        i32.load16_s  | 0x2e  | memory_immediate
        i32.load16_u  | 0x2f  | memory_immediate
        i64.load8_s   | 0x30  | memory_immediate
        i64.load8_u   | 0x31  | memory_immediate
        i64.load16_s  | 0x32  | memory_immediate
        i64.load16_u  | 0x33  | memory_immediate
        i64.load32_s  | 0x34  | memory_immediate
        i64.load32_u  | 0x35  | memory_immediate
        i32.store     | 0x36  | memory_immediate
        i64.store     | 0x37  | memory_immediate
        f32.store     | 0x38  | memory_immediate
        f64.store     | 0x39  | memory_immediate
        i32.store8    | 0x3a  | memory_immediate
        i32.store16   | 0x3b  | memory_immediate
        i64.store8    | 0x3c  | memory_immediate
        i64.store16   | 0x3d  | memory_immediate
        i64.store32   | 0x3e  | memory_immediate
        memory.size	  | 0x3f  | varuint1 (reserved)
        memory.grow   | 0x40  | varuint1 (reserved)

        - MEMORY IMMEDIATE

            operator      | type
            :-------------|:----------
            flags         | varuint32 (a bitfield which currently contains the alignment in the least significant bits, encoded as log2(alignment))
            offset        | varuint32 (the value of the offset)

    - CONSTANTS

        operator      | id    | immediates
        :-------------|:------|:---------
        i32.const     | 0x41  | varint32 (a constant value interpreted as i32)
        i64.const     | 0x42  | varint64 (a constant value interpreted as i64)
        f32.const     | 0x43  | uint32 (a constant value interpreted as f32)
        f64.const     | 0x44  | uint64 (a constant value interpreted as f64)

    - COMPARISONS

        operator      | id    | immediates
        :-------------|:------|:---------
        i32.eqz       | 0x45  |
        i32.eq        | 0x46  |
        i32.ne        | 0x47  |
        i32.lt_s      | 0x48  |
        i32.lt_u      | 0x49  |
        i32.gt_s      | 0x4a  |
        i32.gt_u      | 0x4b  |
        i32.le_s      | 0x4c  |
        i32.le_u      | 0x4d  |
        i32.ge_s      | 0x4e  |
        i32.ge_u      | 0x4f  |
        i64.eqz       | 0x50  |
        i64.eq        | 0x51  |
        i64.ne        | 0x52  |
        i64.lt_s      | 0x53  |
        i64.lt_u      | 0x54  |
        i64.gt_s      | 0x55  |
        i64.gt_u      | 0x56  |
        i64.le_s      | 0x57  |
        i64.le_u      | 0x58  |
        i64.ge_s      | 0x59  |
        i64.ge_u      | 0x5a  |
        f32.eq        | 0x5b  |
        f32.ne        | 0x5c  |
        f32.lt        | 0x5d  |
        f32.gt        | 0x5e  |
        f32.le        | 0x5f  |
        f32.ge        | 0x60  |
        f64.eq        | 0x61  |
        f64.ne        | 0x62  |
        f64.lt        | 0x63  |
        f64.gt        | 0x64  |
        f64.le        | 0x65  |
        f64.ge        | 0x66  |

    - NUMERIC

        operator      | id    | immediates
        :-------------|:------|:---------
        i32.clz       | 0x67  |
        i32.ctz       | 0x68  |
        i32.popcnt    | 0x69  |
        i32.add       | 0x6a  |
        i32.sub       | 0x6b  |
        i32.mul       | 0x6c  |
        i32.div_s     | 0x6d  |
        i32.div_u     | 0x6e  |
        i32.rem_s     | 0x6f  |
        i32.rem_u     | 0x70  |
        i32.and       | 0x71  |
        i32.or        | 0x72  |
        i32.xor       | 0x73  |
        i32.shl       | 0x74  |
        i32.shr_s     | 0x75  |
        i32.shr_u     | 0x76  |
        i32.rotl      | 0x77  |
        i32.rotr      | 0x78  |
        i64.clz       | 0x79  |
        i64.ctz       | 0x7a  |
        i64.popcnt    | 0x7b  |
        i64.add       | 0x7c  |
        i64.sub       | 0x7d  |
        i64.mul       | 0x7e  |
        i64.div_s     | 0x7f  |
        i64.div_u     | 0x80  |
        i64.rem_s     | 0x81  |
        i64.rem_u     | 0x82  |
        i64.and       | 0x83  |
        i64.or        | 0x84  |
        i64.xor       | 0x85  |
        i64.shl       | 0x86  |
        i64.shr_s     | 0x87  |
        i64.shr_u     | 0x88  |
        i64.rotl      | 0x89  |
        i64.rotr      | 0x8a  |
        f32.abs       | 0x8b  |
        f32.neg       | 0x8c  |
        f32.ceil      | 0x8d  |
        f32.floor     | 0x8e  |
        f32.trunc     | 0x8f  |
        f32.nearest   | 0x90  |
        f32.sqrt      | 0x91  |
        f32.add       | 0x92  |
        f32.sub       | 0x93  |
        f32.mul       | 0x94  |
        f32.div       | 0x95  |
        f32.min       | 0x96  |
        f32.max       | 0x97  |
        f32.copysign  | 0x98  |
        f64.abs       | 0x99  |
        f64.neg       | 0x9a  |
        f64.ceil      | 0x9b  |
        f64.floor     | 0x9c  |
        f64.trunc     | 0x9d  |
        f64.nearest   | 0x9e  |
        f64.sqrt      | 0x9f  |
        f64.add       | 0xa0  |
        f64.sub       | 0xa1  |
        f64.mul       | 0xa2  |
        f64.div       | 0xa3  |
        f64.min       | 0xa4  |
        f64.max       | 0xa5  |
        f64.copysign  | 0xa6  |

    - CONVERSIONS

        operator            | id    | immediates
        :-------------------|:------|:---------
        i32.wrap/i64        | 0xa7  |
        i32.trunc_s/f32     | 0xa8  |
        i32.trunc_u/f32     | 0xa9  |
        i32.trunc_s/f64     | 0xaa  |
        i32.trunc_u/f64     | 0xab  |
        i64.extend_s/i32    | 0xac  |
        i64.extend_u/i32    | 0xad  |
        i64.trunc_s/f32     | 0xae  |
        i64.trunc_u/f32     | 0xaf  |
        i64.trunc_s/f64     | 0xb0  |
        i64.trunc_u/f64     | 0xb1  |
        f32.convert_s/i32   | 0xb2  |
        f32.convert_u/i32   | 0xb3  |
        f32.convert_s/i64   | 0xb4  |
        f32.convert_u/i64   | 0xb5  |
        f32.demote/f64      | 0xb6  |
        f64.convert_s/i32   | 0xb7  |
        f64.convert_u/i32   | 0xb8  |
        f64.convert_s/i64   | 0xb9  |
        f64.convert_u/i64   | 0xba  |
        f64.promote/f32     | 0xbb  |

    - REINTERPRETATIONS

        operator            | id    | immediates
        :-------------------|:------|:---------
        i32.reinterpret/f32 | 0xbc  |
        i64.reinterpret/f64 | 0xbd  |
        f32.reinterpret/i32 | 0xbe  |
        f64.reinterpret/i64 | 0xbf  |


### DATA SECTION
fields      | type
:-----------|:------
count       | varuint32  (entry count)
entries     | data_entry*

- DATA ENTRY

        ### DATA_ENTRY
        fields       | type
        :------------|:------
        memory_index | varuint32
        expr         | byte*
        byte_len     | varuint32
        bytes        | byte*

-----------------------------

### VALIDATION
- PARSE-TIME
    - Out of bounds block index
    - Out of bounds type index
    - Out of bounds function index
    - Out of bounds local index
    - Out of bounds memory index
    - Out of bounds table index
    - Out of bounds global index
    - Block input type and input value match (types and arity)
    - Block result type and result value match (types and arity)
    - Valid hexadecimal float
    - Correctness of operand types in opcode
    - Valid respective LEB values
    - Valid utf-8 strings

- INSTANTIATION-TIME (Sometimes Compile-time)
    - Signature check

### TRAPS
- Out of bounds memory access
- Out of bounds table (for functions) access
- Execution reaches unreachable
- Stack overflow
- Division by zero

# ENCODING
## LEB128
#### UNSIGNED
```
MSB ------------------ LSB
00001001 10000111 01100101  In raw binary
 0100110  0001110  1100101  Padded to a multiple of 7 bits
00100110 10001110 11100101  Add high 1 bits on all but last (most significant) group to form bytes

Starting with the first byte, get rid of the msb bit
11100101 -> 11001010 << 1
11001010 -> 01100101 >> 1

Widen to appropriate type
01100101 -> 00000000 01100101 as u32

For the next byte, get rid of the msb bit
10001110 -> 00011100 << 1
00011100 -> 00001110 >> 1

Widen to appropriate type
00001110 -> 00000000 00001110 as u32

Shift by 7 bits to the left
00000000 00001110 -> 00000111 00000000 << 7

Or the value with the previous result
00000111 00000000 | 00000000 01100101

And so on. Basically you shift by a multiple of 7

if byte's msb is unset, you can break the loop
```
#### SIGNED
```
MSB ------------------ LSB
00000110 01111000 10011011  In raw two's complement binary
 1011001  1110001  0011011  Sign extended to a multiple of 7 bits
01011001 11110001 10011011  Add high 1 bits on all but last (most significant) group to form bytes

Starting with the first byte, get rid of the msb bit
10011011 -> 00110110 << 1
00110110 -> 00011011 >> 1

Widen to appropriate type
01100101 -> 00000000 01100101 as u32

For the next byte, get rid of the msb bit
10001110 -> 00011100 << 1
00011100 -> 00001110 >> 1

Widen to appropriate type
00001110 -> 00000000 00001110 as u32

Shift by 7 bits to the left
00000000 00001110 -> 00000111 00000000 << 7

Or the value with the previous result
00000111 00000000 | 00000000 01100101

And so on. Basically you shift by a multiple of 7

Decoding a signed LEB128 encoding has an extra twist, we need to extend the sign bit

If the value is signed, then the msb is set

if result & 0x0100_0000 == 0x0100_0000 {
    result |= !(0x1 << encoding_size)
}

if byte's msb is unset, you can break the loop
```

### WELL-FORMED UTF-8 BYTE SEQUENCES
Based on Unicode Standard 11.0, Section 3.9, Table 3-7.

| Code Points        | First Byte   | Second Byte    | Third Byte    | Fourth Byte   |
|:-------------------|:-------------|:---------------|:--------------|:--------------|
| U+0000..U+007F     | 00..7F       |                |               |               |
| U+0080..U+07FF     | C2..DF       | 80..BF         |               |               |
| U+0800..U+0FFF     | E0           | A0..BF         | 80..BF        |               |
| U+1000..U+CFFF     | E1..EC       | 80..BF         | 80..BF        |               |
| U+D000..U+D7FF     | ED           | 80..9F         | 80..BF        |               |
| U+E000..U+FFFF     | EE..EF       | 80..BF         | 80..BF        |               |
| U+10000..U+3FFFF   | F0           | 90..BF         | 80..BF        | 80..BF        |
| U+40000..U+FFFFF   | F1..F3       | 80..BF         | 80..BF        | 80..BF        |
| U+100000..U+10FFFF | F4           | 80..8F         | 80..BF        | 80..BF        |

---------------------------------------------------
