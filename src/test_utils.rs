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
    ($expression1:expr, $expression2:expr) => {
        if (!remove_whitespace!($expression1).eq(&remove_whitespace!($expression2))) {
            panic!("\nresult:\n{}\n valid:\n{}", $expression1, $expression2);
        }
    };
}
