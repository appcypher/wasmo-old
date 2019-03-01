mod array;
mod enums;
mod float;
mod function;
mod int;
mod pointer;
mod struct_;
mod traits;
mod ty;
mod vector;

pub use array::ArrayType;
pub use enums::BasicType;
pub use float::FloatType;
pub use function::{fn_type, FunctionType};
pub use int::IntType;
pub use pointer::PointerType;
pub use struct_::StructType;
pub(crate) use traits::AsTypeRef;
pub use ty::Type;
pub use vector::VectorType;
