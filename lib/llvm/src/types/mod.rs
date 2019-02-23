mod array;
mod float;
mod function;
mod int;
mod type_;
mod vector;
mod pointer;
mod struct_;

pub use array::ArrayType;
pub use float::{F32Type, F64Type};
pub use function::FunctionType;
pub use int::{I32Type, I64Type};
pub use vector::VectorType;
pub use pointer::PointerType;
pub use struct_::StructType;
pub use type_::Type;
