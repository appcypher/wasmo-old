//! USAGE: cargo run --example jit --features "verbose"

use wasmo_utils::debug;

use wasmo_llvm::{
    types::{fn_type, BasicType},
    values::IntValue, CompilerResult,
    Builder, Context, Module, OptimizationLevel, ExecutionEngine, Func,
};


fn main()  {
    println!("\n=== [ jit_example ] ===\n");

    jit_example().unwrap();

    println!("\n=== [ jit_example ] ===\n");
}

pub fn jit_example() -> CompilerResult<()> {
    let context = Context::create();

    let module = context.create_module("Hello LLVM");

    let builder = context.create_builder();

    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    let sum = unsafe { jit_compile_sum(&context, &module, &builder, &execution_engine)? };

    let (x, y, z) = (1, 2, 3);

    println!("{} + {} + {} = {}", x, y, z, unsafe { sum.call(x, y, z) });

    Ok(())
}

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

unsafe fn jit_compile_sum(context: &Context, module: &Module, builder: &Builder, execution_engine: &ExecutionEngine) -> CompilerResult<Func<SumFunc>> {
    let i64_type: BasicType = context.i64_type().into();

    let func_type = fn_type(&[i64_type, i64_type, i64_type], i64_type, false);

    let function = module.add_function("sum", func_type, None);

    let basic_block = function.append_basic_block("entry", &context);

    builder.position_at_end(&basic_block);

    let x: IntValue = function.get_nth_param(0)?.into();
    let y: IntValue = function.get_nth_param(1)?.into();
    let z: IntValue = function.get_nth_param(2)?.into();

    let sum = builder.build_int_add(x, y, "sum");
    
    let sum = builder.build_int_add(sum, z, "sum");

    builder.build_return(Some(sum));

    Ok(execution_engine.get_function("sum")?)
}
