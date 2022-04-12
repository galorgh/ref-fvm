use crate::vm;
use fvm_shared::error::ExitCode;
use std::panic;

// Wrapper around the assert macro to have a hand on which exit code we want to give to our failed
// assertion
#[macro_export]
macro_rules! assert {
    ($cond:expr, $(,)?) => ({
        let res =panic::catch_unwind(|| {
            core::assert!($cond);
        });
        if res.is_err() {
            vm::abort(ExitCode::ErrAssertionFailed as u32, None);
        }
    });
    ($cond:expr, $($arg:tt)+) => ({
        let res =panic::catch_unwind(|| {
            core::assert!($cond, "{}", format_args!($($arg)+));
        });
        if res.is_err() {
            vm::abort(ExitCode::ErrAssertionFailed as u32, Some(format!("{}", format_args!($($arg)+))));
        }
    });
}

// Wrapper around the assert_eq macro to have a hand on which exit code we want to give to our failed
// assertion
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => ({
        let res =panic::catch_unwind(|| {
            core::assert_eq!($left, $right);
        });
        if res.is_err() {
            vm::abort(ExitCode::ErrAssertionFailed as u32, None);
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let res =panic::catch_unwind(|| {
            core::assert_eq!($left, $right, "{}", format_args!($($arg)+));
        });
        if res.is_err() {
            vm::abort(ExitCode::ErrAssertionFailed as u32, Some(format!("{}", format_args!($($arg)+))));
        }
    });
}

// Wrapper around the assert_ne macro to have a hand on which exit code we want to give to our failed
// assertion
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => ({
        let res =panic::catch_unwind(|| {
            core::assert_ne!($left, $right);
        });
        if res.is_err() {
            vm::abort(ExitCode::ErrAssertionFailed as u32, None);
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let res =panic::catch_unwind(|| {
            core::assert_ne!($left, $right, "{}", format_args!($($arg)+));
        });
        if res.is_err() {
            vm::abort(ExitCode::ErrAssertionFailed as u32, Some(format!("{}", format_args!($($arg)+))));
        }
    });
}
