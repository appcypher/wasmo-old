/// Creates a new enum with variants that wraps certain types.
/// Take BasicValue enum fer example, its varaints wrap certain Value types that are classified as BasicValue.
/// Also provides unwrapping of variant and conversion betweeen the enum and the type.
#[macro_export]
macro_rules! enum_impl_def {
    ($enum:ident (get_type_first: $flag:ident, field: $field:ident, ref: $ref:ident) { $( $llvm_variant0:ident $(| $llvm_variant:ident )* => $variant:ident ),+ }) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $enum {
            $( $variant($variant) ),+
        }

        $(impl From<$variant> for $enum {
            fn from(val: $variant) -> Self {
                $enum::$variant(val)
            }
        })+

        $(impl Into<$variant> for $enum {
            fn into(self) -> $variant {
                match self {
                    $enum::$variant(val) => val,
                    _ => unreachable!("Got {:?}, expected {:?}", self, stringify!($variant))
                }
            }
        })*

        impl $enum {
            pub(crate) fn new($field: $ref) -> $enum {
                let type_kind = unsafe {
                    get_type_kind!($field, $flag)
                };

                match type_kind {
                    $( LLVMTypeKind::$llvm_variant0 $(| LLVMTypeKind::$llvm_variant )* => $enum::$variant($variant::new($field)), )+
                    _ => panic!("Unsupported kind for {:?}: {:?}", stringify!($enum), type_kind)
                }
            }

            pub fn as_ref(self) -> $ref {
                match self {
                    $( $enum::$variant(val) => val.as_ref() ),+
                }
            }
        }
    };
}

/// ...
#[macro_export]
macro_rules! get_type_kind {
    ($field:ident, true) => {
        // Gets the type first. This is useful for LLVMValueRefs where you need to get the type first before getting the kind.
        LLVMGetTypeKind(LLVMTypeOf($field))
    };
    ($field:ident, false) => {
        LLVMGetTypeKind($field)
    };
}

/// For renaming an enum and its variant to another enum.
/// Also adds conversion from the new enum to origninal enum
#[macro_export]
macro_rules! enum_rename {
    ($(#[$enum_attrs:meta])* $old_enum:ident >> $new_enum:ident { $( $(#[$variant_attrs:meta])* $old_variant:ident >> $new_variant:ident ),+ }) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        $(#[$enum_attrs])*
        pub enum $new_enum {
            $(
                $(#[$variant_attrs])*
                $new_variant
            ),+
        }

        impl Into<$old_enum> for $new_enum {
            fn into(self) -> $old_enum {
                match self {
                    $( $new_enum::$new_variant => $old_enum::$old_variant ),+
                }
            }
        }
    };
}

/// For creating overloaded implementations that allows varargs kind of behavior
#[macro_export]
macro_rules! recurse_vararg_impl {
    ($ty:ident) => {
        vararg_impl!($ty);
    };
    ($ty0:ident, $( $ty:ident ),+) => {
        vararg_impl!($ty0, $( $ty ),+);
        recurse_vararg_impl!($( $ty ),+);
    };
}

#[macro_export]
macro_rules! vararg_impl {
    ($( $ty:ident ),*) => {
        impl<$( $ty, )* T> Func<unsafe extern "C" fn ( $( $ty, )* ) -> T> {
            pub unsafe fn call(&self, $( $ty: $ty, )*) -> T {
                (self.address)( $( $ty, )* )
            }
        }
    };
}

///
#[macro_export]
macro_rules! recurse_trait_impl {
    ($ty0:ident, $( $ty:ident ),+) => {
        impl<$ty0, $( $ty, )+ T> ExternFunc for fn ( $ty0, $( $ty, )+ ) -> T {}
        recurse_trait_impl!($( $ty ),+);
    };
    ($ty:ident) => {
        impl<$ty, T> ExternFunc for fn ( $ty ) -> T {}
        recurse_trait_impl!();
    };
    () => {
        impl<T> ExternFunc for fn () -> T {}
    };
}
