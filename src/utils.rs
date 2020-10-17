#[cfg(debug_assertions)]
#[macro_export]
/// In debug mode this simply evaluates to a normal `assert!`.
/// However in release mode this would throw a linking error if not opmized out by
/// the compiler. The only way to optimize this out for the compiler is to proof, that
/// the statement that is being passed always evaluates to true.
macro_rules! compile_time_assert {
    ($stmnt:expr) => {
        assert!($stmnt.evaluate())
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
/// In debug mode this simply evaluates to a normal `assert!`.
/// However in release mode this would throw a linking error if not opmized out by
/// the compiler. The only way to optimize this out for the compiler is to proof, that
/// the statement that is being passed always evaluates to true.
macro_rules! compile_time_assert {
    ($stmnt:expr) => {
        if !$stmnt.evaluate() {
            extern "Rust" {
                #[link_name = "\ncompile_time_assert: Your program has at least one compile time assertion failing"]
                fn undefined() -> !;
            }
            unsafe { undefined() }
        }
    }
}
