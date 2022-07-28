#[macro_export]
macro_rules! remove_whitespace {
    ($s:expr) => {
        $s.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! assert_eq_ir {
    ($result:expr, $valid:expr) => {
        if (!remove_whitespace!($result).eq(&remove_whitespace!($valid))) {
            panic!("\nresult:\n{}\n valid:\n{}", $result, $valid);
        }
    };
}

#[macro_export]
macro_rules! in_main_function {
    ($compiler:expr, $expression:expr) => {
        let main_fun = $compiler.module.add_function(
            MAIN_FUNCTION,
            $compiler
                .context
                .function_type($compiler.context.void_type(), &[], false),
        );
        let block = $compiler.context.append_basic_block(&main_fun, "");
        $compiler.builder.position_builder_at_end(&block);
        $expression
        $compiler.builder.build_ret_void();
        $compiler.verify_function(main_fun);
    };
}
