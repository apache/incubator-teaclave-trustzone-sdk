/// Macro for printing to the trace output, without a newline.
///
/// Equivalent to the `trace_println!` macro expect that a newline is not
/// printed at the end of the message.
///
/// # Examples
///
/// ``` no_run
/// trace_print!("this ")
/// trace_print!("will ")
/// trace_print!("be ")
/// trace_print!("on ")
/// trace_print!("the ")
/// trace_print!("same ")
/// trace_print!("line ")
/// print!("this string has a newline, why not choose println! instead?\n");
/// ```
#[macro_export]
macro_rules! trace_print {
    ($($arg:tt)*) => ($crate::trace::Trace::_print(format_args!($($arg)*)));
}

/// Macro for printing to the trace output, with a newline.
/// Use the `format!` syntax to write data to the standard output. See
/// `std::fmt` for more information.
///
/// # Examples
///
/// ``` no_run
/// trace_println!("Hello, World!");
/// trace_println!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! trace_println {
    () => {
        $crate::trace::Trace::_print(format_args!("\n"));
    };
    ($s:expr) => {
        $crate::trace::Trace::_print(format_args!(concat!($s, "\n")));
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::trace::Trace::_print(format_args!(concat!($s, "\n"), $($tt)*));
    };
}
