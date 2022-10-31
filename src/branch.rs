use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::error::Error;

type IsEven = unsafe extern "C" fn(u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_is_even(&self) -> Option<JitFunction<IsEven>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into()], false);
        let function = self.module.add_function("is_even", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let arg = function.get_nth_param(0)?.into_int_value();
        let cond = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            self.builder.build_int_unsigned_rem(
                arg,
                self.context.i64_type().const_int(2, false),
                "rem",
            ),
            self.context.i64_type().const_zero(),
            "cond",
        );

        let if_then = self.context.append_basic_block(function, "if.then");
        let if_else = self.context.append_basic_block(function, "if.else");
        let if_end = self.context.append_basic_block(function, "if.end");

        self.builder
            .build_conditional_branch(cond, if_then, if_else);

        self.builder.position_at_end(if_then);
        let then_val = self.context.i64_type().const_int(1, false);
        self.builder.build_unconditional_branch(if_end);
        let if_then = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(if_else);
        let else_val = self.context.i64_type().const_zero();
        self.builder.build_unconditional_branch(if_end);
        let if_else = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(if_end);

        let phi = self.builder.build_phi(i64_type, "phi");
        phi.add_incoming(&[(&then_val, if_then), (&else_val, if_else)]);
        self.builder
            .build_return(Some(&phi.as_basic_value().into_int_value()));

        unsafe { self.execution_engine.get_function("is_even").ok() }
    }
}

pub fn exec(x: u64) -> Result<u64, Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("add");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let add = codegen
        .jit_compile_is_even()
        .ok_or("Unable to JIT compile `add`")?;

    let _ = std::fs::write("is_even.ll", codegen.module.to_string());

    unsafe { Ok(add.call(x)) }
}
