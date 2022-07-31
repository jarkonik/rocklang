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
        assert_eq!($result, concat!(indoc!(r#"
            ; ModuleID = 'main'
            source_filename = "main"
            target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
        "#,
        ), "\n", indoc!($valid)))
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
