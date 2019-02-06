_Learn more about wasm binary encoding [here](https://github.com/WebAssembly/design/blob/master/BinaryEncoding.md)_

## WASM BINARY BREAKDOWN

TYPES
==================================

### TYPES [varint7]
type     | hex   | leb
:--------|:------|:----
i32      | 0x7f  | -0x01
i64      | 0x7e  | -0x02
f32      | 0x7d  | -0x03
f64      | 0x7c  | -0x04
anyfunc  | 0x70  | -0x10
func     | 0x60  | -0x20
()       | 0x40  | -0x40

### EXTERNAL_KIND [uint8]
function_import  | 0x00
table_import     | 0x01
memory_import    | 0x02
global_import    | 0x03

### SECTION_ID
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
Code      | 0x10
Data      | 0x11

--------------------------------------

### VALUE_TYPE
- i32, i64, f32, f64

### BLOCK_TYPE
- value_type, ()

### ELEM_TYPE
- any_func

### FUNC_TYPE
fields        | type
:-------------|:---------
form          | varint7
param_count   | varuint32
param_types   | value_type*
return_count  | varuint1 (to know if return type is present)
return_type   | valuetype?

### TABLE_TYPE
fields        | type
:-------------|:------
element_type  | elem_type
limits        | resizable_limit

### MEMORY_TYPE
fields        | type
:-------------|:------
limits        | resizable_limit

### GLOBAL_TYPE
fields        | type
:-------------|:------
content_type  | value_type
mutability    | varuint1

### RESIZABLE_LIMIT
fields        | type
:-------------|:------
flags         | varuint1 (to know if maximum is present)
initial       | varuint32
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
name          | uint1*     (must be valid utf8)
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
Code        | 0x10
Data        | 0x11

### TYPE SECTION
fields      | type
:-----------|:------
count       | varuint32  (signature count)
entries     | func_type*

### IMPORT SECTION
fields      | type
:-----------|:------
count       | varuint32  (signature count)
entries     | import_entry*

### IMPORT_ENTRY
fields      | type
:-----------|:------
module_len  | varuint32
module_str  | bytes
field_len   | varuint32
field_str   | bytes
kind        | external_kind

- IMPORTS
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

--------------------------------------

SKIP

--------------------------------------

### CODE SECTION
fields      | type
:-----------|:------
count       | varuint32
bodies      | function_body*

FUNCTION_BODY
fields      | type
:-----------|:------
body_size   | varuint32 (in bytes)
local_count | varuint32
locals      | local_entry*
code        | byte*
end         | byte

- LOCAL ENTRY
    ### LOCAL_ENTRY
    fields      | type
    :-----------|:------
    count       | varuint32
    type        | value_type


### CONTROL FLOW OPERATORS
- nop

### TRAPS
- Out of bounds memeory access
- Out of bounds table (for functions) access
- Out of bounds table (for blocks) access
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
