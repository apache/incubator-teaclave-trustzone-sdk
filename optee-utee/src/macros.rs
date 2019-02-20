#[macro_export]
macro_rules! trace_print {
    ($($arg:tt)*) => ($crate::Trace::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! trace_println {
    () => {
        $crate::Trace::_print(format_args!("\n"));
    };
    ($s:expr) => {
        $crate::Trace::_print(format_args!(concat!($s, "\n")));
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::Trace::_print(format_args!(concat!($s, "\n"), $($tt)*));
    };
}
