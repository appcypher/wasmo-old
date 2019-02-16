use llvm_sys::{
    core,
};

///
struct Module {
    module: core::LLVMModule,
    builder: core::LLVMBuilder,
    ctx: core::LLVMContext.
}

impl LLVMModule {
    ///
    fn new() -> Self {
        Self {
            module: unimplemented!(),
            builder: unimplemented!(),
            ctx: unimplemented!(),
        }
    }

    ///
    fn get_refs() -> () {
        ()
    }


    ///
    fn get_wasm_ir(code: &[u8]) -> () {
        let parser = Parser::new(code);
        let wasm_ir = parser.module();
        println!("wasm_ir = {:#?}", wasm_ir);
        ()
    }

    ///
    fn create_xxxx() -> () {
        ()
    }
}

impl Drop for LLVMModule {
    /// Dispose builder, context and module.
    fn drop(&mut self) {
        core::LLVMDisposeBuilder(self.builder);
        core::LLVMDisposeModule(self.module);
        core::LLVMDisposeContext(self.ctx);
    }
}
