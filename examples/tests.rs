use wasmlite_parser::Parser;
use wasmlite_utils::{debug, verbose};

mod test_samples;

fn main() {
    // Parse wasm bytes
    // let wasm_module = Parser::new(&invalid_i32_add_wrong_arg_type()).module();
    // let wasm_module = Parser::new(&invalid_i32_add_wrong_arg_arity()).module();
    // let wasm_module = Parser::new(&invalid_wrong_body_size()).module();
    // let wasm_module = Parser::new(&valid_i32_add_more_args_on_stack()).module();
    // let wasm_module = Parser::new(&valid_i64_add_nested_operation()).module();
    // let wasm_module = Parser::new(&valid_i64_load32_u()).module();
    // let wasm_module = Parser::new(&invalid_i64_load32_u_wrong_return_type()).module();
    // let wasm_module = Parser::new(&valid_i64_add_nested_operation()).module();
    // let wasm_module = Parser::new(&test_samples::valid_local_get()).module();
    // let wasm_module = Parser::new(&test_samples::valid_local_set()).module();
    // let wasm_module = Parser::new(&test_samples::valid_local_tee()).module();
    // let wasm_module = Parser::new(&test_samples::valid_nop()).module();
    // let wasm_module = Parser::new(&test_samples::valid_global_get()).module();
    // let wasm_module = Parser::new(&test_samples::invalid_global_get_non_existent_global()).module();
    // let wasm_module = Parser::new(&test_samples::valid_global_set()).module();
    // let wasm_module = Parser::new(&test_samples::valid_drop()).module();
    // let wasm_module = Parser::new(&test_samples::valid_block()).module();
    let wasm_module = Parser::new(&test_samples::valid_block_additional_stack_values()).module();

    // get_local, set_local, tee_local, tee_global, examples

    match wasm_module {
        Ok(module) => verbose!("wasm_module = {:#?}", module),
        Err(error) => debug!("ERROR!\n\n{:#?}", error),
    }
}
