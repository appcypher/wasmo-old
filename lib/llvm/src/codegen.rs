use llvm_sys::{
    LLVMContext,
    LLVMModule,
    LLVMBuilder,
    core::{
        LLVMContextCreate,
        LLVMModuleCreateWithNameInContext,
        LLVMCreateBuilderInContext,
        LLVMContextDispose,
        LLVMDisposeModule,
        LLVMDisposeBuilder
    },
};

use wasmlite_parser::parser::Parser;

///
pub struct LLVMCodegen {
    context: *mut LLVMContext,
    module: *mut LLVMModule,
    builder: *mut LLVMBuilder,
}

impl LLVMCodegen {
    ///
    pub fn new(module_name: &str) -> Self {
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(module_name.as_ptr() as *const _, context);
            let builder = LLVMCreateBuilderInContext(context);
            Self { context, module, builder }
        }
    }

    ///
    pub fn get_refs() -> () {
        ()
    }

    ///
    pub fn get_wasm_ir(code: &[u8]) -> () {
        let mut parser = Parser::new(code);
        let wasm_ir = parser.module();
        println!("wasm_ir = {:#?}", wasm_ir);
        ()
    }

    ///
    pub fn target_triple() -> () {
        ()
    }

    ///
    pub fn generate_instructions() -> () {
        ()
    }

    ///
    pub fn dispose_context() -> () {
        ()
    }

    ///
    pub fn dispose_module() -> () {
        ()
    }

    ///
    pub fn dispose_builder() -> () {
        ()
    }
}

// TODO: My drop implementation segfaults!
impl Drop for LLVMCodegen {
    /// Dispose builder, context and module.
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
            LLVMDisposeModule(self.module);
            LLVMDisposeBuilder(self.builder);
        }
    }
}
