use super::Arguments;
use wasmo_codegen::generator::ModuleGenerator;
use wasmo_codegen::options::CodegenOptions;
use wasmo_utils::file::{convert_wat_to_wasm, get_file_bytes, is_wasm_file};

///
pub struct ArgumentsHandler<'a> {
    args: Arguments<'a>,
}

impl<'a> ArgumentsHandler<'a> {
    /// Create new arguments handler.
    pub fn new() -> Self {
        Self {
            args: Arguments::new(),
        }
    }

    ///
    fn run_optional_file(&self) -> Result<(), String> {
        // Get file if supplied.
        if let Some(file_path) = self.args.get_file_path()? {
            // Check if file is a wasm binary.
            let wasm_binary = if is_wasm_file(&file_path)? {
                // Get bytes if it is.
                get_file_bytes(&file_path)?
            } else {
                // Otherwise assume it is wat and convert to wasm binary
                convert_wat_to_wasm(&file_path)?
            };

            // TODO: The following APIS are meant to be accessed via wasmo_runtime::Module and co.
            // TODO: Use default codegen options for now
            let options = &CodegenOptions::default();

            // Generate llvm module and from wasm binary.
            let result = ModuleGenerator::new(&wasm_binary, options).generate_module();

            // Error handing
            let _result = match result {
                Err(error) => panic!("Parsing Error! = {:?}", error),
                Ok(result) => {
                    println!("LLVM Module generated! = {:?}", result.0);
                    println!("Runtime Module Data generated! = {:?}", result.1);
                    result
                }
            };
        }

        Ok(())
    }

    pub fn setup(&self) -> Result<(), String> {
        self.run_optional_file()
    }
}
