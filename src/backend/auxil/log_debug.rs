//! These macros are wrappers around the macros from the `log` crate.
//! Log messages below `INFO` level will only be output if compiled with `debug_assertions` turned on.
//! This is used to avoid overhead from frequent calls to the logger when compiled in release mode.

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log_debug {
    ($lvl:expr, $($arg:tt)+) => {
        log!($lvl, $($arg)+)
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        log!(target: $target, $lvl, $($arg)+)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log_debug {
    ($lvl:expr, $($arg:tt)+) => {
        if $lvl >= ::log::Level::Info {
            log!($lvl, $($arg)+)
        }
    };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {
        if $lvl >= ::log::Level::Info {
            log!($lvl, $($arg)+)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        log_debug!(::log::Level::Debug, $($arg)+)
    };
    (target: $target:expr, $($arg:tt)+) => {
        log_debug!(target: $target, $($arg)+)
    };
}

#[macro_export]
macro_rules! trace {
    ($(arg:tt)*) => {
        log_debug!(::log::Level::Trace, $($arg)*)
    };
    (target: $target:expr, $($arg:tt)+) => {
        log_debug!(target: $target, $($arg)+)
    };
}
