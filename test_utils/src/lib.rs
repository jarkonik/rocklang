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
        assert_eq!(
            $result,
            concat!(
                indoc!(
                    r#"
            ; ModuleID = 'main'
            source_filename = "main"
        "#,
                ),
                "\n",
                indoc!($valid)
            )
        )
    };
}

#[macro_export]
macro_rules! in_main_function {
    ($context:expr, $module:expr, $builder:expr, $expression:expr) => {
        let main_fun = $module.add_function(
            MAIN_FUNCTION,
            $context.function_type($context.void_type(), &[], false),
        );
        let block = $context.append_basic_block(&main_fun, "");
        $builder.position_builder_at_end(&block);
        $expression
        $builder.build_ret_void();

        main_fun.verify_function().unwrap_or_else(|_x| {
            println!("IR Dump:");
            println!("{}", $module);
            panic!("Function verification failed")
        });
    };
}
