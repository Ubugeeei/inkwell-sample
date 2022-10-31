use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::error::Error;

type SubFunc = unsafe extern "C" fn(u64, u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sub(&self) -> Option<JitFunction<SubFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sub", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();

        let sub = self.builder.build_int_sub(x, y, "sub");

        self.builder.build_return(Some(&sub));

        unsafe { self.execution_engine.get_function("sub").ok() }
    }
}

pub fn exec(x: u64, y: u64) -> Result<u64, Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("sub");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let sub = codegen
        .jit_compile_sub()
        .ok_or("Unable to JIT compile `sub`")?;

    unsafe { Ok(sub.call(x, y)) }
}
